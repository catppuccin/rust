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
    println!();

    // iterate over the 16 ANSI colors (i.e. Black, Red, ..., Bright Black, Bright Red, ...)
    println!("Mocha's ANSI colors in code order:");
    for AnsiColor {
        name,
        rgb,
        hsl,
        code,
        hex,
    } in &mocha.ansi_colors
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
    println!();

    // iterate over the 16 ANSI colors in 8 pairs (i.e. Black, Bright Black, Red, Bright Red, ...)
    println!("Mocha's ANSI color pairs:");
    for pair in &mocha.ansi_colors.all_pairs() {
        println!(
            "[{:2}] {:7} / [{:2}] {}",
            pair.normal.code,
            pair.normal.name.to_string(),
            pair.bright.code,
            pair.bright.name
        );
    }
}
