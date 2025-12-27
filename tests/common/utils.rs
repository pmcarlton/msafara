use ratatui::{backend::TestBackend, Terminal};
use ratatui::buffer::Buffer;

use termal_msa::ui::render_ui;

pub fn render(app: &termal_msa::app::App, w: u16, h: u16) -> Buffer {
    let backend = TestBackend::new(w, h);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal.draw(|f| render_ui(app, f)).expect("draw");
    terminal.backend().buffer().clone()
}

pub fn buffer_text(buf: &Buffer) -> String {
    let area = buf.area;
    let mut out = String::new();
    for y in 0..area.height {
        for x in 0..area.width {
            out.push(buf.get(x, y)
                .symbol()
                .chars()
                .next()
                .unwrap_or(' '));
            }
        out.push('\n');
    }
    out
}

