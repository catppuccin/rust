use catppuccin::{Rgb, PALETTE};

fn main() {
    let latte_teal = PALETTE.latte.colors.teal;
    let Rgb { r, g, b } = latte_teal.rgb;
    println!(
        "Latte's teal is #{}, which is rgb({r}, {g}, {b})",
        latte_teal.hex
    );

    // You can also take from the full palette of colors:
    let mocha = PALETTE.mocha.colors;
    let Rgb { r, g, b } = mocha.teal.rgb;
    println!(
        "Mocha's teal is #{}, which is rgb({r}, {g}, {b})",
        mocha.teal.hex
    );
}
