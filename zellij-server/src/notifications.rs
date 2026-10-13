use std::collections::BTreeMap;

use zellij_utils::input::options::HostNotificationProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationProtocol {
    Osc99,
    Osc9,
    Bell,
    Off,
}

impl Default for NotificationProtocol {
    fn default() -> Self {
        NotificationProtocol::Osc9
    }
}

impl NotificationProtocol {
    pub fn from_host_terminal_env(host_terminal_env: &BTreeMap<String, String>) -> Self {
        let is_kitty = host_terminal_env
            .get("TERM")
            .map(|term| term == "xterm-kitty")
            .unwrap_or(false)
            || host_terminal_env.contains_key("KITTY_WINDOW_ID");
        if is_kitty {
            NotificationProtocol::Osc99
        } else {
            NotificationProtocol::Osc9
        }
    }
    pub fn resolve(
        configured: HostNotificationProtocol,
        host_terminal_env: &BTreeMap<String, String>,
    ) -> Self {
        match configured {
            HostNotificationProtocol::Auto => Self::from_host_terminal_env(host_terminal_env),
            HostNotificationProtocol::Osc9 => NotificationProtocol::Osc9,
            HostNotificationProtocol::Osc99 => NotificationProtocol::Osc99,
            HostNotificationProtocol::Bell => NotificationProtocol::Bell,
            HostNotificationProtocol::Off => NotificationProtocol::Off,
        }
    }
    pub fn render(&self, title: &str, body: &str) -> Option<String> {
        // 空内容守卫: title 与 body 均为空时这不是一条通知 (典型形态: OSC 99 能力
        // 探测/控制帧, metadata 非空但无内容), 转发渲染只会产出空 OSC 9 / 裸铃声,
        // 在 wezterm 等终端上表现为无标题桌面通知 — 直接丢弃。
        if title.is_empty() && body.is_empty() {
            return None;
        }
        let payload = if title.is_empty() {
            body.to_owned()
        } else if body.is_empty() {
            title.to_owned()
        } else {
            format!("{}: {}", title, body)
        };
        match self {
            NotificationProtocol::Osc99 => Some(format!("\u{1b}]99;;{}\u{7}", payload)),
            NotificationProtocol::Osc9 => Some(format!("\u{1b}]9;{}\u{7}", payload)),
            NotificationProtocol::Bell => Some("\u{7}".to_owned()),
            NotificationProtocol::Off => None,
        }
    }
}

#[cfg(test)]
#[path = "./unit/notifications_tests.rs"]
mod notifications_tests;
