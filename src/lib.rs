//! 🦀 Soothing pastel theme for Rust.
//!
//! # Usage
//!
//! Add Catppuccin to your project's `Cargo.toml`:
//!
//! ```console
//! $ cargo add catppuccin
//! ```
//!
//! # Example
//!
//! ```rust
//! struct Button {
//!     text: String,
//!     background_color: String,
//! };
//!
//! fn confirm(text: String) -> Button {
//!     Button {
//!         text,
//!         background_color: catppuccin::PALETTE.mocha.colors.green.hex.to_string(),
//!     }
//! }
//! ```
//!
//! More examples can be found
//! [here](https://github.com/catppuccin/rust/tree/main/examples).
//!
//! # Optional Features
//!
//! ## ANSI string painting
//!
//! Enable the `ansi-term` feature to add the
//! [`Color::ansi_paint`](Color::ansi_paint) method.
//! This adds [ansi-term](https://crates.io/crates/ansi_term) as a dependency.
//!
//! Example: [`examples/term_grid.rs`](https://github.com/catppuccin/rust/blob/main/examples/term_grid.rs)
//!
//! ### CSS colors
//!
//! Enable the `css-colors` feature to enable the conversion of Catppuccin colors to
//! [`css_colors::RGB`] instances.
//! This adds [css-colors](https://crates.io/crates/css-colors) as a dependency.
//!
//! Example: [`examples/css.rs`](https://github.com/catppuccin/rust/blob/main/examples/css.rs)
//!
//! ### Ratatui
//!
//! Enable the `ratatui` feature to enable the conversion of Catppuccin colors to
//! [`ratatui::style::Color`] instances.
//! This adds [ratatui](https://crates.io/crates/ratatui) as a dependency.
//!
//! Example: [`examples/ratatui.rs`](https://github.com/catppuccin/rust/blob/main/examples/ratatui.rs)
//!
//! ### Serde
//!
//! Enable the `serde` feature to enable the serialization of Catppuccin's palette,
//! flavor, and color types.
//! This adds [serde](https://crates.io/crates/serde) as a dependency.
//!
//! Example: [`examples/serde.rs`](https://github.com/catppuccin/rust/blob/main/examples/serde.rs)
use std::{fmt, marker::PhantomData, ops::Index};

include!(concat!(env!("OUT_DIR"), "/generated_palette.rs"));

/// The top-level type that encompasses the Catppuccin palette data structure.
/// Primarily used via the [`PALETTE`] constant.
///
/// Can be iterated over, in which case the flavors are yielded in the canonical order:
/// Latte, Frappé, Macchiato, Mocha.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Palette {
    /// The light flavor.
    pub latte: Flavor,
    /// The lightest dark flavor.
    pub frappe: Flavor,
    /// The medium dark flavor.
    pub macchiato: Flavor,
    /// The darkest dark flavor.
    pub mocha: Flavor,
}

/// Enum of all four flavors of Catppuccin. Can be used to index [`Palette`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum FlavorName {
    /// The light flavor.
    Latte,
    /// The lightest dark flavor.
    #[cfg_attr(feature = "serde", serde(rename = "Frappé"))]
    Frappe,
    /// The medium dark flavor.
    Macchiato,
    /// The darkest dark flavor.
    Mocha,
}

/// An iterator over flavors in the palette.
/// Obtained via [`Palette::iter()`].
pub struct FlavorIterator<'a> {
    current: usize,
    phantom: PhantomData<&'a ()>,
}

/// Color represented as individual red, green, and blue channels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Rgb {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
}

/// Color represented as 6-digit hexadecimal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hex(Rgb);

/// Color represented as individual hue (0-359), saturation (0-1), and lightness (0-1) channels.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Hsl {
    /// Hue channel.
    pub h: f64,
    /// Saturation channel.
    pub s: f64,
    /// Lightness channel.
    pub l: f64,
}

/// A single color in the Catppuccin palette.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Color {
    /// The [`ColorName`] for this color.
    pub name: ColorName,
    /// Whether the color is considered an accent color.
    /// Accent colors are the first 14 colors in the palette, also called
    /// the analogous colours. The remaining 12 non-accent colors are also
    /// referred to as the monochromatic colors.
    pub accent: bool,
    /// The color represented as a six-digit hex string with a leading hash (#).
    pub hex: Hex,
    /// The color represented as individual red, green, and blue channels.
    pub rgb: Rgb,
    /// The color represented as individual hue, saturation, and lightness channels.
    pub hsl: Hsl,
}

/// A flavor is a collection of colors. Catppuccin has four flavors; Latte,
/// Frappé, Macchiato, and Mocha.
///
/// Can be iterated over, in which case the colors are yielded in order.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Flavor {
    /// The name of the flavor.
    pub name: FlavorName,
    /// Whether this flavor is dark or light oriented. Latte is light, the other
    /// three flavors are dark.
    pub dark: bool,
    /// The colors in the flavor.
    pub colors: FlavorColors,
}

/// An iterator over colors in a flavor.
/// Obtained via [`Flavor::into_iter()`](struct.Flavor.html#method.into_iter) or [`FlavorColors::iter()`].
pub struct ColorIterator<'a> {
    colors: &'a FlavorColors,
    current: usize,
}

impl Palette {
    /// Get an array of the flavors in the palette.
    #[must_use]
    pub const fn all_flavors(&self) -> [&Flavor; 4] {
        [&self.latte, &self.frappe, &self.macchiato, &self.mocha]
    }

    /// Create an iterator over the flavors in the palette.
    #[must_use]
    pub const fn iter(&self) -> FlavorIterator {
        FlavorIterator {
            current: 0,
            phantom: PhantomData,
        }
    }
}

impl Index<FlavorName> for Palette {
    type Output = Flavor;

    fn index(&self, index: FlavorName) -> &Self::Output {
        match index {
            FlavorName::Latte => &self.latte,
            FlavorName::Frappe => &self.frappe,
            FlavorName::Macchiato => &self.macchiato,
            FlavorName::Mocha => &self.mocha,
        }
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Rgb { r, g, b } = self.0;
        write!(f, "#{r:02x}{g:02x}{b:02x}")
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Hex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl fmt::Display for FlavorName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Latte => write!(f, "Latte"),
            Self::Frappe => write!(f, "Frappé"),
            Self::Macchiato => write!(f, "Macchiato"),
            Self::Mocha => write!(f, "Mocha"),
        }
    }
}

impl FlavorColors {
    /// Create an iterator over the colors in the flavor.
    #[must_use]
    pub const fn iter(&self) -> ColorIterator {
        ColorIterator {
            colors: self,
            current: 0,
        }
    }
}

impl<'a> Iterator for FlavorIterator<'a> {
    type Item = &'a Flavor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= PALETTE.all_flavors().len() {
            None
        } else {
            let flavor = PALETTE.all_flavors()[self.current];
            self.current += 1;
            Some(flavor)
        }
    }
}

impl<'a> IntoIterator for &'a Palette {
    type Item = &'a Flavor;
    type IntoIter = FlavorIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Iterator for ColorIterator<'a> {
    type Item = &'a Color;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.colors.all_colors().len() {
            None
        } else {
            let color = self.colors.all_colors()[self.current];
            self.current += 1;
            Some(color)
        }
    }
}

impl<'a> IntoIterator for &'a FlavorColors {
    type Item = &'a Color;
    type IntoIter = ColorIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Flavor {
    /// Create an iterator over the colors in the flavor.
    #[must_use]
    pub const fn iter(&self) -> ColorIterator {
        self.colors.iter()
    }
}

impl<'a> IntoIterator for &'a Flavor {
    type Item = &'a Color;
    type IntoIter = ColorIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.colors.iter()
    }
}

impl Index<ColorName> for Flavor {
    type Output = Color;

    fn index(&self, index: ColorName) -> &Self::Output {
        self.colors.index(index)
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

impl From<(u8, u8, u8)> for Hex {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self(Rgb { r, g, b })
    }
}

impl From<(f64, f64, f64)> for Hsl {
    fn from((h, s, l): (f64, f64, f64)) -> Self {
        Self { h, s, l }
    }
}

#[cfg(feature = "css-colors")]
impl From<Color> for css_colors::RGB {
    fn from(value: Color) -> Self {
        Self {
            r: css_colors::Ratio::from_u8(value.rgb.r),
            g: css_colors::Ratio::from_u8(value.rgb.g),
            b: css_colors::Ratio::from_u8(value.rgb.b),
        }
    }
}

#[cfg(feature = "ansi-term")]
impl Color {
    /// Paints the given input with a color à la [ansi_term](https://docs.rs/ansi_term/latest/ansi_term/)
    pub fn ansi_paint<'a, I, S: 'a + ToOwned + ?Sized>(
        &self,
        input: I,
    ) -> ansi_term::ANSIGenericString<'a, S>
    where
        I: Into<std::borrow::Cow<'a, S>>,
        <S as ToOwned>::Owned: core::fmt::Debug,
    {
        ansi_term::Color::RGB(self.rgb.r, self.rgb.g, self.rgb.b).paint(input)
    }
}

#[cfg(feature = "ratatui")]
impl From<Color> for ratatui::style::Color {
    fn from(value: Color) -> Self {
        Self::Rgb(value.rgb.r, value.rgb.g, value.rgb.b)
    }
}
