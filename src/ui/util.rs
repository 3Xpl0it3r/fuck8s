use tui_logger::{
    TuiLoggerWidget,
};
use tui::{style::{Style, Color}, widgets::{Block, Borders}, layout::Alignment, Frame};
use tui::widgets::{Table, Paragraph, BorderType};
use tui::text::{Span, Spans};
use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Rect, Layout, Direction, Constraint};


// log widget for debug
pub fn draw_log_widget<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend
{
    let log_widget = TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_info(Style::default().fg(Color::Blue))
        .style_debug(Style::default().fg(Color::Green))
        .block(
            Block::default().title("Log").title_alignment(Alignment::Left)
            .borders(Borders::ALL).border_type(tui::widgets::BorderType::Rounded).border_style(Style::default().fg(Color::White).bg(Color::Black))
              );
    f.render_widget(log_widget, area);
}



// help draw windows
pub fn draw_help_widget<B>(f: &mut Frame<B>,  area: Rect)
where
    B: Backend
{

    let out_block = Block::default().borders(Borders::ALL)
        .border_type(BorderType::Rounded).title("Help").title_alignment(Alignment::Center)
        .style(Style::default());
    f.render_widget(out_block, area);
    let _wrapper_area = Rect::new(area.x + 1, area.y + 1, area.width-2, area.height-2);
    let key_style = Style::default();
    let value_style = Style::default();


    let mut rows = vec![];
    f.render_widget(Table::new(rows), _wrapper_area);
}


// logo widget
pub fn draw_logo_widget<B> (f: &mut Frame<B>, area: Rect)
where
    B: Backend
{
    let mut pic: Vec<Spans> = Vec::new();
    let pic_split = LOGO3.split(|s| s.len() == 0);

    for line in pic_split {
        let mut spans: Vec<Span> = Vec::new();
        for (i, item) in line.iter().enumerate() {
            spans.push(Span::styled(*item, Style::default()))
        }
        pic.push(Spans::from(spans));
    }
    let a :String = "".to_string();
    f.render_widget(Paragraph::new(pic).alignment(Alignment::Center), area);
}



#[rustfmt::skip]
const LOGO1: &[&str] = &[
    " _______ ", " __    __  ", "  ______ ", " __  ___ ",  "  ___   ", "     _______.", "",
    "|   ____|", "|  |  |  | ", " /      |", "|  |/  / ", " / _ \\  ", "    /       |", "",
    "|  |__   ", "|  |  |  | ", "|  ,----'", "|  '  /  ", "| (_) | ", "   |   (----`", "",
    "|   __|  ", "|  |  |  | ", "|  |     ", "|    <   ", " > _ <  ", "    \\   \\    ", "",
    "|  |     ", "|  `--'  | ", "|  `----.", "|  .  \\  ", "| (_) | ", ".----)   |   ", "",
    "|__|     ", " \\______/  ", " \\______|", "|__|\\__\\ ", " \\___/  ", "|_______/    ", "",
];


const LOGO2: &[&str] = &[
    "   ___  ", "         ", "      __      ", "   __     ", "        ", "",
    " /'___\\ ", "         ", "     /\\ \\     ", " /'_ `\\   ", "        ", "",
    "/\\ \\__/ ", " __  __  ", "  ___\\ \\ \\/'\\ ", "/\\ \\L\\ \\  ", "  ____  ", "",
    "\\ \\ ,__\\", "/\\ \\/\\ \\ ", " /'___\\ \\ , < ", "\\/_> _ <_ ", " /',__\\ ", "",
    " \\ \\ \\_/", "\\ \\ \\_\\ \\", "/\\ \\__/\\ \\ \\`", "\\ /\\ \\L\\ \\", "/\\__, `\\", "",
    "  \\ \\_\\ ", " \\ \\____/", "\\ \\____\\ \\_\\ ", "\\_\\ \\____/", "\\/\\____/", "",
    "   \\/_/ ", "  \\/___/ ", " \\/____/ \\/_/\\", "/_/\\/___/ ", " \\/___/ ", "",
];

const LOGO3: &[&str] = &[
    "   ___","   ","     "," __ ","  ___ ","   ","",
    "  / _/","_ _","_____","/ /_","_( _ )","___","",
    " / _/ ","// ","/ __/","  '_","/ _  (","_-<","",
    "/_/ \\_",",_/","\\__/_","/\\_\\","\\___/_","__/","",
];


pub fn get_help_docs()->Vec<Vec<String>>{
    vec![
        vec![
            'q'.to_string(),
            String::from("quit"),
        ]
    ]
}