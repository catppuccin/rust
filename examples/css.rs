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

    let ansi_normal_magenta = catppuccin::PALETTE.mocha.ansi_colors.magenta;
    let ansi_bright_magenta = catppuccin::PALETTE.mocha.ansi_colors.bright_magenta;
    let ansi_magenta_normal_rgb: css_colors::RGB = ansi_normal_magenta.into();
    let ansi_magenta_bright_rgb: css_colors::RGB = ansi_bright_magenta.into();

    println!("ANSI Magenta RGB: {}", ansi_magenta_normal_rgb.to_css());
    println!(
        "ANSI Bright Magenta RGB: {}",
        ansi_magenta_bright_rgb.to_css()
    );
}
