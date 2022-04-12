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
use crate::app::{App, state_machine};
use super::util::draw_log_widget;

pub fn draw<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    app.active_block = state_machine::StateMachine::WorkloadBlock;
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .horizontal_margin(1)
        .vertical_margin(1).split(area);
    draw_log_widget(f, chunks[1]);
}
