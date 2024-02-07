use catppuccin::Flavor;

fn americano() -> Flavor {
    let mut oled = catppuccin::PALETTE.mocha;
    oled.colors.base.hex = "#000000";
    oled.colors.base.rgb = (0, 0, 0).into();
    oled.colors.base.hsl = (0.0, 0.0, 0.0).into();
    oled
}

fn use_flavor(flavor: &Flavor) {
    println!("bg: {}", flavor.colors.base.hex);
    println!("fg: {}", flavor.colors.text.hex);
    println!("accent: {}", flavor.colors.mauve.hex);
}

fn main() {
    let flavor = americano();
    use_flavor(&flavor);
}
