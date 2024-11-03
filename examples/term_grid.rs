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

        for ansi_color_pair in &flavor.ansi_colors {
            for ansi_color in [ansi_color_pair.normal, ansi_color_pair.bright] {
                let rgb = format!(
                    "rgb({:3}, {:3}, {:3})",
                    ansi_color.rgb.r, ansi_color.rgb.g, ansi_color.rgb.b
                );
                let hsl = format!(
                    "hsl({:3.0}, {:5.3}, {:5.3})",
                    ansi_color.hsl.h, ansi_color.hsl.s, ansi_color.hsl.l
                );

                println!(
                    "{} {:18} →  {:6}  {:18}  {:18}",
                    ansi_term_ansi_color(&ansi_color).reverse().paint("  "),
                    ansi_color.name.to_string(),
                    ansi_color.hex,
                    rgb,
                    hsl,
                );
            }
        }

        println!();
    }
}
