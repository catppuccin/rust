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
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;

// the palette json currently has no order field for ANSI colors.
const ANSI_COLOR_ORDER: [&str; 8] = [
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
];

#[derive(Debug, Deserialize)]
struct Palette {
    #[allow(dead_code)]
    version: String,
    #[serde(flatten)]
    flavors: HashMap<String, Flavor>,
}

#[derive(Debug, Deserialize)]
struct Flavor {
    emoji: char,
    order: u32,
    dark: bool,
    colors: HashMap<String, Color>,
    #[serde(rename = "ansiColors")]
    ansi_colors: HashMap<String, AnsiColorPair>,
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
struct AnsiColorPair {
    normal: AnsiColor,
    bright: AnsiColor,
}

#[derive(Debug, Deserialize)]
struct AnsiColor {
    hex: String,
    code: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(&env::var("OUT_DIR")?);
    let codegen_path = out_dir.join("generated_palette.rs");
    let mut code_writer = BufWriter::new(File::create(codegen_path)?);

    let palette: Palette =
        serde_json::from_reader(BufReader::new(File::open("src/palette.json")?))?;
    let sample_flavor = palette
        .flavors
        .values()
        .next()
        .expect("at least one flavor");

    let tokens = [
        make_flavor_colors_struct(sample_flavor),
        make_flavor_colors_all_impl(sample_flavor),
        make_flavor_ansi_colors_struct(sample_flavor),
        make_color_name_enum(sample_flavor),
        make_color_name_index_impl(sample_flavor),
        make_color_name_display_impl(sample_flavor),
        make_color_name_identifier_impl(sample_flavor),
        make_color_name_fromstr_impl_tokens(sample_flavor),
        make_palette_const(&palette),
    ];
    let ast = syn::parse2(tokens.into_iter().collect())?;
    let code = prettyplease::unparse(&ast);
    write!(&mut code_writer, "{code}")?;

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

fn titlecase<S: AsRef<str>>(s: S) -> String {
    let mut chars = s.as_ref().chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().to_string() + chars.as_str()
    })
}

fn flavors_in_order(palette: &Palette) -> std::vec::IntoIter<(&String, &Flavor)> {
    palette
        .flavors
        .iter()
        .sorted_by(|(_, a), (_, b)| a.order.cmp(&b.order))
}

fn colors_in_order(flavor: &Flavor) -> std::vec::IntoIter<(&String, &Color)> {
    flavor
        .colors
        .iter()
        .sorted_by(|(_, a), (_, b)| a.order.cmp(&b.order))
}

fn ansi_colors_in_order(flavor: &Flavor) -> std::vec::IntoIter<(&str, &AnsiColorPair)> {
    ANSI_COLOR_ORDER
        .iter()
        .map(|key| (*key, &flavor.ansi_colors[*key]))
        .collect::<Vec<_>>()
        .into_iter()
}

fn make_flavor_colors_struct(sample_flavor: &Flavor) -> TokenStream {
    let colors = colors_in_order(sample_flavor).map(|(k, _)| {
        let ident = format_ident!("{k}");
        let color_img = format!(" {}", color_imgs(k));
        quote! {
            #[doc = #color_img]
            pub #ident: Color
        }
    });
    quote! {
        /// All of the colors for a particular flavor of Catppuccin.
        /// Obtained via [`Flavor::colors`].
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct FlavorColors {
            #(#colors),*
        }
    }
}

fn make_flavor_colors_all_impl(sample_flavor: &Flavor) -> TokenStream {
    let items = colors_in_order(sample_flavor).map(|(identifier, _)| {
        let ident = format_ident!("{identifier}");
        quote! { &self.#ident }
    });
    quote! {
        impl FlavorColors {
            /// Get an array of the colors in the flavor.
            #[must_use]
            pub const fn all_colors(&self) -> [&Color; 26] {
                [
                    #(#items),*
                ]
            }
        }
    }
}

fn make_flavor_ansi_colors_struct(sample_flavor: &Flavor) -> TokenStream {
    let colors = ansi_colors_in_order(sample_flavor).map(|(k, _)| {
        let ident = format_ident!("{k}");
        quote! {
            /// The normal and bright #ident ANSI color pair.
            pub #ident: AnsiColorPair
        }
    });
    quote! {
        /// All of the ANSI colors for a particular flavor of Catppuccin.
        /// Obtained via [`Flavor::ansi_colors`].
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct FlavorAnsiColors {
            #(#colors),*
        }

        /// A pair of ANSI colors - normal and bright.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct AnsiColorPair {
            /// The normal color.
            pub normal: AnsiColor,
            /// The bright color.
            pub bright: AnsiColor,
        }

        /// A single ANSI color.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct AnsiColor {
            /// The color represented as a six-digit hex string with a leading hash (#).
            pub hex: Hex,
            /// The color's ANSI code.
            pub code: u8,
        }
    }
}

fn make_color_name_enum(sample_flavor: &Flavor) -> TokenStream {
    let variants = colors_in_order(sample_flavor).map(|(name, _)| {
        let ident = format_ident!("{}", titlecase(name));
        let color_imgs = format!(" {}", color_imgs(name));
        quote! {
            #[doc = #color_imgs]
            #ident
        }
    });
    quote! {
        /// Enum of all named Catppuccin colors. Can be used to index into a [`FlavorColors`].
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum ColorName {
            #(#variants),*
        }
    }
}

fn make_color_name_index_impl(sample_flavor: &Flavor) -> TokenStream {
    let first = colors_in_order(sample_flavor).map(|(identifier, _)| {
        let variant = format_ident!("{}", titlecase(identifier));
        let ident = format_ident!("{}", identifier);
        quote! {
            ColorName::#variant => &self.#ident
        }
    });
    let second = first.clone();
    quote! {
        impl Index<ColorName> for FlavorColors {
            type Output = Color;

            fn index(&self, index: ColorName) -> &Self::Output {
                match index {
                    #(#first),*
                }
            }
        }

        impl FlavorColors {
            /// Get a color by name.
            ///
            /// This is equivalent to using the index operator, but can also be used in
            /// const contexts.
            #[must_use]
            pub const fn get_color(&self, name: ColorName) -> &Color {
                match name {
                    #(#second),*
                }
            }
        }
    }
}

fn make_color_name_display_impl(sample_flavor: &Flavor) -> TokenStream {
    let match_arms = colors_in_order(sample_flavor).map(|(identifier, color)| {
        let variant = format_ident!("{}", titlecase(identifier));
        let name = &color.name;
        quote! {
            Self::#variant => write!(f, #name)
        }
    });
    quote! {
        impl std::fmt::Display for ColorName {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}

fn make_color_name_identifier_impl(sample_flavor: &Flavor) -> TokenStream {
    let match_arms = colors_in_order(sample_flavor).map(|(identifier, _)| {
        let variant = format_ident!("{}", titlecase(identifier));
        quote! {
            Self::#variant => #identifier
        }
    });
    quote! {
        impl ColorName {
            /// Get the color's identifier; the lowercase key used to identify the color.
            /// This differs from `to_string` in that it's intended for machine usage
            /// rather than presentation.
            ///
            /// Example:
            ///
            /// ```rust
            /// let surface0 = catppuccin::PALETTE.latte.colors.surface0;
            /// assert_eq!(surface0.name.to_string(), "Surface 0");
            /// assert_eq!(surface0.name.identifier(), "surface0");
            /// ```
            #[must_use]
            pub const fn identifier(&self) -> &'static str {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}

fn make_color_name_fromstr_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let match_arms = colors_in_order(sample_flavor)
        .map(|(identifier, _)| {
            let variant = format_ident!("{}", titlecase(identifier));
            quote! {
                #identifier => Ok(Self::#variant)
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl std::str::FromStr for ColorName {
            type Err = ParseColorNameError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#match_arms),*,
                    _ => Err(ParseColorNameError),
                }
            }
        }
    }
}

fn make_palette_const(palette: &Palette) -> TokenStream {
    let flavors =
        flavors_in_order(palette).map(|(identifier, flavor)| make_flavor_entry(identifier, flavor));
    let tokens = quote! {
        /// The Catppuccin palette. This constant will generally be your entrypoint
        /// into using the crate.
        #[allow(clippy::unreadable_literal)]
        pub const PALETTE: Palette = Palette {
            #(#flavors),*
        };
    };

    tokens
}

fn make_flavor_entry(identifier: &str, flavor: &Flavor) -> TokenStream {
    let Flavor {
        emoji, order, dark, ..
    } = flavor;
    let colors =
        colors_in_order(flavor).map(|(identifier, color)| make_color_entry(identifier, color));
    let ansi_colors = ansi_colors_in_order(flavor)
        .map(|(identifier, ansi_color_pair)| make_ansi_color_entry(identifier, ansi_color_pair));
    let flavorname_variant = format_ident!("{}", titlecase(identifier));
    let ident = format_ident!("{}", identifier);
    quote! {
        #ident: Flavor {
            name: FlavorName::#flavorname_variant,
            emoji: #emoji,
            order: #order,
            dark: #dark,
            colors: FlavorColors {
                #(#colors),*
            },
            ansi_colors: FlavorAnsiColors {
                #(#ansi_colors),*
            }
        }
    }
}

fn make_color_entry(identifier: &str, color: &Color) -> TokenStream {
    let ident = format_ident!("{}", identifier);
    let colorname_variant = format_ident!("{}", titlecase(identifier));
    let Color {
        order,
        accent,
        rgb: Rgb { r, g, b },
        hsl: Hsl { h, s, l },
        ..
    } = color;
    let rgb = quote! { Rgb { r: #r, g: #g, b: #b } };
    let hsl = quote! { Hsl { h: #h, s: #s, l: #l } };
    quote! {
        #ident: Color {
            name: ColorName::#colorname_variant,
            order: #order,
            accent: #accent,
            hex: Hex(#rgb),
            rgb: #rgb,
            hsl: #hsl,
        }
    }
}

fn make_ansi_color_entry(identifier: &str, ansi_color_pair: &AnsiColorPair) -> TokenStream {
    let ident = format_ident!("{}", identifier);
    let AnsiColorPair {
        normal: AnsiColor {
            hex: normal_hex,
            code: normal_code,
        },
        bright: AnsiColor {
            hex: bright_hex,
            code: bright_code,
        },
    } = ansi_color_pair;

    // we don't get RGB from the palette json for ansi colours, so we have to
    // parse the hex string into an RGB and then wrap that in Hex(...)
    let (normal_r, normal_g, normal_b) = parse_hex_string(normal_hex);
    let (bright_r, bright_g, bright_b) = parse_hex_string(bright_hex);

    let normal_hex = quote! { Hex(Rgb { r: #normal_r, g: #normal_g, b: #normal_b }) };
    let bright_hex = quote! { Hex(Rgb { r: #bright_r, g: #bright_g, b: #bright_b }) };

    quote! {
        #ident: AnsiColorPair {
            normal: AnsiColor {
                hex: #normal_hex,
                code: #normal_code,
            },
            bright: AnsiColor {
                hex: #bright_hex,
                code: #bright_code,
            }
        }
    }
}

fn parse_hex_string(hex: &str) -> (u8, u8, u8) {
    let hex = hex
        .strip_prefix('#')
        .expect("hex string should start with #");
    let hex = u32::from_str_radix(hex, 16).expect("hex string should be valid hexadecimal");
    (
        ((hex >> 16) & 0xff) as u8,
        ((hex >> 8) & 0xff) as u8,
        (hex & 0xff) as u8,
    )
}
