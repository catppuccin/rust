//! Example demonstrating integration with the `ansi_term` crate.
use catppuccin::PALETTE;

const fn ansi_term_color(color: &catppuccin::Color) -> ansi_term::Colour {
    ansi_term::Colour::RGB(color.rgb.r, color.rgb.g, color.rgb.b)
}
const fn ansi_term_ansi_color(color: &catppuccin::AnsiColor) -> ansi_term::Colour {
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
                ansi_term_color(color).reverse().paint("  "),
                name,
                color.hex,
                rgb,
                hsl,
            );
        }
        println!();

        println!(
            "{}\n",
            ansi_term::Style::new()
                .underline()
                .bold()
                .paint(format!("{} ANSI", flavor.name))
        );

        for ansi_color in &flavor.ansi_colors {
            let name = format!("{}", ansi_color.name);
            let rgb = format!(
                "rgb({:3}, {:3}, {:3})",
                ansi_color.normal.rgb.r, ansi_color.normal.rgb.g, ansi_color.normal.rgb.b
            );
            let hsl = format!(
                "hsl({:3.0}, {:5.3}, {:5.3})",
                ansi_color.normal.hsl.h, ansi_color.normal.hsl.s, ansi_color.normal.hsl.l
            );
            println!(
                "{} {:18} →  {:6}  {:18}  {:18}",
                ansi_term_ansi_color(&ansi_color.normal).reverse().paint("  "),
                name,
                ansi_color.normal.hex,
                rgb,
                hsl,
            );

            let bright_name = format!("Bright {}", ansi_color.name);
            let bright_rgb = format!(
                "rgb({:3}, {:3}, {:3})",
                ansi_color.bright.rgb.r, ansi_color.bright.rgb.g, ansi_color.bright.rgb.b
            );
            let bright_hsl = format!(
                "hsl({:3.0}, {:5.3}, {:5.3})",
                ansi_color.bright.hsl.h, ansi_color.bright.hsl.s, ansi_color.bright.hsl.l
            );
            println!(
                "{} {:18} →  {:6}  {:18}  {:18}",
                ansi_term_ansi_color(&ansi_color.bright).reverse().paint("  "),
                bright_name,
                ansi_color.bright.hex,
                bright_rgb,
                bright_hsl,
            );
        }

        println!();
    }
}
