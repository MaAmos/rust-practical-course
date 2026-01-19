
use std::collections::{HashMap, HashSet, VecDeque};
use crate::model::{SessionEventInfo, DataPointEventType};
use log::{info, warn};
use serde::Serialize;

// Double-ended queue to store events in time order
// Consumes SessionEventInfo
#[derive(Debug, Clone, Default, Serialize)]
pub struct AggregatedMetrics {
    // 活跃用户数
    pub active_users: usize,
    // 会话数
    pub session_count: usize,
    // 平均会话时长 (ms)
    pub avg_session_duration: f64,
    // 平均会话深度 (Pageviews per session)
    pub avg_session_depth: f64,
    // 设备类型分布
    pub device_distribution: HashMap<String, usize>,
    // 浏览器分布
    pub browser_distribution: HashMap<String, usize>,
    // 版本号分布
    pub version_distribution: HashMap<String, usize>,
    // 常见入口页面
    pub common_entry_pages: HashMap<String, usize>,
    // 常见出口页面
    pub common_exit_pages: HashMap<String, usize>,
    // 热门路径
    pub popular_paths: HashMap<String, usize>,
    // 事件总数
    pub total_events: usize,
    // 平均事件数 (per session)
    pub avg_events_per_session: f64,
}

// 5 minutes in milliseconds
const WINDOW_SIZE_MS: i64 = 5 * 60 * 1000;

pub struct RealTimeAggregator {
    // Events sorted by event_time (asc)
    events: VecDeque<SessionEventInfo>,
    // Aggregation window size
    window_duration: i64,
}

impl RealTimeAggregator {
    pub fn new() -> Self {
        RealTimeAggregator {
            events: VecDeque::new(),
            window_duration: WINDOW_SIZE_MS,
        }
    }

    /// Add a new event to the aggregator.
    /// This assumes that events are generally added in chronological order.
    pub fn add_event(&mut self, event: SessionEventInfo) {
        self.events.push_back(event);
    }

    /// Prune events that are older than the window based on the latest event time or system time.
    /// Using the latest event time in the queue is safer for replayed data or slight delays.
    /// If no events, nothing happens.
    pub fn prune_events(&mut self) {
        if self.events.is_empty() {
            return;
        }

        // Determine the reference time ("now") for the window.
        // Option 1: System time (Real-time dashboard)
        // Option 2: Max event time (Data-driven window) -> Chosen for consistency with data stream
        let max_time = self.events.back().map(|e| e.event_time).unwrap_or(0);

        // Safety check for unsorted arrival (though rare in Kafka partition processing usually)
        // Ideally we use SystemTime::now() for "Last 5 mins of WALL CLOCK"
        // OR max_time for "Last 5 mins of DATA TIME".
        // Let's use max_time to support backfill/processing delays correctly.
        let cutoff_time = max_time - self.window_duration;

        while let Some(front_event) = self.events.front() {
            if front_event.event_time < cutoff_time {
                self.events.pop_front();
            } else {
                break;
            }
        }
    }

    /// Calculate metrics based on the current window of events.
    pub fn calculate_metrics(&mut self) -> AggregatedMetrics {
        // Ensure the window is clean before calculating
        self.prune_events();

        let mut metrics = AggregatedMetrics::default();
        metrics.total_events = self.events.len();

        if self.events.is_empty() {
            return metrics;
        }

        let mut user_set = HashSet::new();
        let mut session_map: HashMap<String, Vec<&SessionEventInfo>> = HashMap::new();

        // Pass 1: Group by session and Basic Counts
        for event in &self.events {
            user_set.insert(&event.person_code);
            session_map.entry(event.session_id.clone())
                .or_default()
                .push(event);

            // Device Distribution
            *metrics.device_distribution.entry(event.event_platform.clone()).or_insert(0) += 1;

            // Version Distribution (Assuming property maps to useful info, using chrome_version/platform as proxies if specific version fields absent)
            // Using event_chrome_version if present
            if !event.event_chrome_version.is_empty() && event.event_chrome_version != "-" {
                 *metrics.version_distribution.entry(event.event_chrome_version.clone()).or_insert(0) += 1;
            }

             // Browser Distribution heuristic
            let browser = if !event.event_chrome_version.is_empty() && event.event_chrome_version != "-" {
                "Chrome".to_string()
            } else if !event.event_safari_version.is_empty() && event.event_safari_version != "-" {
                "Safari".to_string()
            } else if event.event_platform.to_lowercase().contains("android") {
                "Android Webview".to_string()
            } else if event.event_platform.to_lowercase().contains("ios") {
                "Safari Mobile".to_string()
            } else {
                "Other".to_string()
            };
            *metrics.browser_distribution.entry(browser).or_insert(0) += 1;
        }

        metrics.active_users = user_set.len();
        metrics.session_count = session_map.len();

        let mut total_duration = 0;
        let mut total_pv_depth = 0;

        // Pass 2: Session Analysis
        for (_session_id, events) in &session_map {
            // events are likely sorted by time because self.events is sorted, but let's be safe
            // We need to re-sort detailed slice just to be safe if HashMap grouping lost relative order?
            // No, grouping doesn't sort. The events Vec reference order is based on insertion.
            // But let's just iterate and find min/max time effectively.

            let min_time = events.iter().map(|e| e.event_time).min().unwrap_or(0);
            let max_time = events.iter().map(|e| e.event_time).max().unwrap_or(0);

            // Session Duration in this window
            let duration = max_time - min_time;
            total_duration += duration;

            // Find first and last event by time for Entry/Exit
            // (Simpler than sorting if just needing first/last)
            let mut sorted_events = events.clone();
            sorted_events.sort_by_key(|e| e.event_time);

            if let Some(first) = sorted_events.first() {
                 if let Some(p) = Self::get_page_path(first) {
                     *metrics.common_entry_pages.entry(p).or_insert(0) += 1;
                 }
            }
            if let Some(last) = sorted_events.last() {
                if let Some(p) = Self::get_page_path(last) {
                    *metrics.common_exit_pages.entry(p).or_insert(0) += 1;
                }
            }

            // Path Analysis & Depth
            let mut path_sequence = Vec::new();
            let mut pv_count = 0;

            for evt in sorted_events {
                 if let DataPointEventType::Pageview = evt.event_type {
                     pv_count += 1;
                     if let Some(p) = Self::get_page_path(evt) {
                         path_sequence.push(p);
                     }
                 }
            }
            total_pv_depth += pv_count;

            if !path_sequence.is_empty() {
                let path_str = path_sequence.join(" -> ");
                *metrics.popular_paths.entry(path_str).or_insert(0) += 1;
            }
        }

        if metrics.session_count > 0 {
            metrics.avg_session_duration = total_duration as f64 / metrics.session_count as f64;
            metrics.avg_session_depth = total_pv_depth as f64 / metrics.session_count as f64;
            metrics.avg_events_per_session = metrics.total_events as f64 / metrics.session_count as f64;
        }

        metrics
    }

    fn get_page_path(event: &SessionEventInfo) -> Option<String> {
        if event.event_url_path.is_empty() || event.event_url_path == "-" {
            None
        } else {
            Some(event.event_url_path.clone())
        }
    }
}