use crate::monitor::types::{MonitorConfig, CheckResult, MonitorConfigDetail, CheckResultDetail};
use crate::tools_types::{AlertRuleTypes, MonitorType, NotifyCondition, NotifyConfig, AlertVerificationRules};
use regex::Regex;
use reqwest::Client;

pub struct NotifyEngine{
    // 这里可以添加一些配置参数，比如通知方式、接收人等
    notify_type: String, // 通知类型 email sms webhook 等
    notify_config: NotifyConfig, // 通知配置内容 比如邮箱地址 电话号码等
}

impl NotifyEngine {
    pub fn new(notify_type: String, notify_config: NotifyConfig) -> Self {
        NotifyEngine {
            notify_type,
            notify_config,
        }
    }

    // 这里可以添加一些方法，比如发送通知等
    pub async fn send_alert_message(&self, message: String) {
        // 这里可以根据具体的通知方式进行发送
        match self.notify_type.as_str() {
            "EMAIL" => {
                // 发送邮件通知
                println!("Sending email alert...");
            },
            "SMS" => {
                // 发送短信通知
                println!("Sending SMS alert...");
            },
            "FEISHU" => {
                // 发送飞书通知
                println!("Sending Feishu alert... {}", self.notify_config.webhook_url);

                // 发动post 请求 到 webhook_url
                let response = Client::new()
                    .post(self.notify_config.webhook_url.clone())
                    .json(&serde_json::json!({
                        "msg_type": "text",
                        "content": {
                            "text": message
                        }
                    }))
                    .send();
                match response.await {
                    Ok(resp) => {
                        println!("Feishu alert sent successfully: {:?}", resp);
                        match resp.text().await {
                            Ok(_) => println!("Response text: FEISHU" ),
                            Err(e) => println!("Failed to get response text: {:?}", e),
                        }
                    },
                    Err(e) => {
                        println!("Failed to send Feishu alert: {:?}", e);
                    }
                }
            },
            _ => {
                println!("Unknown notify type");
            }
        }
    }
}

pub struct AlertsEngine {
    // 这里可以添加一些配置参数，比如告警规则、通知方式等
    notify: NotifyEngine,
    alert_rules: AlertVerificationRules,
}

impl AlertsEngine {
    pub fn new(alert_rule: AlertVerificationRules) -> Self {
        AlertsEngine {
            alert_rules: alert_rule.clone(),
            notify: NotifyEngine::new(alert_rule.notify_type, alert_rule.notify_config),
        }
    }

    // 这里可以添加一些方法，比如添加告警规则、触发告警等
    // 定义一个 告警规则check事件 当监控结果产生时，调用该方法，检查是否需要告警通知
    pub async fn check(&self,  check_result: CheckResult) -> Result<(), String> {
        // 只对成功的监控结果进行告警检查
        if !check_result.status{
            return Ok(());
        }
        // 这里可以根据具体的告警规则进行检查
        match check_result.monitor_type {
            MonitorType::Http => {
                // 调用HTTP监控的检查方法
                Ok(self.check_http(check_result).await)
            },
            _ => {
                // 其他监控类型的检查方法

                Ok(())
            }
        }

    }

    pub async fn check_http(&self, check_result: CheckResult)  {
        // 获取当前config 下面的具体的告警规则
        let alerts_rules= self.alert_rules.rules.clone();
        if alerts_rules.is_empty() {
            return;
        }
        // 遍历规则列表 根据不同的类型 进行不同的提醒
        for single_rule in alerts_rules.iter() {
            match single_rule.rule_type {
                AlertRuleTypes::ResponseCode => {
                    // 检查响应码是否符合告警条件
                    if let CheckResultDetail::Http(ref http_result) = check_result.details {
                        let NotifyCondition{contains, no_contains, regex} = &single_rule.condition;
                        if contains.contains(&http_result.basic_avaliable.res_status_code.map(|code| code as u16).unwrap_or(0)) {
                            // 触发告警
                            self.notify.send_alert_message(format!("Response code {} triggered alert, in contains list: {:?}", http_result.basic_avaliable.res_status_code.map(|code| code as u16).unwrap_or(0), contains)).await;
                            return ;
                        }
                        if !no_contains.contains(&http_result.basic_avaliable.res_status_code.map(|code| code as u16).unwrap_or(0)) {
                            self.notify.send_alert_message(format!("Response code {} triggered alert, not in no_contains list: {:?}", http_result.basic_avaliable.res_status_code.map(|code| code as u16).unwrap_or(0), no_contains)).await;
                            return ;
                        }
                        // 正则匹配
                        let regex_r = Regex::new(&regex).unwrap();
                        // 判断 regex 不为 “”
                        if regex.len() > 0 && regex_r.is_match(&http_result.basic_avaliable.res_status_code.map(|code| code.to_string()).unwrap_or("".to_string())) {
                            // 触发告警
                           self.notify.send_alert_message(format!("Response code {} triggered alert, regex matched: {:?}", http_result.basic_avaliable.res_status_code.map(|code| code as u16).unwrap_or(0), regex)).await;
                            return ;
                        }
                    }
                    return;
                },
                _ => {
                    // 其他规则类型的检查方法

                }
            }

        }

    }
}