
use std::fmt::{self, Display, Formatter};
use crossterm::event;


#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key{
    /// Both Enter (or Return) and numpad Enter
    Enter,

    /// left key arrow
    Left,
    /// right key arrow
    Right,
    /// up key arrow
    Up,
    /// down key arrow
    Down,

    Char(char),
    Ctrl(char),
    Alt(char),
    Unknown,
}

impl Key {
    pub fn from_f(n: u8) -> Key{
        match n {
            _ => panic!("unknown function key: F{}", n),
        }
    }
}


impl Display for Key{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match *self {
            Key::Alt(' ') => write!(f, "<Alt+Space>"),
            Key::Ctrl(' ') => write!(f, "<Ctrl+Space>"),
            Key::Char(' ') => write!(f, "<Space>"),
            Key::Alt(c) => write!(f, "<Alt+{}>", c),
            Key::Ctrl(c) => write!(f, "<Ctrl+{}>",c),
            Key::Char(c)=>write!(f, "{}", c),
            Key::Left 
                | Key::Right 
                | Key::Up 
                |Key::Down 
                => write!(f, "<{:?} Arrow Key>", self),
            _ => write!(f, "{:?}", self),
        }
    }
}

// Trans crossterm key event into custom_Key_event
impl From<event::KeyEvent> for Key{
    fn from(key_event: event::KeyEvent) -> Self{
        match key_event{
            event::KeyEvent{code: event::KeyCode::Backspace, ..} => Key::Enter,
            event::KeyEvent{code: event::KeyCode::Char(c),..} => Key::Char(c),
            event::KeyEvent{code: event::KeyCode::Left,..} => Key::Left,
            event::KeyEvent{code: event::KeyCode::Right,..} => Key::Right,
            event::KeyEvent{code: event::KeyCode::Up,..} => Key::Up,
            event::KeyEvent{code: event::KeyCode::Down,..} => Key::Down,

            _ => Key::Unknown,
        }
    }
}
