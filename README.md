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

```toml
[dependencies]
catppuccin = "1.1.1"
```

## Example

```rust
use catppuccin::MOCHA;

struct Button {
    text: String,
    background_colour: String,
};

fn confirm(text: String) -> Button {
    Button {
        text,
        background_colour: MOCHA.green.hex(),
    }
}
```

More examples can be found
[here](https://github.com/catppuccin/rust/tree/main/examples).

Clone the repository to run them locally:

```bash
$ cargo run --example simple
```

![Output from simple example](https://raw.githubusercontent.com/catppuccin/rust/main/assets/simple-example.png)

```bash
$ cargo run --features ansi --example term
```

![Output from term example](https://raw.githubusercontent.com/catppuccin/rust/main/assets/term-example.png)

## Optional Features

### ANSI string painting

Enable the `ansi` feature to add the
[`Colour::ansi_paint`](crate::Colour::ansi_paint) method.
This adds [ansi-term](https://crates.io/crates/ansi_term) as a dependency.


## 💝 Thanks to

- [backwardspy](https://github.com/backwardspy)
- [Gingeh](https://github.com/Gingeh)
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
