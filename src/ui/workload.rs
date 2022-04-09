// use system library

// use thirdparty library
use tui::{
    backend::Backend,
    layout::{
        Constraint, Direction, Layout, Rect, 
    },
    Frame,
};
// use private library
use crate::app::{App};

pub fn draw<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .horizontal_margin(1)
        .vertical_margin(1).split(area);

}
