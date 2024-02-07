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
mod generated_palette;
pub use generated_palette::PALETTE;

/// The top-level type that encompasses the Catppuccin palette data structure.
/// Primarily used via the [`PALETTE`] constant.
///
/// Can be iterated over, in which case the flavors are yielded in the canonical order:
/// Latte, Frappé, Macchiato, Mocha.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Palette {
    pub latte: Flavor,
    pub frappe: Flavor,
    pub macchiato: Flavor,
    pub mocha: Flavor,
}

/// An iterator over flavors in the palette.
/// Obtained via [`Palette::iter()`].
pub struct FlavorIterator {
    current: usize,
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
    /// The name of the color, such as "mauve" or "overlay2".
    pub name: &'static str,
    /// Whether the color is considered an accent color.
    /// Accent colors are the first 14 colors in the palette, also called
    /// the analogous colours. The remaining 12 non-accent colors are also
    /// referred to as the monochromatic colors.
    pub accent: bool,
    /// The color represented as a six-digit hex string with a leading hash (#).
    pub hex: &'static str,
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
    /// The name of the flavor, such as "Latte" or "Mocha".
    pub name: &'static str,
    /// Whether this flavor is dark or light oriented. Latte is light, the other
    /// three flavors are dark.
    pub dark: bool,
    /// The colors in the flavor.
    pub colors: FlavorColors,
}

/// All of the colors for a particular flavor of Catppuccin.
/// Obtained via [`Flavor::colors`].
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FlavorColors {
    pub rosewater: Color,
    pub flamingo: Color,
    pub pink: Color,
    pub mauve: Color,
    pub red: Color,
    pub maroon: Color,
    pub peach: Color,
    pub yellow: Color,
    pub green: Color,
    pub teal: Color,
    pub sky: Color,
    pub sapphire: Color,
    pub blue: Color,
    pub lavender: Color,
    pub text: Color,
    pub subtext1: Color,
    pub subtext0: Color,
    pub overlay2: Color,
    pub overlay1: Color,
    pub overlay0: Color,
    pub surface2: Color,
    pub surface1: Color,
    pub surface0: Color,
    pub base: Color,
    pub mantle: Color,
    pub crust: Color,
}

/// An iterator over colors in a flavor.
/// Obtained via [`Flavor::into_iter()`](struct.Flavor.html#method.into_iter) or [`FlavorColors::iter()`].
pub struct ColorIterator {
    colors: &'static FlavorColors,
    current: usize,
}

impl Palette {
    /// Get an array of the flavors in the palette.
    #[must_use]
    pub const fn all_flavors(&'static self) -> [&'static Flavor; 4] {
        [&self.latte, &self.frappe, &self.macchiato, &self.mocha]
    }

    /// Create an iterator over the flavors in the palette.
    #[must_use]
    pub const fn iter(&'static self) -> FlavorIterator {
        FlavorIterator { current: 0 }
    }
}

impl FlavorColors {
    /// Get an array of the colors in the flavor.
    #[must_use]
    pub const fn all_colors(&'static self) -> [&'static Color; 26] {
        [
            &self.rosewater,
            &self.flamingo,
            &self.pink,
            &self.mauve,
            &self.red,
            &self.maroon,
            &self.peach,
            &self.yellow,
            &self.green,
            &self.teal,
            &self.sky,
            &self.sapphire,
            &self.blue,
            &self.lavender,
            &self.text,
            &self.subtext1,
            &self.subtext0,
            &self.overlay2,
            &self.overlay1,
            &self.overlay0,
            &self.surface2,
            &self.surface1,
            &self.surface0,
            &self.base,
            &self.mantle,
            &self.crust,
        ]
    }

    /// Create an iterator over the colors in the flavor.
    #[must_use]
    pub const fn iter(&'static self) -> ColorIterator {
        ColorIterator {
            colors: self,
            current: 0,
        }
    }
}

impl Iterator for FlavorIterator {
    type Item = &'static Flavor;

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

impl IntoIterator for &'static Palette {
    type Item = &'static Flavor;
    type IntoIter = FlavorIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Iterator for ColorIterator {
    type Item = &'static Color;

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

impl IntoIterator for &'static FlavorColors {
    type Item = &'static Color;
    type IntoIter = ColorIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Flavor {
    #[must_use]
    pub const fn iter(&'static self) -> ColorIterator {
        self.colors.iter()
    }
}

impl IntoIterator for &'static Flavor {
    type Item = &'static Color;
    type IntoIter = ColorIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.colors.iter()
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
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
