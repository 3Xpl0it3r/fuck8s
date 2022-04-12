use tui::Frame;
use tui::layout::{Rect, Alignment};
use tui::backend::Backend;
use tui::text::{Spans, Span};
use tui::widgets::Paragraph;
use tui::style::Style;

// logo widget
pub fn draw_banner_widget<B> (f: &mut Frame<B>, area: Rect)
    where
        B: Backend
{
    let mut pic: Vec<Spans> = Vec::new();
    let pic_split = Small.split(|s| s.len() == 0);

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
const Large: &[&str] = &[
    " _______ ", " __    __  ", "  ______ ", " __  ___ ",  "  ___   ", "     _______.", "",
    "|   ____|", "|  |  |  | ", " /      |", "|  |/  / ", " / _ \\  ", "    /       |", "",
    "|  |__   ", "|  |  |  | ", "|  ,----'", "|  '  /  ", "| (_) | ", "   |   (----`", "",
    "|   __|  ", "|  |  |  | ", "|  |     ", "|    <   ", " > _ <  ", "    \\   \\    ", "",
    "|  |     ", "|  `--'  | ", "|  `----.", "|  .  \\  ", "| (_) | ", ".----)   |   ", "",
    "|__|     ", " \\______/  ", " \\______|", "|__|\\__\\ ", " \\___/  ", "|_______/    ", "",
];


const Middle: &[&str] = &[
    "   ___  ", "         ", "      __      ", "   __     ", "        ", "",
    " /'___\\ ", "         ", "     /\\ \\     ", " /'_ `\\   ", "        ", "",
    "/\\ \\__/ ", " __  __  ", "  ___\\ \\ \\/'\\ ", "/\\ \\L\\ \\  ", "  ____  ", "",
    "\\ \\ ,__\\", "/\\ \\/\\ \\ ", " /'___\\ \\ , < ", "\\/_> _ <_ ", " /',__\\ ", "",
    " \\ \\ \\_/", "\\ \\ \\_\\ \\", "/\\ \\__/\\ \\ \\`", "\\ /\\ \\L\\ \\", "/\\__, `\\", "",
    "  \\ \\_\\ ", " \\ \\____/", "\\ \\____\\ \\_\\ ", "\\_\\ \\____/", "\\/\\____/", "",
    "   \\/_/ ", "  \\/___/ ", " \\/____/ \\/_/\\", "/_/\\/___/ ", " \\/___/ ", "",
];

const Small: &[&str] = &[
    "   ___","   ","     "," __ ","  ___ ","   ","",
    "  / _/","_ _","_____","/ /_","_( _ )","___","",
    " / _/ ","// ","/ __/","  '_","/ _  (","_-<","",
    "/_/ \\_",",_/","\\__/_","/\\_\\","\\___/_","__/","",
];
