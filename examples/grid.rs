use catppuccin::PALETTE;

fn ansi(color: &catppuccin::Color) -> ansi_term::Colour {
    ansi_term::Colour::RGB(color.rgb[0], color.rgb[1], color.rgb[2])
}

fn main() {
    for flavor in PALETTE.iter() {
        let heading = format!(
            "{} ({})",
            flavor.name,
            if flavor.dark { "dark" } else { "light" }
        );
        println!(
            "{}\n",
            ansi_term::Style::new().underline().bold().paint(heading)
        );

        for color in flavor.iter() {
            let name = format!(
                "{}{}",
                color.name,
                if color.is_accent { " (accent)" } else { "" }
            );
            let rgb = format!("rgb({}, {}, {})", color.rgb[0], color.rgb[1], color.rgb[2]);
            let hsl = format!("hsl({}, {}, {})", color.hsl[0], color.hsl[1], color.hsl[2]);
            println!(
                "{} {:18} →  #{:6}  {:18}  {:18}",
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
