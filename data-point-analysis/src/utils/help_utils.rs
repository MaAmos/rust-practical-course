
// 时间相关的工具

use chrono::{DateTime, Timelike, Utc};
use ua_parser::{device};

// 计算两个时间的差 格式为时间戳 换算成天
pub fn time_diff_to_day(start_ms: i64 , end_ms: i64) -> i64 {
    if end_ms <= start_ms {
        return 1;
    }
    let diff = end_ms - start_ms;
    return (diff / 86400000) as i64;
}

// 初始化 按小时的直方图基础数据集
pub fn init_hour_histogram() -> Vec<(i64,i64)> {
    let mut histogram = vec![(0,0); 24];
    for i in 0..24 {
        histogram[i] = (i as i64, 0);
    }
    return histogram;
}

// 将毫秒时间戳转成字符串与小时数
pub fn format_ts_and_hour(timestamp_ms: i64) -> Option<(String, u32)> {
    chrono::DateTime::<Utc>::from_timestamp_millis(timestamp_ms).map(|dt| {
        let formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();
        let hour = dt.hour();
        (formatted, hour)
    })
}

// 根据时间戳更新小时直方图
pub fn update_hour_histogram(histogram: &mut Vec<(i64, i64)>, timestamp_ms: i64) -> &mut Vec<(i64, i64)> {
    if let Some(dt) = DateTime::<Utc>::from_timestamp_millis(timestamp_ms) {
        let hour = dt.hour() as i64;
        let idx = (hour % 24) as usize;
        histogram[idx] = (hour, histogram[idx].1 + 1);
    }
    return histogram;
}

// 获取直方图中 最大值对应的小时
pub fn get_peak_hour(histogram: &Vec<(i64, i64)>) -> u8 {
    let mut peak_hour = 0u8;
    let mut max_count = 0i64;
    for &(hour, count) in histogram.iter() {
        if count > max_count {
            max_count = count;
            peak_hour = hour as u8;
        }
    }
    peak_hour
}

// 判断是否为夜猫子用户（晚上活跃） 超过1/3的活跃时间段为夜猫子
pub fn is_night_cat(histogram: &Vec<(i64, i64)>) -> bool {
    let night_start = 22;
    let mut night_count = 0;
    let mut total_count = 0;
    for &(hour, count) in histogram.iter() {
        if hour >= night_start {
            night_count += count;
        }
    }
    total_count = histogram.iter().map(|&(_, count)| count).sum::<i64>();

    return night_count * 3 > total_count;
}


// 设置用户的路径 hist
pub fn set_user_hist_config(hist: &mut Vec<(String, i64)>, value: String) -> Vec<(String, i64)> {
    let mut found = false;
    for (hist_value, count) in hist.iter_mut() {
        if *hist_value == value {
            *count += 1;
            found = true;
            break;
        }
    }
    // 如果没有找到 则添加
    if !found {
        hist.push((value, 1));
    }
    hist.to_vec()
}

// 获取用户路径 hist  获取前三的路径
pub fn get_user_path_by_three(mut hist: Vec<(String, i64)>) -> Vec<String> {
    hist.sort_by(|a, b| b.1.cmp(&a.1));
    hist.iter().take(3).map(|(path, _)| path.clone()).collect::<Vec<_>>()
}

// 根据UA 获取设备型号
pub fn extract_device_model(ua: &str) -> Option<String> {
    let aa = device::Builder::new().build();
    let device = match  aa {
        Ok(extract) => extract,
        Err(_) => return None,
    };
    return Some(device.extract(ua).map_or("unknown".to_string(), |d| d.into_owned().device));
}

// 设置用户的设备类型
pub fn set_user_device_hist(hist: &mut Vec<(String, i64)>, ua: String) -> Vec<(String, i64)> {
    let device_model = extract_device_model(&ua).map_or("unknown".to_string(), |m| m);
    set_user_hist_config(hist, device_model)
}

// 设置用户的平台类型
pub fn set_user_platform_hist(hist: &mut Vec<(String, i64)>,  platform_info: (String, String,String)) -> Vec<(String, i64)> {
    let (os_name, os_chrome_ver, os_safari_ver) = platform_info;
    let hist_key = match os_name.as_str() {
        "PC" => format!("PC_{}", os_chrome_ver),
        "IOS" => format!("IOS_{}", os_safari_ver),
        "Android" => format!("Android_{}", os_safari_ver),
        _ => format!("Other_{}_{}", os_chrome_ver, os_safari_ver),
    };
    set_user_hist_config(hist, hist_key)
}