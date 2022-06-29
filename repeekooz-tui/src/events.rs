use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use lazy_static::lazy_static;

use crate::app::{App, AppMode};
use crate::draw;

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;

lazy_static! {
    pub static ref NOTIFY: (Sender<HGEvent>, Receiver<HGEvent>) = bounded(1024);
    pub static ref GG_COMBINE: AtomicBool = AtomicBool::new(false);
}

#[derive(Debug, Clone)]
pub enum HGEvent {
    UserEvent(KeyEvent),

    NotifyEvent(Notify),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Notify {
    /// 重绘界面
    Redraw,

    /// 退出应用
    Quit,

    /// 弹出窗口展示消息
    Message(Message),

    /// tick
    Tick,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Error(String),

    Warn(String),

    Tips(String),
}

impl Default for Message {
    fn default() -> Self {
        Message::Error(String::default())
    }
}

pub fn handle_key_event(event_app: Arc<Mutex<App>>) {
    let (sender, receiver) = unbounded();

    std::thread::spawn(move || loop {
        if let Ok(Event::Key(event)) = crossterm::event::read() {
            sender.send(HGEvent::UserEvent(event)).unwrap();
        }
    });
    std::thread::spawn(move || loop {
        if let Ok(HGEvent::UserEvent(key_event)) = receiver.recv() {
            let mut app = event_app.lock().unwrap();
            match (key_event.modifiers, key_event.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                    quit();
                    break;
                }
                _ => {}
            }
        }
    });
}

pub fn redraw() {
    NOTIFY.0.send(HGEvent::NotifyEvent(Notify::Redraw)).unwrap();
}

pub fn quit() {
    NOTIFY.0.send(HGEvent::NotifyEvent(Notify::Quit)).unwrap();
}

pub fn err(msg: String) {
    NOTIFY
        .0
        .send(HGEvent::NotifyEvent(Notify::Message(Message::Error(msg))))
        .unwrap();
}

pub fn warn(msg: String) {
    NOTIFY
        .0
        .send(HGEvent::NotifyEvent(Notify::Message(Message::Warn(msg))))
        .unwrap();
}

pub fn tips(msg: String) {
    NOTIFY
        .0
        .send(HGEvent::NotifyEvent(Notify::Message(Message::Tips(msg))))
        .unwrap();
}

pub fn show_help() {
    tips(
        r###"CTRL j(Down)/k(Up) 切换 浏览/搜索 模式
搜索模式
Ctrl+h 获得帮助
输入 #{数字} 按期数搜索
输入 ${类别} 按类别搜索
其他按关键字搜索

浏览模式：
k(Up)/j(Down) 上/下 移动一行
u(PageUp)/d(PageDown) 上/下 移动五行
gg(Home) 移动至首行
G(End)  移动至末行
h(Left)/l(Right) 前/后 翻页
o | Ctrl+Right(Left) 查看（关闭）详细
s 帮 HG 点个小星星吧
ENTER 打开 GitHub 页面
q 退出应用"###
            .into(),
    );
}

pub fn tick() {
    NOTIFY.0.send(HGEvent::NotifyEvent(Notify::Tick)).unwrap();
}

pub fn handle_notify(notify_app: Arc<Mutex<App>>) {
    // first draw
    redraw();

    if notify_app.lock().unwrap().show_help {
        show_help();
    }

    std::thread::spawn(move || loop {
        tick();
        std::thread::sleep(Duration::from_secs(1));
    });

    let notify_recv = NOTIFY.1.clone();

    loop {
        if let Ok(HGEvent::NotifyEvent(notify)) = notify_recv.recv() {
            match notify {
                Notify::Redraw | Notify::Tick => {
                    let mut app = notify_app.lock().unwrap();

                    draw::redraw(&mut app);
                }
                Notify::Message(msg) => {
                    let mut app = notify_app.lock().unwrap();
                    draw::redraw(&mut app);
                }
                Notify::Quit => {
                    break;
                }
            }
        }
    }
}
