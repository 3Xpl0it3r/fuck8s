use std::{sync::mpsc, time::Duration, thread};
use crossterm::event;
use log::{info, debug, error, warn};


use crate::event::Key;
use futures::TryFutureExt;


#[derive(Debug,Copy, Clone)]
pub struct EventConfig{
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for EventConfig{
    fn default() -> EventConfig{
        EventConfig{
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(200),
        }
    }
}

/// An occurred event
pub enum Event<T>{
    Input(T),
    Tick,
}


pub struct Events{
    rx: mpsc::Receiver<Event<Key>>,
    _tx: mpsc::Sender<Event<Key>>,
}

impl Events {
    pub fn new(tick_rate: u64) -> Events{
        Events::with_config(EventConfig{
            tick_rate: Duration::from_millis(tick_rate),
            ..Default::default()
        })
    }

    pub fn with_config(config: EventConfig) -> Events{
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        thread::spawn(move||{
            loop {
                if crossterm::event::poll(config.tick_rate).unwrap(){
                    if let event::Event::Key(key) = event::read().unwrap(){
                        error!("Debug in Events handler : {:?}", key);
                        let key = Key::from(key);
                        event_tx.send(Event::Input(key)).unwrap();
                    }
                }
                event_tx.send(Event::Tick).unwrap();
            }
        });
        Events{rx, _tx: tx}
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError>{
        self.rx.recv()
    }
}
