//! Example demonstrating integration with the `ansi_term` crate.
use catppuccin::PALETTE;

const fn ansi(color: &catppuccin::Color) -> ansi_term::Colour {
    ansi_term::Colour::RGB(color.rgb.r, color.rgb.g, color.rgb.b)
}

fn main() {
    for flavor in &PALETTE {
        let heading = format!(
            "{} ({})",
            flavor.name,
            if flavor.dark { "dark" } else { "light" }
        );
        println!(
            "{}\n",
            ansi_term::Style::new().underline().bold().paint(heading)
        );

        for color in flavor {
            let name = format!(
                "{}{}",
                color.name,
                if color.accent { " (accent)" } else { "" }
            );
            let rgb = format!(
                "rgb({:3}, {:3}, {:3})",
                color.rgb.r, color.rgb.g, color.rgb.b
            );
            let hsl = format!(
                "hsl({:3.0}, {:5.3}, {:5.3})",
                color.hsl.h, color.hsl.s, color.hsl.l
            );
            println!(
                "{} {:18} →  {:6}  {:18}  {:18}",
                ansi(color).reverse().paint("  "),
                name,
                color.hex,
                rgb,
                hsl,
            );
        }
        println!();
    }
}
