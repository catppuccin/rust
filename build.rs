//! Build script for the Catppuccin crate.
//! This script uses the palette JSON file from the catppuccin/palette github repo
//! in order to populate the `FlavorColors` struct as well as implement the various
//! iteration & indexing primitives offered by the crate.
use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use indexmap::IndexMap;
use serde::Deserialize;

const PALETTE_VERSION: &str = "v1.0.3";

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
    hex: String,
    rgb: Rgb,
    hsl: Hsl,
    accent: bool,
}

#[derive(Debug, Deserialize)]
struct Flavor {
    name: String,
    dark: bool,
    colors: IndexMap<String, Color>,
}

type Palette = IndexMap<String, Flavor>;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(&env::var("OUT_DIR")?);
    let palette_path = out_dir.join(PALETTE_VERSION).join("palette.json");

    if !palette_path.exists() {
        println!("cargo:warning=no palette json in cache, fetching...");
        download_palette(&palette_path)?;
    }

    let palette: Palette = serde_json::from_reader(BufReader::new(File::open(palette_path)?))?;

    let codegen_path = out_dir.join("generated_palette.rs");
    let codegen_file = BufWriter::new(File::create(codegen_path)?);
    let mut code_writer = BufWriter::new(codegen_file);

    let sample_flavor = palette.values().next().expect("at least one flavor");
    make_flavorcolors_struct(&mut code_writer, &palette)?;
    make_flavorcolors_all_impl(&mut code_writer, sample_flavor)?;
    make_colorname_enum(&mut code_writer, &palette)?;
    make_colorname_index_impl(&mut code_writer, sample_flavor)?;
    make_palette_const(&mut code_writer, &palette)?;

    Ok(())
}

fn download_palette(path: &Path) -> Result<(), Box<dyn Error>> {
    let palette_url = format!(
        "https://raw.githubusercontent.com/catppuccin/palette/{PALETTE_VERSION}/palette.json"
    );
    let contents = ureq::get(&palette_url).call()?.into_string()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}

#[allow(clippy::cast_possible_truncation)]
fn as_percentage(v: f64) -> i32 {
    assert!((0.0..=1.0).contains(&v));
    (v * 100.0) as i32
}

fn color_div(color: &Color) -> String {
    let Hsl { h, s, l } = color.hsl;
    let s = as_percentage(s);
    let l = as_percentage(l);
    format!(
        "<div style=\"display: inline-block; background-color:hsl({h} {s}% {l}%); \
        width: 10px; padding: 10px; margin: 0 2px 0; \
        border: 1px solid #1e1e2e; border-radius: 10px\"></div>"
    )
}

fn color_divs(color_name: &str, palette: &Palette) -> String {
    palette
        .values()
        .map(|flavour| &flavour.colors[color_name])
        .map(color_div)
        .collect::<String>()
}

fn make_flavorcolors_struct<W: Write>(w: &mut W, palette: &Palette) -> Result<(), Box<dyn Error>> {
    let fields = palette
        .values()
        .next()
        .expect("at least one flavor")
        .colors
        .keys()
        .map(|name| format!("/// {}\n    pub {name}: Color,", color_divs(name, palette)))
        .collect::<Vec<_>>();
    writeln!(
        w,
        r#"/// All of the colors for a particular flavor of Catppuccin.
/// Obtained via [`Flavor::colors`].
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
    let items = sample_flavor
        .colors
        .keys()
        .map(|name| format!("&self.{name},"))
        .collect::<Vec<_>>();
    writeln!(
        w,
        "impl FlavorColors {{
    /// Get an array of the colors in the flavor.
    #[must_use]
    pub const fn all_colors(&'static self) -> [&'static Color; 26] {{
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
    let fields = palette
        .values()
        .next()
        .expect("at least one flavor")
        .colors
        .keys()
        .map(|name| {
            format!(
                "/// {}\n    {},",
                color_divs(name, palette),
                titlecase(name)
            )
        })
        .collect::<Vec<_>>();
    writeln!(
        w,
        "/// Enum of all named Catppuccin colors. Can be used to index into a [`FlavorColors`].
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
    let match_arms = sample_flavor
        .colors
        .keys()
        .map(|name| format!("ColorName::{} => &self.{name},", titlecase(name)))
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
    for (flavor_name, flavor) in palette {
        write!(w, "    {flavor_name}: ")?;
        make_flavor_entry(w, flavor)?;
    }
    writeln!(w, "}};")?;
    Ok(())
}

fn make_flavor_entry<W: Write>(w: &mut W, flavor: &Flavor) -> Result<(), Box<dyn Error>> {
    writeln!(
        w,
        "Flavor {{
        name: {:?},
        dark: {:?},
        colors: FlavorColors {{",
        flavor.name, flavor.dark
    )?;
    for (color_name, color) in &flavor.colors {
        write!(w, "            {color_name}: ")?;
        make_color_entry(w, color, color_name)?;
    }
    writeln!(w, "        }},\n    }},")?;
    Ok(())
}

fn make_color_entry<W: Write>(w: &mut W, color: &Color, name: &str) -> Result<(), Box<dyn Error>> {
    writeln!(
        w,
        r#"Color {{
                name: {:?},
                accent: {:?},
                hex: {:?},
                rgb: Rgb {{ r: {:?}, g: {:?}, b: {:?} }},
                hsl: Hsl {{ h: {:?}, s: {:?}, l: {:?} }},
            }},"#,
        name,
        color.accent,
        color.hex,
        color.rgb.r,
        color.rgb.g,
        color.rgb.b,
        color.hsl.h,
        color.hsl.s,
        color.hsl.l,
    )?;
    Ok(())
}
