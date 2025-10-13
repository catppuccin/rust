<h3 align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
	Catppuccin for <a href="https://www.rust-lang.org/">Rust</a>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
	<a href="https://github.com/catppuccin/rust/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/rust?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
	<a href="https://github.com/catppuccin/rust/issues"><img src="https://img.shields.io/github/issues/catppuccin/rust?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
	<a href="https://github.com/catppuccin/rust/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/rust?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

<p align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/rust/main/assets/ferris.webp"/>
</p>

## Usage

Add Catppuccin to your project's `Cargo.toml`:

```console
$ cargo add catppuccin
```

## Example

```rust
struct Button {
    text: String,
    background_color: String,
};

fn confirm(text: String) -> Button {
    Button {
        text,
        background_color: catppuccin::PALETTE.mocha.colors.green.hex.to_string(),
    }
}
```

More examples can be found
[here](https://github.com/catppuccin/rust/tree/main/examples).

## Optional Features

### ANSI string painting

Enable the `ansi-term` feature to add the
[`Color::ansi_paint`](Color::ansi_paint) method.
This adds [ansi-term](https://crates.io/crates/ansi_term) as a dependency.

Example: [`examples/term_grid.rs`](https://github.com/catppuccin/rust/blob/main/examples/term_grid.rs)

#### Bevy

Enable the `bevy` feature to enable the conversion of Catppuccin colors to
[`bevy::prelude::Color`] instances.
This adds [bevy](https://crates.io/crates/bevy) as a dependency.

Example: [`examples/bevy.rs`](https://github.com/catppuccin/rust/blob/main/examples/bevy.rs)

#### CSS colors

Enable the `css-colors` feature to enable the conversion of Catppuccin colors to
[`css_colors::RGB`] instances.
This adds [css-colors](https://crates.io/crates/css-colors) as a dependency.

Example: [`examples/css.rs`](https://github.com/catppuccin/rust/blob/main/examples/css.rs)

#### Iced

Enable the `iced` feature to enable the conversion of Catppuccin colors to
[`iced::Color`] instances.
This adds [iced](https://crates.io/crates/iced) as a dependency.

Example: [`examples/iced.rs`](https://github.com/catppuccin/rust/blob/main/examples/iced.rs)

#### Ratatui

Enable the `ratatui` feature to enable the conversion of Catppuccin colors to
[`ratatui::style::Color`] instances.
This adds [ratatui](https://crates.io/crates/ratatui) as a dependency.

Example: [`examples/ratatui.rs`](https://github.com/catppuccin/rust/blob/main/examples/ratatui.rs)

#### Serde

Enable the `serde` feature to enable the serialization of Catppuccin's palette,
flavor, and color types.
This adds [serde](https://crates.io/crates/serde) as a dependency.

Example: [`examples/serde.rs`](https://github.com/catppuccin/rust/blob/main/examples/serde.rs)

## Contributing

This project uses [pre-commit](https://pre-commit.com/) to maintain consistent code style and standards.

See also [CONTRIBUTING.md](https://github.com/catppuccin/catppuccin/blob/main/CONTRIBUTING.md)

## 💝 Thanks to

- [backwardspy](https://github.com/backwardspy)
- [Gingeh](https://github.com/Gingeh)
- [Julius](https://github.com/juliuskreutz)
- [Nyx](https://github.com/nyxkrage)

&nbsp;

<p align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" />
</p>

<p align="center">
	Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
</p>

<p align="center">
	<a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8"/></a>
</p>
