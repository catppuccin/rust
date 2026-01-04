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
    name: String,
    order: u32,
    normal: AnsiColor,
    bright: AnsiColor,
}

#[derive(Debug, Deserialize)]
struct AnsiColor {
    name: String,
    rgb: Rgb,
    hsl: Hsl,
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

    let flavor_tokens = [
        // Colors
        make_flavor_colors_struct_tokens(sample_flavor),
        make_flavor_colors_all_impl_tokens(sample_flavor),
        // ANSI Colors
        make_flavor_ansi_colors_struct_tokens(sample_flavor),
        make_flavor_ansi_colors_all_impl_tokens(sample_flavor),
        // ANSI Color Pairs
        make_flavor_ansi_color_pairs_struct_tokens(sample_flavor),
        make_flavor_ansi_color_pairs_all_impl_tokens(sample_flavor),
    ];
    let color_tokens = [
        make_color_name_enum_tokens(sample_flavor),
        make_color_name_index_impl_tokens(sample_flavor),
        make_color_name_display_impl_tokens(sample_flavor),
        make_color_name_identifier_impl_tokens(sample_flavor),
        make_color_name_fromstr_impl_tokens(sample_flavor),
    ];
    let ansi_color_tokens = [
        make_ansi_color_name_enum_tokens(sample_flavor),
        make_ansi_color_name_index_impl_tokens(sample_flavor),
        make_ansi_color_name_display_impl_tokens(sample_flavor),
        make_ansi_color_name_identifier_impl_tokens(sample_flavor),
        make_ansi_color_name_fromstr_impl_tokens(sample_flavor),
    ];
    let ansi_color_pair_tokens = [
        make_ansi_color_pair_name_enum_tokens(sample_flavor),
        make_ansi_color_pair_name_index_impl_tokens(sample_flavor),
        make_ansi_color_pair_name_display_impl_tokens(sample_flavor),
        make_ansi_color_pair_name_identifier_impl_tokens(sample_flavor),
        make_ansi_color_pair_name_fromstr_impl_tokens(sample_flavor),
    ];
    let palette_tokens = [make_palette_const_tokens(&palette)];

    let ast = syn::parse2(
        [
            &flavor_tokens[..],
            &color_tokens[..],
            &ansi_color_tokens[..],
            &ansi_color_pair_tokens[..],
            &palette_tokens[..],
        ]
        .concat()
        .into_iter()
        .collect(),
    )?;
    let code = prettyplease::unparse(&ast);
    write!(&mut code_writer, "{code}")?;

    Ok(())
}

fn palette_circle(filename: &str) -> String {
    format!(
        r#"<img width="23" height="23" src="https://raw.githubusercontent.com/catppuccin/catppuccin/95aae3360eb88fc1b6a89398be08ec6deae0bc9a/assets/palette/circles/{filename}.png">"#
    )
}

fn color_palette_circles(color_key: &str) -> String {
    ["latte", "frappe", "macchiato", "mocha"]
        .map(|n| palette_circle(format!("{n}_{color_key}").as_str()))
        .into_iter()
        .collect::<String>()
}

fn ansi_color_palette_circles(color_key: &str) -> String {
    ["latte", "frappe", "macchiato", "mocha"]
        .map(|n| palette_circle(format!("ansi/{n}_ansi_{color_key}").as_str()))
        .into_iter()
        .collect::<String>()
}

fn titlecase<S: AsRef<str>>(s: S) -> String {
    let mut chars = s.as_ref().chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().to_string() + chars.as_str()
    })
}

fn remove_whitespace(s: &str) -> String {
    s.replace(' ', "")
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

fn ansi_color_pairs_in_order(
    flavor: &Flavor,
) -> std::vec::IntoIter<(&String, &AnsiColorPair, String, String)> {
    flavor
        .ansi_colors
        .iter()
        .map(|(ident, pair)| {
            (
                ident,
                pair,
                pair.normal.name.to_lowercase().replace(' ', "_"),
                pair.bright.name.to_lowercase().replace(' ', "_"),
            )
        })
        .sorted_by(|(_, a, _, _), (_, b, _, _)| a.order.cmp(&b.order))
}

fn ansi_colors_in_order(flavor: &Flavor) -> std::vec::IntoIter<(String, &AnsiColor)> {
    flavor
        .ansi_colors
        .iter()
        .flat_map(|(_, c)| [&c.normal, &c.bright])
        .map(|c| (c.name.to_lowercase().replace(' ', "_"), c))
        .sorted_by(|(_, a), (_, b)| a.code.cmp(&b.code))
}

fn make_flavor_colors_struct_tokens(sample_flavor: &Flavor) -> TokenStream {
    let colors = colors_in_order(sample_flavor).map(|(k, _)| {
        let ident = format_ident!("{k}");
        let color_img = format!(" {}", color_palette_circles(k));
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

fn make_flavor_ansi_colors_struct_tokens(sample_flavor: &Flavor) -> TokenStream {
    let colors = ansi_colors_in_order(sample_flavor).map(|(k, _)| {
        let ident = format_ident!("{k}");
        let color_img = format!(" {}", ansi_color_palette_circles(&k));
        quote! {
            #[doc = #color_img]
            pub #ident: AnsiColor
        }
    });
    quote! {
        /// All of the ANSI colors for a particular flavor of Catppuccin.
        /// Obtained via [`Flavor::ansi_colors`].
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct FlavorAnsiColors {
            #(#colors),*
        }

        /// A single ANSI color.
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct AnsiColor {
            /// The [`AnsiColorName`] for this color.
            pub name: AnsiColorName,
            /// The color represented as a six-digit hex string with a leading hash (#).
            pub hex: Hex,
            /// The color represented as individual red, green, and blue channels.
            pub rgb: Rgb,
            /// The color represented as individual hue, saturation, and lightness channels.
            pub hsl: Hsl,
            /// The color's ANSI code.
            pub code: u8,
        }
    }
}

fn make_flavor_ansi_color_pairs_struct_tokens(sample_flavor: &Flavor) -> TokenStream {
    let color_pairs = ansi_color_pairs_in_order(sample_flavor).map(|(k, ..)| {
        let ident = format_ident!("{k}");
        let doc = format!("The normal and bright {k} ANSI color pair.");
        quote! {
            #[doc = #doc]
            pub #ident: AnsiColorPair
        }
    });
    quote! {
        /// All of the ANSI color pairs for a particular flavor of Catppuccin.
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct FlavorAnsiColorPairs {
            #(#color_pairs),*
        }

        /// A pair of ANSI colors - normal and bright.
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct AnsiColorPair {
            /// The [`AnsiColorPairName`] for this color.
            pub name: AnsiColorPairName,
            /// Order of the ANSI color in the palette spec.
            pub order: u32,
            /// The normal color.
            pub normal: AnsiColor,
            /// The bright color.
            pub bright: AnsiColor,
        }
    }
}

fn make_flavor_colors_all_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
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

fn make_flavor_ansi_colors_all_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let ansi_colors = ansi_colors_in_order(sample_flavor).map(|(identifier, _)| {
        let ident = format_ident!("{identifier}");
        quote! { &self.#ident }
    });
    let ansi_color_pairs = ansi_color_pairs_in_order(sample_flavor).map(
        |(identifier, color_pair, normal_identifier, bright_identifier)| {
            make_ansi_color_pair_entry(
                identifier,
                color_pair,
                &normal_identifier,
                &bright_identifier,
            )
        },
    );
    quote! {
        impl FlavorAnsiColors {
            /// Get an array of the ANSI colors in the flavor.
            #[must_use]
            pub const fn all_ansi_colors(&self) -> [&AnsiColor; 16] {
                [
                    #(#ansi_colors),*
                ]
            }

            /// Convert the 16 ANSI colors to 8 ANSI color pairs.
            #[must_use]
            #[allow(clippy::too_many_lines, clippy::unreadable_literal)]
            pub const fn to_ansi_color_pairs(&self) -> FlavorAnsiColorPairs {
                FlavorAnsiColorPairs {
                    #(#ansi_color_pairs),*
                }
            }
        }
    }
}

fn make_flavor_ansi_color_pairs_all_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let items = ansi_color_pairs_in_order(sample_flavor).map(|(identifier, ..)| {
        let ident = format_ident!("{identifier}");
        quote! { &self.#ident }
    });
    quote! {
        impl FlavorAnsiColorPairs {
            /// Get an array of the ANSI color pairs in the flavor.
            #[must_use]
            pub const fn all_ansi_color_pairs(&self) -> [&AnsiColorPair; 8] {
                [
                    #(#items),*
                ]
            }
        }
    }
}

fn make_color_name_enum_tokens(sample_flavor: &Flavor) -> TokenStream {
    let variants = colors_in_order(sample_flavor).map(|(name, _)| {
        let ident = format_ident!("{}", titlecase(name));
        let circles = format!(" {}", color_palette_circles(name));
        quote! {
            #[doc = #circles]
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

fn make_ansi_color_name_enum_tokens(sample_flavor: &Flavor) -> TokenStream {
    let variants = ansi_colors_in_order(sample_flavor).map(|(identifier, color)| {
        let name = remove_whitespace(&color.name);
        let ident = format_ident!("{name}");
        let circles = format!(" {}", ansi_color_palette_circles(&identifier));
        quote! {
            #[doc = #circles]
            #ident
        }
    });
    quote! {
        /// Enum of all named ANSI colors. Can be used to index into a [`FlavorAnsiColors`]
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum AnsiColorName {
            #(#variants),*
        }
    }
}

fn make_ansi_color_pair_name_enum_tokens(sample_flavor: &Flavor) -> TokenStream {
    let variants = ansi_color_pairs_in_order(sample_flavor).map(|(name, ..)| {
        let ident = format_ident!("{}", titlecase(name));
        let circles = format!(" {}", ansi_color_palette_circles(name));
        quote! {
            #[doc = #circles]
            #ident
        }
    });
    quote! {
        /// Enum of all ANSI color pairs. Can be used to index into a [`FlavorAnsiColorPairs`].
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum AnsiColorPairName {
            #(#variants),*
        }
    }
}

fn make_color_name_index_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
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

fn make_ansi_color_name_index_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let first = ansi_colors_in_order(sample_flavor).map(|(identifier, color)| {
        let variant = format_ident!("{}", remove_whitespace(&color.name));
        let ident = format_ident!("{}", identifier);
        quote! {
            AnsiColorName::#variant => &self.#ident
        }
    });
    let second = first.clone();
    quote! {
        impl Index<AnsiColorName> for FlavorAnsiColors {
            type Output = AnsiColor;

            fn index(&self, index: AnsiColorName) -> &Self::Output {
                match index {
                    #(#first),*
                }
            }
        }

        impl FlavorAnsiColors {
            /// Get an ANSI color by name.
            ///
            /// This is equivalent to using the index operator, but can also be used in
            /// const contexts.
            #[must_use]
            pub const fn get_ansi_color(&self, name: AnsiColorName) -> &AnsiColor {
                match name {
                    #(#second),*
                }
            }
        }
    }
}

fn make_ansi_color_pair_name_index_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let first = ansi_color_pairs_in_order(sample_flavor).map(|(identifier, ..)| {
        let variant = format_ident!("{}", titlecase(identifier));
        let ident = format_ident!("{}", identifier);
        quote! {
            AnsiColorPairName::#variant => &self.#ident
        }
    });
    let second = first.clone();
    quote! {
        impl Index<AnsiColorPairName> for FlavorAnsiColorPairs {
            type Output = AnsiColorPair;

            fn index(&self, index: AnsiColorPairName) -> &Self::Output {
                match index {
                    #(#first),*
                }
            }
        }

        impl FlavorAnsiColorPairs {
            /// Get an ANSI color pair by name.
            ///
            /// This is equivalent to using the index operator, but can also be used in
            /// const contexts.
            #[must_use]
            pub const fn get_ansi_color_pair(&self, name: AnsiColorPairName) -> &AnsiColorPair {
                match name {
                    #(#second),*
                }
            }
        }
    }
}

fn make_color_name_display_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let match_arms = colors_in_order(sample_flavor).map(|(identifier, color)| {
        let variant = format_ident!("{}", titlecase(identifier));
        let name = &color.name;
        quote! {
            Self::#variant => write!(f, #name)
        }
    });
    quote! {
        impl core::fmt::Display for ColorName {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}

fn make_ansi_color_name_display_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let match_arms = ansi_colors_in_order(sample_flavor).map(|(_, color)| {
        let name = &color.name;
        let variant = format_ident!("{}", remove_whitespace(name));
        quote! {
            Self::#variant => write!(f, #name)
        }
    });
    quote! {
        impl core::fmt::Display for AnsiColorName {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}

fn make_ansi_color_pair_name_display_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let match_arms = ansi_color_pairs_in_order(sample_flavor).map(|(identifier, ..)| {
        let name = titlecase(identifier);
        let variant = format_ident!("{name}");
        quote! {
            Self::#variant => write!(f, #name)
        }
    });
    quote! {
        impl core::fmt::Display for AnsiColorPairName {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}

fn make_color_name_identifier_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
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

fn make_ansi_color_name_identifier_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let match_arms = ansi_colors_in_order(sample_flavor).map(|(identifier, color)| {
        let variant = format_ident!("{}", remove_whitespace(&color.name));
        quote! {
            Self::#variant => #identifier
        }
    });
    quote! {
        impl AnsiColorName {
            /// Get the ANSI color's identifier; the lowercase key used to identify the color.
            /// This differs from `to_string` in that it's intended for machine usage
            /// rather than presentation.
            ///
            /// Example:
            ///
            /// ```rust
            /// let bright_black = catppuccin::PALETTE.latte.ansi_colors.bright_black;
            /// assert_eq!(bright_black.name.to_string(), "Bright Black");
            /// assert_eq!(bright_black.name.identifier(), "bright_black");
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

fn make_ansi_color_pair_name_identifier_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let match_arms = ansi_color_pairs_in_order(sample_flavor).map(|(identifier, ..)| {
        let variant = format_ident!("{}", titlecase(identifier));
        quote! {
            Self::#variant => #identifier
        }
    });
    quote! {
        impl AnsiColorPairName {
            /// Get the ANSI color pair's identifier; the lowercase key used to identify the color.
            /// This differs from `to_string` in that it's intended for machine usage
            /// rather than presentation.
            ///
            /// Example:
            ///
            /// ```rust
            /// let black_ansi_pair = catppuccin::PALETTE.latte.ansi_colors.all_pairs().black;
            /// assert_eq!(black_ansi_pair.name.to_string(), "Black");
            /// assert_eq!(black_ansi_pair.name.identifier(), "black");
            /// assert_eq!(black_ansi_pair.normal.name.to_string(), "Black");
            /// assert_eq!(black_ansi_pair.normal.name.identifier(), "black");
            /// assert_eq!(black_ansi_pair.bright.name.to_string(), "Bright Black");
            /// assert_eq!(black_ansi_pair.bright.name.identifier(), "bright_black");
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
        impl core::str::FromStr for ColorName {
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

fn make_ansi_color_name_fromstr_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let match_arms = ansi_colors_in_order(sample_flavor)
        .map(|(identifier, color)| {
            let variant = format_ident!("{}", remove_whitespace(&color.name));
            quote! {
                #identifier => Ok(Self::#variant)
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl core::str::FromStr for AnsiColorName {
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

fn make_ansi_color_pair_name_fromstr_impl_tokens(sample_flavor: &Flavor) -> TokenStream {
    let match_arms = ansi_color_pairs_in_order(sample_flavor)
        .map(|(identifier, ..)| {
            let variant = format_ident!("{}", titlecase(identifier));
            quote! {
                #identifier => Ok(Self::#variant)
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl core::str::FromStr for AnsiColorPairName {
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

fn make_palette_const_tokens(palette: &Palette) -> TokenStream {
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
        .map(|(identifier, ansi_color_pair)| make_ansi_color_entry(&identifier, ansi_color_pair));
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

fn make_ansi_color_entry(identifier: &str, ansi_color: &AnsiColor) -> TokenStream {
    let ident = format_ident!("{identifier}");
    let AnsiColor {
        name,
        code,
        rgb: Rgb { r, g, b },
        hsl: Hsl { h, s, l },
    } = ansi_color;

    let name_variant = format_ident!("{}", remove_whitespace(name));
    let rgb = quote! { Rgb { r: #r, g: #g, b: #b } };
    let hsl = quote! { Hsl { h: #h, s: #s, l: #l } };

    quote! {
        #ident: AnsiColor {
            name: AnsiColorName::#name_variant,
            hex: Hex(#rgb),
            rgb: #rgb,
            hsl: #hsl,
            code: #code,
        }
    }
}

fn make_ansi_color_pair_entry(
    identifier: &str,
    ansi_color_pair: &AnsiColorPair,
    normal_identifier: &str,
    bright_identifier: &str,
) -> TokenStream {
    let identifier = format_ident!("{}", identifier);
    let name_identifier = format_ident!("{}", ansi_color_pair.name);
    let order = ansi_color_pair.order;
    let normal_identifier = format_ident!("{}", normal_identifier);
    let bright_identifier = format_ident!("{}", bright_identifier);

    quote! {
        #identifier: AnsiColorPair {
            name: AnsiColorPairName::#name_identifier,
            order: #order,
            normal: self.#normal_identifier,
            bright: self.#bright_identifier,
        }
    }
}
