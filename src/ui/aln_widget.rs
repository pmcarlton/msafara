use ratatui::{
    prelude::{
        Buffer,
        Rect,
    },
    style::Style,
    widgets::Widget
};

use crate::ui::get_residue_style;

pub struct AlnWidget<'a> {
    pub sequences: &'a [String],
    pub ordering: &'a [usize],

    pub top_i: usize,
    pub left_j: usize,

    // pub video_mode: VideoMode, // whatever your type is
    // pub theme: Theme,          // whatever your type is
    // pub colormap: &'a ColorMap, // your mapping residue -> something
}

impl<'a> AlnWidget<'a> {
    fn cell_style(&self, ch: u8) -> Style {
        let cur_char = ch as char;
        // get_residue_style(self.video_mode, &self.theme, self.colormap.get(cur_char))
        get_residue_style()
    }
}

impl<'a> Widget for AlnWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Clear the aln pane - especially needed after zoom-outs.
        for y in area.y..area.y.saturating_add(area.height) {
            for x in area.x..area.x.saturating_add(area.width) {
                buf.get_mut(x, y).reset();
            }
        }

        // visible dimensions
        let rows = area.height as usize;
        let cols = area.width as usize;

        for r in 0..rows {
            let i = self.top_i + r;
            if i >= self.ordering.len() { break; }

            let seq = self.sequences[self.ordering[i]].as_bytes();

            for c in 0..cols {
                let j = self.left_j + c;
                if j >= seq.len() { break; }

                let ch = seq[j];
                let style = self.cell_style(ch);

                let x = area.x + c as u16;
                let y = area.y + r as u16;

                buf.get_mut(x, y)
                    .set_char(ch as char)
                    .set_style(style);
            }
        }
    }
}
