//! Example demonstrating integration with the `ratatui` crate.
use std::io::{self, stdout};

use catppuccin::PALETTE;
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Stylize as _,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    Terminal, TerminalOptions, Viewport,
};

fn main() -> io::Result<()> {
    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(0),
        },
    )?;
    for flavor in &PALETTE {
        terminal.insert_before(8, |buf| {
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
            let ansi_normals: Vec<Span> = flavor
                .ansi_colors
                .into_iter()
                .filter(|c| c.code < 8)
                .map(|c| "██".fg(*c)) // fg accepts any type that implements Into<Color>
                .collect::<Vec<Span>>();
            let ansi_brights: Vec<Span> = flavor
                .ansi_colors
                .into_iter()
                .filter(|c| c.code >= 8)
                .map(|c| "██".fg(*c)) // fg accepts any type that implements Into<Color>
                .collect::<Vec<Span>>();

            let width = buf.area.width;
            Paragraph::new(flavor.name.to_string()).render(Rect::new(0, 0, width, 1), buf);
            Paragraph::new(Line::from(analogous)).render(Rect::new(0, 1, width, 1), buf);
            Paragraph::new(Line::from(monochromatic)).render(Rect::new(0, 2, width, 1), buf);
            Paragraph::new(format!("{} ANSI", flavor.name)).render(Rect::new(0, 4, width, 1), buf);
            Paragraph::new(Line::from(ansi_normals)).render(Rect::new(0, 5, width, 1), buf);
            Paragraph::new(Line::from(ansi_brights)).render(Rect::new(0, 6, width, 1), buf);
        })?;
    }

    Ok(())
}
