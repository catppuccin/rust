//! Simple example showing how to get colors from the Catppuccin palette.
use catppuccin::{ColorName, Rgb, PALETTE};

fn main() {
    let latte_teal = PALETTE.latte.colors.teal;
    let Rgb { r, g, b } = latte_teal.rgb;
    println!(
        "Latte's teal is {}, which is rgb({r}, {g}, {b})",
        latte_teal.hex
    );

    // you can also get a color by its name, from `FlavorColors` or `Flavor`:
    let mocha_teal = PALETTE.mocha.colors[ColorName::Teal];
    let mocha_mauve = PALETTE.mocha[ColorName::Mauve];

    let Rgb { r, g, b } = mocha_teal.rgb;
    println!(
        "Mocha's teal is {}, which is rgb({r}, {g}, {b})",
        mocha_teal.hex
    );

    println!("Mocha's mauve is {}", mocha_mauve.hex);
}
