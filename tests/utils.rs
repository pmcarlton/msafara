use ratatui::{backend::TestBackend, Terminal};
use ratatui::buffer::Buffer;

use termal_msa::app::App;
use termal_msa::ui::{
    render::render_ui,
    UI,
};

pub fn render(app: &mut App, w: u16, h: u16) -> Buffer {
    let backend = TestBackend::new(w, h);
    let mut terminal = Terminal::new(backend).expect("terminal");
    // terminal.draw(|f| termal_msa::ui::draw(app, f)).expect("draw");
    let mut ui = UI::new(app);
    terminal.draw(|f| render_ui(f, &mut ui)).expect("draw");
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

