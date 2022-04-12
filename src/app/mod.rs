pub mod state_machine;

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
use log::{warn};
use std::any::Any;
use futures::{FutureExt, StreamExt};

use crate::storage::{MemoryShareStorage, KubeNodeStatus, Resource};
use state_machine::StateMachine;


pub enum InputMode {
    Normal,
    Editing,
}

pub enum ActiveBlock{
    HomeIndex, //首页
}

const APP_TITLE: &str = "fuck8s";

pub struct App<'a> {
    pub title: &'static str,
    pub should_quit: bool,
    pub menu_tabs: MenuTabsState<'a>,
    // channel

    pub storage: MemoryShareStorage,
    pub enhanced_graphics: bool,
    // search/ input
    pub input_mode: InputMode,
    pub input_buffer: String,

    // kubernetes
    pub pod_buffer: Vec<Pod>,
    pub pod_total_running: i32,

    // Statement Machine
    pub active_block: StateMachine,

    // 面版属性 dashabord
    pub node_list: Vec<KubeNodeStatus>,
}

impl<'a> App<'a> {
    pub fn new(db : MemoryShareStorage) -> App<'a> {
        App {
            title: APP_TITLE,
            should_quit: false,
            menu_tabs: MenuTabsState::new(vec!["概况", "工作负载", "网络"]),
            enhanced_graphics: false,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            pod_buffer: Vec::new(),
            node_list: Vec::<KubeNodeStatus>::new(),
            storage:db,
            pod_total_running: 1,

            active_block: StateMachine::Empty,

        }
    }

    pub fn on_tick(&mut self) {
        //
    }

    // 初始化数据
    pub async fn initialized(&mut self) {
        self.node_list.clear();
        let node_storage = self.storage[2].lock().await;
        for (_, Resource::KubeNode(node)) in node_storage.iter(){
            self.node_list.push(node.clone())
        }
    }
}


pub struct MenuTabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> MenuTabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> MenuTabsState {
        MenuTabsState { titles, index: 0 }
    }
    fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    pub fn on_up(&mut self) {
        self.previous()
    }

    pub fn on_down(&mut self) {
        self.next()
    }

    pub fn on_right(&mut self) {
        self.previous()
    }

    pub fn on_left(&mut self) {
        self.next()
    }


}
