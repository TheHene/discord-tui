use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Gauge, List, ListState, Paragraph, Row, Table, Text},
    Frame,
};

pub fn draw_main_layout(terminal) -> Result<(), ()> {
    draw_text_chanels(terminal);
    draw_voice_chanels();
    draw_text_chat();
    draw_text_chat_input();
}

fn draw_text_chanels(terminal) {}
fn draw_text_chat() {}
fn draw_text_chat_input() {}
fn draw_voice_chanels() {}
