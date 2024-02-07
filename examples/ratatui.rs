use std::io::{self, stdout};

use catppuccin::PALETTE;
use ratatui::{prelude::*, widgets::*};

fn main() -> io::Result<()> {
    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(0),
        },
    )?;
    for flavor in &PALETTE {
        terminal.insert_before(4, |buf| {
            let analogous: Vec<Span> = flavor
                .colors
                .into_iter()
                .filter(|c| c.accent)
                .map(|c| "██".fg(*c)) // fg accepts any type that implements Into<Color>
                .collect::<Vec<Span>>();
            let monochromatic: Vec<Span> = flavor
                .colors
                .into_iter()
                .filter(|c| !c.accent)
                .map(|c| "██".fg(*c)) // fg accepts any type that implements Into<Color>
                .collect();

            let width = buf.area.width;
            Paragraph::new(flavor.name).render(Rect::new(0, 0, width, 1), buf);
            Paragraph::new(Line::from(analogous)).render(Rect::new(0, 1, width, 1), buf);
            Paragraph::new(Line::from(monochromatic)).render(Rect::new(0, 2, width, 1), buf);
        })?;
    }

    Ok(())
}
