//! Simple example showing how to get colors from the Catppuccin palette.
use catppuccin::{ColorName, Rgb, PALETTE};

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
}
