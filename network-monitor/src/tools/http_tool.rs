// 定义一些 HTTP监控的工具函数和结构体
// 使用trust-dns-resolver 进行DNS解析和连接耗时
use trust_dns_resolver::{TokioAsyncResolver};
use std::time::Instant;
use tokio::{net::TcpStream, time::{Duration, timeout}};
use tokio_native_tls::TlsConnector as TokioTlsConnector;
use native_tls::TlsConnector;
use crate::tools_types::CertificateInfo;
type DnsTcpTlsPerformance = (u128, u128, u128, Option<CertificateInfo>); // DNS时间，TCP时间，TLS时间，证书信息
use x509_parser::prelude::*;
use std::io;
// 计算 性能监控相关数据 DNS TCP TLS 三个数据耗时
pub async fn get_dns_tcp_tls_performance(url: &str) -> Result<DnsTcpTlsPerformance, Box<dyn std::error::Error>> {
    // 设置一个默认初始时间
    let url = reqwest::Url::parse(url)?;
    let host = url.host_str().ok_or("Invalid URL")?;
    let port = url.port_or_known_default().unwrap_or(80);
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;
    let dns_lookup_time =Instant::now();
    let ips = resolver.lookup_ip(host).await?;
    // 开始设置DNS的缓存时间
    let dns_lookup_ms = dns_lookup_time.elapsed().as_millis();
    // 开始设置TCP的缓存时间 以及 TLS的时间
    // 遍历相关的IP 地址 取其中最小的一个时间即可
    let mut tls_min_time: Option<u128> = None;
    let mut tcp_min_time: Option<u128> = None;

    let per_addr_timeout = Duration::from_secs(3);
    let is_https = url.scheme() == "https";
    // 复用 TLS 连接器
    let tls_connector = if is_https {
        Some(TokioTlsConnector::from(TlsConnector::new()?))
    } else {
        None
    };
    let mut ssl_certificate_info: Option<CertificateInfo> = None;
    for ip in ips.iter(){
        let addr = std::net::SocketAddr::new(ip, port);
        let start_tcp = Instant::now();
        let tcp_res = timeout(per_addr_timeout, TcpStream::connect(addr)).await;
        let tcp_stream = match tcp_res {
            Ok(Ok(stream)) => {
                let ms = start_tcp.elapsed().as_millis();
                tcp_min_time = tcp_min_time.map_or(Some(ms), |min| Some(min.min(ms)));
                stream
            },
            _ => continue, // 超时或连接失败，尝试下一个地址
        };
        if let Some(tls) = &tls_connector {
            let hs_start = Instant::now();
            let hs_res = timeout(per_addr_timeout, tls.connect(host, tcp_stream)).await;
            if let Ok(Ok(_tls_stream)) = hs_res {
                let ms = hs_start.elapsed().as_millis();
                tls_min_time = tls_min_time.map_or(Some(ms), |min| Some(min.min(ms)));
                // 检测证书有效性可以在这里进行
                if ssl_certificate_info.is_none() {
                    if let Some(cert) = _tls_stream.get_ref().peer_certificate()? {
                        let cert_der = cert.to_der()?;
                        // from_der 返回 nom::IResult，需要手动 map_err
                        let (_, x509_cert) = X509Certificate::from_der(&cert_der)
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("x509 parse error: {e}")))?;

                        let validity = x509_cert.validity();
                        let now =  std::time::SystemTime::now();
                        let not_before = validity.not_before.to_datetime();
                        let not_after = validity.not_after.to_datetime();
                        let days_until_expiry = (not_after - now).whole_days();

                        let info = CertificateInfo {
                            issuer: Some(x509_cert.issuer().to_string()),
                            subject: Some(x509_cert.subject().to_string()),
                            valid_from: Some(not_before.to_string()),
                            valid_until: Some(not_after.to_string()),
                            serial_number: Some(x509_cert.tbs_certificate.serial.to_string()),
                            signature_algorithm: Some(x509_cert.signature_algorithm.algorithm.to_string()),
                            public_key_algorithm: Some(format!("{:?}", x509_cert.public_key().algorithm)),
                            public_key_size: Some(x509_cert.public_key().subject_public_key.data.len() * 8), // 转换为位数
                            is_valid: days_until_expiry > 0,
                        };
                        ssl_certificate_info = Some(info);
                    }
                }
            }
        }
    }
    let tcp = tcp_min_time.ok_or("all TCP connect attempts failed")?;
    let tls = if is_https { tls_min_time.unwrap_or(0) } else { 0 };
    Ok((dns_lookup_ms, tcp, tls, ssl_certificate_info))
}