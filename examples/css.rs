//! Example demonstrating integration with the `css-colors` crate.
use css_colors::{percent, Color};

fn main() {
    let teal = catppuccin::PALETTE.mocha.colors.teal;
    let rgb: css_colors::RGB = teal.into();

    println!("RGB: {}", rgb.to_css());

    let hsl = rgb.to_hsl();
    println!("HSL: {}", hsl.to_css());

    let lighter = hsl.lighten(percent(20));
    println!("20% lighter: {lighter}");
}
