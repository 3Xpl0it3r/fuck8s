use crate::app::App;
use crate::event::Key;


pub fn handler(key :Key, app: &mut App){
    match key {
        Key::Left => app.menu_tabs.on_left(),
        Key::Right => app.menu_tabs.on_right(),
        Key::Down => app.menu_tabs.on_down(),
        Key::Up => app.menu_tabs.on_up(),
        _ => {}
    }
}