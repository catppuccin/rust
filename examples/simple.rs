//! Simple example showing how to get colors from the Catppuccin palette.
use catppuccin::{AnsiColor, ColorName, Rgb, PALETTE};

fn main() {
    let latte_teal = PALETTE.latte.colors.teal;
    let Rgb { r, g, b } = latte_teal.rgb;
    println!(
        "Latte's {} is {}, which is rgb({r}, {g}, {b})",
        latte_teal.name, latte_teal.hex
    );

    // you can also get a color by its name, from `FlavorColors` or `Flavor`:
    let mocha = &PALETTE.mocha;
    let mocha_teal = mocha.colors[ColorName::Teal];
    let mocha_mauve = mocha[ColorName::Mauve];

    let Rgb { r, g, b } = mocha_teal.rgb;
    println!(
        "Mocha's {} is {}, which is rgb({r}, {g}, {b})",
        mocha_teal.name, mocha_teal.hex
    );

    println!("Mocha's {} is {}", mocha_mauve.name, mocha_mauve.hex);

    for AnsiColor {
        name,
        rgb,
        hsl,
        code,
        hex,
    } in &mocha.ansi_colors
    {
        {
            println!(
                "Mocha ANSI [{:2}] {:15} →  {:6}  {:3?}  {:19?}",
                code,
                name.to_string(),
                hex,
                rgb,
                hsl,
            );
        }
    }

    println!();

    for mocha_ansi_color_pairs in &mocha.ansi_colors.all_pairs() {
        for AnsiColor {
            name,
            rgb,
            hsl,
            code,
            hex,
        } in [mocha_ansi_color_pairs.normal, mocha_ansi_color_pairs.bright]
        {
            println!(
                "Mocha ANSI [{:2}] {:15} →  {:6}  {:3?}  {:19?}",
                code,
                name.to_string(),
                hex,
                rgb,
                hsl,
            );
        }
    }
}
