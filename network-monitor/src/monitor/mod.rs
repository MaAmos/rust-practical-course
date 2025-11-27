

pub mod types;
use crate::tools_types::MonitorType;

pub mod monitor_trait;
use monitor_trait::Monitor;

mod icmp_monitor;
mod tcp_monitor;
mod udp_monitor;
mod dns_monitor;
mod http_monitor;
mod ftp_monitor;
mod traceroute_monitor;
mod cpu_monitor;
mod memory_monitor;
mod disk_monitor;
mod process_monitor;
mod unknown_monitor;
// 不同类型的监控系统
use icmp_monitor::IcmpMonitor;
use tcp_monitor::TcpMonitor;
use udp_monitor::UdpMonitor;
use dns_monitor::DnsMonitor;
use http_monitor::HttpMonitor;
use ftp_monitor::FtpMonitor;
use traceroute_monitor::TracerouteMonitor;
use cpu_monitor::CpuMonitor;
use memory_monitor::MemoryMonitor;
use disk_monitor::DiskMonitor;
use process_monitor::ProcessMonitor;
use unknown_monitor::UnknowMonitor;
// 定义监控工厂
pub struct MonitorFactory;

impl MonitorFactory {
    pub fn create_monitor(monitor_type: MonitorType) -> Box<dyn Monitor> {
        match monitor_type {
            MonitorType::Icmp => Box::new(IcmpMonitor::new()),
            MonitorType::Tcp => Box::new(TcpMonitor::new()),
            MonitorType::Udp => Box::new(UdpMonitor::new()),
            MonitorType::Dns => Box::new(DnsMonitor::new()),
            MonitorType::Http => Box::new(HttpMonitor::new()),
            MonitorType::Ftp => Box::new(FtpMonitor::new()),
            MonitorType::Traceroute => Box::new(TracerouteMonitor::new()),
            MonitorType::Cpu => Box::new(CpuMonitor::new()),
            MonitorType::Memory => Box::new(MemoryMonitor::new()),
            MonitorType::Disk => Box::new(DiskMonitor::new()),
            MonitorType::Process => Box::new(ProcessMonitor::new()),
            _ => Box::new(UnknowMonitor::new())
        }
    }

}


