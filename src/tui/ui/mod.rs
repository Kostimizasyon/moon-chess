use ratatui::{
    Frame, layout::{Alignment, Constraint, Layout, Rect}, style::{Color, Style}, text::Line, widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::game::{board::Point, slot::SlotState};
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
        format!(
            "Press 'Esc' to quit
Press arrow keys or 'hjkl' to move and 'space' to mark
Current Turn: {}
Move Count  : {}
", app.board.curr_turn, app.board.total_moves
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

fn render_block(app: &App, point: Point, frame: &mut Frame, area: Rect) {
    let is_focus = app.point.to_tuple() == point.to_tuple();

    let fg_color = if is_focus { Color::White } else { Color::Magenta };
    let is_marked = !matches!(app.board.get_slot_state(&point), SlotState::Empty);
    let bg_color = if is_marked { fg_color } else { Color::Reset };

    let block = Block::default()
            .title(format!("{}, {}", point.to_tuple().0, point.to_tuple().1))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .style(Style::default().fg(fg_color).bg(bg_color));
    
    let inner_area = block.inner(area);

    let info = Line::from(format!("{}", app.board.get_slot_state(&point)));
    frame.render_widget(
        block,
        area
    );
    frame.render_widget(info, inner_area);

}

fn render_grid(app: &App, frame: &mut Frame, area: Rect) {
    let rows = Layout::vertical([Constraint::Fill(1); 3]).split(area);

    for (y, row) in rows.iter().enumerate() {
        let cols = Layout::horizontal([Constraint::Fill(1); 3]).split(*row);
        for (x, cell_area) in cols.iter().enumerate() {
            render_block(app, Point::new(x, y), frame, *cell_area);
        }
    }
}
