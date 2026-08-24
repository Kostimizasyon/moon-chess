use ratatui::{
    Frame, layout::{Alignment, Constraint, Layout, Rect}, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::game::{board::{Point}, slot::SlotState};
use crate::tui::app::{App};

pub fn render(app: &mut App, frame: &mut Frame) {
    let [header_area, grid_area] = Layout::vertical([
        Constraint::Length(7),
        Constraint::Fill(1),
    ]).areas(frame.area());

    render_header(app, frame, header_area);
    render_grid(app, frame, grid_area);
}

fn render_header(app: &mut App, frame: &mut Frame, area: Rect) {
    frame.render_widget(Paragraph::new(
        format!("
            Press 'Esc' to quit
            Arrow keys and enter for P1, Wasd and space for P2
            Current Turn: {}
            Total Move Count  : {}
            ", 
            app.board.curr_turn, app.board.total_moves
        )).block(
            Block::default()
                .title("Moon Chess")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        , area);
}

fn render_block(app: &mut App, point: Point, frame: &mut Frame, area: Rect) {
    let is_focus = app.point.to_tuple() == point.to_tuple();

    let fg_color = if is_focus { Color::White } else { Color::Magenta };

    let cur_state =  app.board.get_slot_state(&point);

    let bg_color = match cur_state {
        SlotState::P1 => Color::Cyan,
        SlotState::P2 => Color::Green,
                _     => Color::Reset
    };

    let block = Block::default()
            .title(format!("{}, {}", point.to_tuple().0, point.to_tuple().1))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .style(Style::default()
            .fg(fg_color)
            .bg(bg_color)
        );
    
    frame.render_widget(
        block,
        area
    );

}

fn render_grid(app: &mut App, frame: &mut Frame, area: Rect) {
    let rows = Layout::vertical([Constraint::Fill(1); 3]).split(area);

    for (y, row) in rows.iter().enumerate() {
        let cols = Layout::horizontal([Constraint::Fill(1); 3]).split(*row);
        for (x, cell_area) in cols.iter().enumerate() {
            render_block(app, Point::new(x, y), frame, *cell_area);
        }
    }
}
