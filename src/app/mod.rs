use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};
use k8s_openapi::api::core::v1::{
    self as kube_core_v1,
    Pod, Event, Service
};

use eyre::Result;
use tui::widgets::ListState;
use tokio::sync::mpsc::Receiver;
use k8s_openapi::Resource;
use log::{warn};
use std::any::Any;
use futures::{FutureExt, StreamExt};

use crate::storage::MemoryShareStorage;


pub enum InputMode {
    Normal,
    Editing,
}

const APP_TITLE: &str = "fuck8s";

pub struct App<'a> {
    pub title: &'static str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    // channel

    pub storage: MemoryShareStorage,
    pub enhanced_graphics: bool,
    // search/ input
    pub input_mode: InputMode,
    pub input_buffer: String,

    // kubernetes
    pub pod_buffer: Vec<Pod>,

    // 面版属性 dashabord
}

impl<'a> App<'a> {
    pub fn new(db : MemoryShareStorage) -> App<'a> {
        App {
            title: APP_TITLE,
            should_quit: false,
            tabs: TabsState::new(vec!["概况", "工作负载", "网络", "神秘功能"]),
            enhanced_graphics: false,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            pod_buffer: Vec::new(),

            storage:db,
        }
    }


    pub fn on_up(&mut self) {
        self.tabs.previous()
    }

    pub fn on_down(&mut self) {
        self.tabs.next()
    }

    pub fn on_right(&mut self) {
        self.tabs.previous()
    }

    pub fn on_left(&mut self) {
        self.tabs.next()
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            },
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        //
    }
}



pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // 总共有多少个节点
}
