
use crate::app::{App, state_machine};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};
use super::util::draw_log_widget;

pub fn draw<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    app.active_block = state_machine::StateMachine::NetworkBlock;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ]
                .as_ref(),
        )
        .split(area);
    draw_log_widget(f, chunks[1]);
}
