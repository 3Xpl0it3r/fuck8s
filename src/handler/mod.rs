

use crate::event::Key;
use crate::app::App;


pub fn handler_app(key: Key, app: &mut App){
    match key {
        Key::Left => app.on_left(),
        Key::Right => app.on_right(),
        Key::Down => app.on_down(),
        Key::Up => app.on_up(),
        _ => {}
    }
}
