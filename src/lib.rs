#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]

mod palette;
pub use palette::PALETTE;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Color {
    pub name: &'static str,
    pub accent: bool,
    pub hex: &'static str,
    pub rgb: &'static [u8; 3],
    pub hsl: &'static [f64; 3],
}

#[derive(Debug)]
pub struct Flavor {
    pub name: &'static str,
    pub dark: bool,
    pub colors: phf::Map<&'static str, Color>,
}

#[derive(Debug)]
pub struct Palette {
    pub flavors: phf::Map<&'static str, Flavor>,
}

static FLAVORS_ORDER: &[&str] = &["latte", "frappe", "macchiato", "mocha"];
static COLOURS_ORDER: &[&str] = &[
    "rosewater",
    "flamingo",
    "pink",
    "mauve",
    "red",
    "maroon",
    "peach",
    "yellow",
    "green",
    "teal",
    "sky",
    "sapphire",
    "blue",
    "lavender",
    "text",
    "subtext1",
    "subtext0",
    "overlay2",
    "overlay1",
    "overlay0",
    "surface2",
    "surface1",
    "surface0",
    "base",
    "mantle",
    "crust",
];

pub struct FlavorIterator {
    current: usize,
}

pub struct ColorIterator {
    flavor: &'static Flavor,
    current: usize,
}

impl Iterator for FlavorIterator {
    type Item = &'static Flavor;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= FLAVORS_ORDER.len() {
            None
        } else {
            let flavor = FLAVORS_ORDER[self.current];
            self.current += 1;
            Some(&PALETTE.flavors[flavor])
        }
    }
}

impl Iterator for ColorIterator {
    type Item = &'static Color;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= COLOURS_ORDER.len() {
            None
        } else {
            let color = COLOURS_ORDER[self.current];
            self.current += 1;
            Some(&self.flavor.colors[color])
        }
    }
}

impl Palette {
    #[must_use]
    pub const fn iter(&self) -> FlavorIterator {
        FlavorIterator { current: 0 }
    }
}

impl IntoIterator for &'static Palette {
    type Item = &'static Flavor;
    type IntoIter = FlavorIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Flavor {
    #[must_use]
    pub const fn iter(&'static self) -> ColorIterator {
        ColorIterator {
            flavor: self,
            current: 0,
        }
    }
}

impl IntoIterator for &'static Flavor {
    type Item = &'static Color;
    type IntoIter = ColorIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn assert_flavor(flavor: Option<&Flavor>, name: &str) {
        assert!(flavor.is_some());
        assert_eq!(flavor.unwrap().name, name);
    }

    fn assert_color(color: Option<&Color>, name: &str) {
        assert!(color.is_some());
        assert_eq!(color.unwrap().name, name);
    }

    #[test]
    fn iterate_flavors() {
        let mut iter = PALETTE.iter();
        assert_flavor(iter.next(), "Latte");
        assert_flavor(iter.next(), "Frappé");
        assert_flavor(iter.next(), "Macchiato");
        assert_flavor(iter.next(), "Mocha");
        assert!(iter.next().is_none());
    }

    #[test]
    fn iterate_colors() {
        let mut iter = PALETTE.flavors["latte"].iter();
        assert_color(iter.next(), "rosewater");
        assert_color(iter.next(), "flamingo");
        assert_color(iter.next(), "pink");
        assert_color(iter.next(), "mauve");
        assert_color(iter.next(), "red");
        assert_color(iter.next(), "maroon");
        assert_color(iter.next(), "peach");
        assert_color(iter.next(), "yellow");
        assert_color(iter.next(), "green");
        assert_color(iter.next(), "teal");
        assert_color(iter.next(), "sky");
        assert_color(iter.next(), "sapphire");
        assert_color(iter.next(), "blue");
        assert_color(iter.next(), "lavender");
        assert_color(iter.next(), "text");
        assert_color(iter.next(), "subtext1");
        assert_color(iter.next(), "subtext0");
        assert_color(iter.next(), "overlay2");
        assert_color(iter.next(), "overlay1");
        assert_color(iter.next(), "overlay0");
        assert_color(iter.next(), "surface2");
        assert_color(iter.next(), "surface1");
        assert_color(iter.next(), "surface0");
        assert_color(iter.next(), "base");
        assert_color(iter.next(), "mantle");
        assert_color(iter.next(), "crust");
        assert!(iter.next().is_none());
    }
}
