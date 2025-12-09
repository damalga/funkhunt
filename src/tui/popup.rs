// src/tui/popup.rs
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::components;
use super::state::TuiState;

/// Renderiza el popup para agregar nueva carpeta
pub fn render_add_folder_popup(frame: &mut Frame, state: &TuiState) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    // Renderizar interfaz base
    components::render_header(frame, state, main_chunks[0]);
    components::render_book_list(frame, state, body_chunks[0]);
    components::render_book_details(frame, state, body_chunks[1]);
    components::render_adding_folder_footer(frame, main_chunks[2]);

    // Superponer modal en panel izquierdo
    render_folder_modal(frame, state, body_chunks[0]);
}

/// Renderiza el modal de agregar carpeta
fn render_folder_modal(frame: &mut Frame, state: &TuiState, area: Rect) {
    // Fondo oscuro para el panel izquierdo
    let background = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(background, area);

    // Marco del modal centrado
    let inner_modal = centered_in_rect(80, 40, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" AGREGAR CARPETA ")
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_widget(block, inner_modal);

    // Contenido del modal
    let modal_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(1), Constraint::Length(3)])
        .split(inner_modal);

    let instruction = Paragraph::new("Ruta de carpeta:").style(Style::default().fg(Color::Cyan));
    frame.render_widget(instruction, modal_chunks[0]);

    let input_display = state.input_buffer.clone();
    let input = Paragraph::new(input_display)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(input, modal_chunks[1]);
}

/// Centra un rectángulo dentro de otro rectángulo
fn centered_in_rect(percent_x: u16, percent_y: u16, outer: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(outer);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}
