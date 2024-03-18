//! Build script for the Catppuccin crate.
//! This script uses the palette JSON file from the catppuccin/palette github repo
//! in order to populate the `FlavorColors` struct as well as implement the various
//! iteration & indexing primitives offered by the crate.
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};

use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Deserialize)]
struct Hsl {
    h: f64,
    s: f64,
    l: f64,
}

#[derive(Debug, Deserialize)]
struct Color {
    name: String,
    order: u32,
    rgb: Rgb,
    hsl: Hsl,
    accent: bool,
}

#[derive(Debug, Deserialize)]
struct Flavor {
    dark: bool,
    colors: HashMap<String, Color>,
}

type Palette = HashMap<String, Flavor>;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(&env::var("OUT_DIR")?);
    let codegen_path = out_dir.join("generated_palette.rs");
    let mut code_writer = BufWriter::new(File::create(codegen_path)?);

    let palette: Palette =
        serde_json::from_reader(BufReader::new(File::open("src/palette.json")?))?;
    let sample_flavor = palette.values().next().expect("at least one flavor");

    make_flavorcolors_struct(&mut code_writer, &palette)?;
    make_flavorcolors_all_impl(&mut code_writer, sample_flavor)?;
    make_colorname_enum(&mut code_writer, &palette)?;
    make_colorname_index_impl(&mut code_writer, sample_flavor)?;
    make_colorname_display_impl(&mut code_writer, sample_flavor)?;
    make_colorname_identifier_impl(&mut code_writer, sample_flavor)?;
    make_colorname_fromstr_impl(&mut code_writer, sample_flavor)?;
    make_palette_const(&mut code_writer, &palette)?;

    Ok(())
}

fn color_img(flavor_key: &str, color_key: &str) -> String {
    format!(
        r#"<img width="23" height="23" src="https://github.com/catppuccin/catppuccin/raw/main/assets/palette/circles/{flavor_key}_{color_key}.png">"#
    )
}

fn color_imgs(color_key: &str) -> String {
    [
        color_img("latte", color_key),
        color_img("frappe", color_key),
        color_img("macchiato", color_key),
        color_img("mocha", color_key),
    ]
    .into_iter()
    .collect::<String>()
}

fn colors_in_order(flavor: &Flavor) -> std::vec::IntoIter<(&String, &Color)> {
    flavor
        .colors
        .iter()
        .sorted_by(|(_, a), (_, b)| a.order.cmp(&b.order))
}

fn make_flavorcolors_struct<W: Write>(w: &mut W, palette: &Palette) -> Result<(), Box<dyn Error>> {
    let sample_flavor = palette.values().next().expect("at least one flavor");
    let fields = colors_in_order(sample_flavor)
        .map(|(name, _)| format!("/// {}\n    pub {name}: Color,", color_imgs(name)))
        .collect::<Vec<_>>();
    writeln!(
        w,
        r#"/// All of the colors for a particular flavor of Catppuccin.
/// Obtained via [`Flavor::colors`].
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlavorColors {{
    {}
}}"#,
        fields.join("\n    ")
    )?;
    Ok(())
}

fn make_flavorcolors_all_impl<W: Write>(
    w: &mut W,
    sample_flavor: &Flavor,
) -> Result<(), Box<dyn Error>> {
    let items = colors_in_order(sample_flavor)
        .map(|(name, _)| format!("&self.{name},"))
        .collect::<Vec<_>>();
    writeln!(
        w,
        "impl FlavorColors {{
    /// Get an array of the colors in the flavor.
    #[must_use]
    pub const fn all_colors(&self) -> [&Color; 26] {{
        [
            {}
        ]
    }}
}}",
        items.join("\n            ")
    )?;
    Ok(())
}

fn titlecase<S: AsRef<str>>(s: S) -> String {
    let mut chars = s.as_ref().chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().to_string() + chars.as_str()
    })
}

fn make_colorname_enum<W: Write>(w: &mut W, palette: &Palette) -> Result<(), Box<dyn Error>> {
    let sample_flavor = palette.values().next().expect("at least one flavor");
    let fields = colors_in_order(sample_flavor)
        .map(|(name, _)| format!("/// {}\n    {},", color_imgs(name), titlecase(name)))
        .collect::<Vec<_>>();
    writeln!(
        w,
        "/// Enum of all named Catppuccin colors. Can be used to index into a [`FlavorColors`].
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]
pub enum ColorName {{
    {}
}}",
        fields.join("\n    ")
    )?;
    Ok(())
}

fn make_colorname_index_impl<W: Write>(
    w: &mut W,
    sample_flavor: &Flavor,
) -> Result<(), Box<dyn Error>> {
    let match_arms = colors_in_order(sample_flavor)
        .map(|(name, _)| format!("ColorName::{} => &self.{name},", titlecase(name)))
        .collect::<Vec<_>>();
    writeln!(
        w,
        "impl Index<ColorName> for FlavorColors {{
    type Output = Color;

    fn index(&self, index: ColorName) -> &Self::Output {{
        match index {{
            {}
        }}
    }}
}}

impl FlavorColors {{
    /// Get a color by name.
    ///
    /// This is equivalent to using the index operator, but can also be used in
    /// const contexts.
    #[must_use]
    pub const fn get_color(&self, name: ColorName) -> &Color {{
        match name {{
            {}
        }}
    }}
}}",
        match_arms.join("\n            "),
        match_arms.join("\n            "),
    )?;
    Ok(())
}

fn make_colorname_display_impl<W: Write>(
    w: &mut W,
    sample_flavor: &Flavor,
) -> Result<(), Box<dyn Error>> {
    let match_arms = sample_flavor
        .colors
        .iter()
        .map(|(name, color)| format!("Self::{} => write!(f, {:?}),", titlecase(name), color.name))
        .collect::<Vec<_>>();
    writeln!(
        w,
        "impl std::fmt::Display for ColorName {{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
        match self {{
            {}
        }}
    }}
}}",
        match_arms.join("\n            ")
    )?;
    Ok(())
}

fn make_colorname_identifier_impl<W: Write>(
    w: &mut W,
    sample_flavor: &Flavor,
) -> Result<(), Box<dyn Error>> {
    let match_arms = sample_flavor
        .colors
        .keys()
        .map(|name| format!("Self::{} => {:?},", titlecase(name), name))
        .collect::<Vec<_>>();
    writeln!(
        w,
        "impl ColorName {{
    /// Get the color's identifier; the lowercase key used to identify the color.
    /// This differs from `to_string` in that it's intended for machine usage
    /// rather than presentation.
    ///
    /// Example:
    ///
    /// ```rust
    /// let surface0 = catppuccin::PALETTE.latte.colors.surface0;
    /// assert_eq!(surface0.name.to_string(), \"Surface 0\");
    /// assert_eq!(surface0.name.identifier(), \"surface0\");
    /// ```
    #[must_use]
    pub const fn identifier(&self) -> &'static str {{
        match self {{
            {}
        }}
    }}
}}",
        match_arms.join("\n            ")
    )?;
    Ok(())
}

fn make_colorname_fromstr_impl<W: Write>(
    w: &mut W,
    sample_flavor: &Flavor,
) -> Result<(), Box<dyn Error>> {
    let match_arms = sample_flavor
        .colors
        .keys()
        .map(|name| format!("{:?} => Ok(Self::{}),", name, titlecase(name)))
        .collect::<Vec<_>>();
    writeln!(
        w,
        "impl std::str::FromStr for ColorName {{
    type Err = ParseColorNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s {{
            {}
            _ => Err(ParseColorNameError),
        }}
    }}
}}",
        match_arms.join("\n            ")
    )?;
    Ok(())
}

fn make_palette_const<W: Write>(w: &mut W, palette: &Palette) -> Result<(), Box<dyn Error>> {
    writeln!(
        w,
        "/// The Catppuccin palette. This constant will generally be your entrypoint
/// into using the crate.
#[allow(clippy::unreadable_literal)]
pub const PALETTE: Palette = Palette {{"
    )?;
    for (flavor_key, flavor) in palette {
        write!(w, "    {flavor_key}: ")?;
        make_flavor_entry(w, flavor_key, flavor)?;
    }
    writeln!(w, "}};")?;
    Ok(())
}

fn make_flavor_entry<W: Write>(
    w: &mut W,
    flavor_key: &str,
    flavor: &Flavor,
) -> Result<(), Box<dyn Error>> {
    writeln!(
        w,
        "Flavor {{
        name: FlavorName::{},
        dark: {:?},
        colors: FlavorColors {{",
        titlecase(flavor_key),
        flavor.dark
    )?;
    for (color_key, color) in &flavor.colors {
        write!(w, "            {color_key}: ")?;
        make_color_entry(w, color, color_key)?;
    }
    writeln!(w, "        }},\n    }},")?;
    Ok(())
}

fn make_color_entry<W: Write>(w: &mut W, color: &Color, name: &str) -> Result<(), Box<dyn Error>> {
    writeln!(
        w,
        r#"Color {{
                name: ColorName::{},
                accent: {:?},
                hex: Hex(Rgb {{ r: {:?}, g: {:?}, b: {:?} }}),
                rgb: Rgb {{ r: {:?}, g: {:?}, b: {:?} }},
                hsl: Hsl {{ h: {:?}, s: {:?}, l: {:?} }},
            }},"#,
        titlecase(name),
        color.accent,
        color.rgb.r,
        color.rgb.g,
        color.rgb.b,
        color.rgb.r,
        color.rgb.g,
        color.rgb.b,
        color.hsl.h,
        color.hsl.s,
        color.hsl.l,
    )?;
    Ok(())
}
