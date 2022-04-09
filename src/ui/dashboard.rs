// use system library
use std::borrow::BorrowMut;
// use thirdparty library
use tui::{
    backend::Backend,
    layout::{ Constraint, Direction, Layout, Rect, Alignment, },
    style::{ Color, Modifier, Style, },
    text::{ Span, Spans ,Text, },
    widgets::{
        canvas::{ Canvas, Line, Map, MapResolution, Rectangle, },
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap, BorderType

    },
    Frame,
    symbols,
};
// use private library
use crate::app::{self, App, StatefulList, InputMode};

use super::util::{draw_log_widget, draw_help_widget, draw_logo_widget};
use tui::widgets::GraphType;

const DATA2: [(f64, f64); 7] = [
    (0.0, 0.0),
    (10.0, 1.0),
    (20.0, 0.5),
    (30.0, 1.5),
    (40.0, 1.0),
    (50.0, 2.5),
    (60.0, 3.0),
];


///
///
/// ｜--------------------------|-----｜          ->
/// ｜  node|  pod|  mem| cpu   |     ｜          log layout
/// ｜------------------------- |help ｜         layout1        log layot
/// ｜   node1    status        |     ｜
/// ｜--------------------------|-----｜         ->
/// ｜       log                      ｜          log layout
/// ｜--------------------------------｜
///
///
pub fn draw<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let out_block = Block::default().borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());
    f.render_widget(out_block, area);
    let _wrapper_area = Rect::new(area.x + 1, area.y + 1, area.width-2, area.height-2);
     let wrapper_chunk = Layout::default().direction(Direction::Vertical)
         .constraints([Constraint::Percentage(70), Constraint::Percentage(30)]).split(_wrapper_area);
    // 绘画上层
    draw_dashboard_content(f, app, wrapper_chunk[0]);

    draw_log_widget(f, wrapper_chunk[1]);

}

fn draw_dashboard_content<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend
{
    let whole_chunks = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(20)]).split(area);

    // // cluster status
    // 集群整体状态
    draw_kubernetes_cluster(f, app, whole_chunks[0]);

    // draw help
    draw_help_widget(f, whole_chunks[1]);
}


fn draw_kubernetes_cluster<B>(f: &mut Frame<B>, app: &mut App, area:Rect)
where
    B: Backend
{

    let out_block = Block::default().borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());
    f.render_widget(out_block, area);
    let _wrapper_area = Rect::new(area.x + 1, area.y + 1, area.width-2, area.height-2);
    let wrapper_chunk = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)]).split(_wrapper_area);


    // // 集群整体状态 ，四个gauge
    let cluster_chunks = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25),Constraint::Percentage(25)].as_ref())
        .split(wrapper_chunk[0]);

    let pod = widget_charts("Pod Usage");
    let cpu = widget_charts("Cpu Usage");
    let mem = widget_charts("Mem Usage");
    let disk = widget_charts("Disk Usage");

    //
    f.render_widget(pod, cluster_chunks[0]);
    f.render_widget(cpu, cluster_chunks[1]);
    f.render_widget(mem, cluster_chunks[2]);
    f.render_widget(disk, cluster_chunks[3]);
    // 每个节点单独的状态
    // table
    // draw_logo_widget(f, wrapper_chunk[1]);

    let node_detail_tabls = widget_table();
    f.render_widget(node_detail_tabls, wrapper_chunk[1]);

}



fn draw_search<B>(f: &mut Frame<B>, app: &mut App, ares: Rect)
    where
        B: Backend,
{
    let text = Text::from(Spans::from(vec![
        Span::styled("  ", Style::default()),
    ]));
    let search = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).style(Style::default()));


    let input = Paragraph::new(app.input_buffer.as_ref())
        .style(match app.input_mode{
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Green)
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, ares);

    f.set_cursor(ares.x + 20, ares.y + 1);
}

fn widget_charts<'a>(title: &'a str)->Chart<'a>{
    let datasets = vec![Dataset::default()
        .name("data")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&DATA2)];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(title, Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )).title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("date/time")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 50.0])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("25"),
                    Span::styled("50", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Usage")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 5.0])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("2.5"),
                    Span::styled("5", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    chart
}

fn widget_table<'a>()->Table<'a> {
    let mock_data = vec![
        vec!["minikube", "control-plane,master", "4", "8151352Ki", "61255492Ki", "192.168.49.2", "Ubuntu 20.04.2 LTS", "Ready"],
    ];
    let header_cells = ["Node", "Role", "Cpu", "Mem", "Disk", "ExternalIp", "Os", "Status"].iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Gray)));

    let header = Row::new(header_cells)
        .style(Style::default())
        .height(1)
        .bottom_margin(1);

    let rows = mock_data.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });

    Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default()).title("节点🔎"))
        .highlight_style(Style::default())
        .widths(&[
            Constraint::Min(10),        // node
            Constraint::Min(20),        // role
            Constraint::Min(3),        // cpu
            Constraint::Min(10),        // mem
            Constraint::Min(10),        // disk
            Constraint::Min(20),        // ip
            Constraint::Min(20),        // os
            Constraint::Min(5),        // status
        ])

}