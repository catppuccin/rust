//! Example demonstrating how to make a custom flavor.
//! Two options are provided; setting colors one-by-one, or using a helper macro.
use catppuccin::{Color, Flavor, FlavorColors};

fn americano_simple() -> Flavor {
    let mut oled = catppuccin::PALETTE.mocha;

    oled.colors.base.hex = (0, 0, 0).into();
    oled.colors.base.rgb = (0, 0, 0).into();
    oled.colors.base.hsl = (0.0, 0.0, 0.0).into();

    oled.colors.mantle.hex = (10, 10, 10).into();
    oled.colors.mantle.rgb = (10, 10, 10).into();
    oled.colors.mantle.hsl = (0.0, 0.0, 0.04).into();

    oled.colors.crust.hex = (0, 0, 0).into();
    oled.colors.crust.rgb = (0, 0, 0).into();
    oled.colors.crust.hsl = (0.0, 0.0, 0.08).into();

    oled
}

macro_rules! custom_flavor {
    ($base:expr, $($color_key:ident: $rgb:expr, $hsl:expr,)*) => {
        Flavor {
            colors: FlavorColors {
                $($color_key: Color {
                    hex: $rgb.into(),
                    rgb: $rgb.into(),
                    hsl: $hsl.into(),
                    ..$base.colors.$color_key
                },)*
                ..$base.colors
            },
            ..$base
        }
    };
}

fn use_flavor(flavor: &Flavor) {
    println!("bg: {}", flavor.colors.base.hex);
    println!("bg2: {}", flavor.colors.mantle.hex);
    println!("fg: {}", flavor.colors.text.hex);
    println!("accent: {}", flavor.colors.mauve.hex);
}

fn main() {
    println!("The simple way:");
    let flavor = americano_simple();
    use_flavor(&flavor);
    println!();

    println!("Or with a macro:");
    let flavor = custom_flavor!(catppuccin::PALETTE.mocha,
        base: (0, 0, 0), (0.0, 0.0, 0.0),
        mantle: (10, 10, 10), (0.0, 0.0, 0.04),
        crust: (20, 20, 20), (0.0, 0.0, 0.08),
    );
    use_flavor(&flavor);
}
