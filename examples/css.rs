use css_colors::{percent, Angle, Color, HSL};

fn hsl(color: &catppuccin::Color) -> HSL {
    css_colors::hsl(
        color.hsl[0] as i32,
        (color.hsl[1] * 100.0) as u8,
        (color.hsl[2] * 100.0) as u8,
    )
}

fn main() {
    let mocha_teal = hsl(&catppuccin::PALETTE.flavors["mocha"].colors["teal"]);
    println!("Mocha Teal is {} / {}", mocha_teal, mocha_teal.to_rgb());
    println!("20% lighter: {}", mocha_teal.lighten(percent(20)));
    println!("10° rotated: {}", mocha_teal.spin(Angle::new(10)));
}
