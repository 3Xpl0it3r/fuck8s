// private module
mod dashboard;
mod network;
mod workload;
mod util;

use std::{ sync::Arc, io, };

use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{warn};

use eyre::Result;
use tui::{
    backend::{Backend, CrosstermBackend}, Terminal, Frame,
    layout::{Layout, Constraint, Alignment},
    text ::{Spans, Span},
    widgets::{Tabs, Block, Borders, Paragraph},
    style::{Style, Color},
};
use tokio::{time::{sleep, Duration}};


use tui::{
    layout::Rect,
    widgets::BorderType,
};
use tui_logger::TuiLoggerWidget;

use crate::{
    app::App, 
    event::{Events, Key, Event, EventConfig},
};
use self::util::draw_logo_widget;
use crate::handler::handler_app;


use self::{
    dashboard::draw as draw_dashboard,
    network::draw as draw_network,
    workload::draw as draw_workload,
};
use crate::ui::util::draw_log_widget;

pub async fn start_ui(app: Arc<tokio::sync::Mutex<App<'_>>>) -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // terminal.clear()?;
    // terminal.hide_cursor()?;

    let events = Events::with_config(EventConfig::default());

    loop {
        let mut app = app.lock().await;
        // 初始化数据

        // check windows size is ok 

        // get current route  路由区分应该处理哪个界面， 防止在刷新的时候重新回到main界面
        terminal.draw(|f| draw(f, &mut app))?;

        match events.next()?{
            Event::Input(key) => {
                warn!("Warn Key from Event Handler: {}", key);
                if key == Key::Ctrl('c') || key == Key::Char('q') {
                    break;
                }
                handler_app(key, &mut app);
            },
            Event::Tick => {
                sleep(Duration::from_millis(20)).await;
            },
        }
    }

    // restore
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
            )?;

    terminal.show_cursor()?;

    Ok(())
}

// entry point
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .split(f.size());
    // mem tables
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(app.title).title_alignment(Alignment::Center))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);


    let boader = Block::default().border_type(BorderType::Rounded).title("Termail").borders(Borders::ALL);
    f.render_widget(boader, chunks[1]);


    match app.tabs.index {
        0 => draw_dashboard(f, app, chunks[1]),      // dashboard monitor
        1 => draw_workload(f, app, chunks[1]),
        2 => draw_network(f, app, chunks[1]),
        _ => {}
    }


    // // foot ,license
    // let note_help_message: &'static str = "Help: q->exit";
    // let foot = Paragraph::new(note_help_message)
    //     .style(Style::default())
    //     .block(Block::default().style(Style::default()).border_type(BorderType::Rounded).borders(Borders::ALL))
    //     .alignment(Alignment::Left);
    // f.render_widget(foot, chunks[2]);
}




