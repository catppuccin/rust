#[cfg(feature = "ansi")]
use std::{borrow::Cow, fmt::Debug};

/// A simple three-channel RGB colour representation.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Colour(pub u8, pub u8, pub u8);

impl Colour {
    /// Returns a hexadecimal string representing the colour.
    ///
    /// # Examples
    ///
    /// ```
    /// use catppuccin::Flavour;
    ///
    /// let hex = Flavour::Mocha.teal().hex();
    /// assert_eq!(hex, "94E2D5");
    /// ```
    pub fn hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

#[cfg(feature = "css")]
impl From<Colour> for css_colors::RGB {
    fn from(value: Colour) -> Self {
        Self {
            r: css_colors::Ratio::from_u8(value.0),
            g: css_colors::Ratio::from_u8(value.1),
            b: css_colors::Ratio::from_u8(value.2),
        }
    }
}

#[cfg(feature = "ansi")]
impl Colour {
    /// Paints the given input with the colour à la [ansi_term](https://docs.rs/ansi_term/latest/ansi_term/)
    pub fn ansi_paint<'a, I, S: 'a + ToOwned + ?Sized>(
        &self,
        input: I,
    ) -> ansi_term::ANSIGenericString<'a, S>
    where
        I: Into<Cow<'a, S>>,
        <S as ToOwned>::Owned: Debug,
    {
        ansi_term::Colour::RGB(self.0, self.1, self.2).paint(input)
    }
}

impl From<Colour> for (u8, u8, u8) {
    /// Converts the colour into a tuple of its component red, green, and blue channels.
    fn from(colour: Colour) -> Self {
        (colour.0, colour.1, colour.2)
    }
}
