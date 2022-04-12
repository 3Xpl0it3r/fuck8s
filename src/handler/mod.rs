mod home;
mod network;
mod workload;

use crate::event::Key;
use crate::app::App;
use crate::app::state_machine::{StateMachine};



pub fn handler_app(key: Key, app: &mut App){
    match &app.active_block{
        StateMachine::WorkloadBlock => network::handler(key, app),
        StateMachine::NetworkBlock => workload::handler(key, app),
        StateMachine::HomeBlock | _ => home::handler(key, app),
    }
}


