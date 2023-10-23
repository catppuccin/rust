use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    path::Path,
};

use serde::Deserialize;

const PALETTE_URL: &str =
    "https://raw.githubusercontent.com/catppuccin/palette/v0.2.0/palette-porcelain.json";
const PALETTE_PATH: &str = ".cache/palette.json";
const CODEGEN_PATH: &str = "codegen.rs";

#[derive(Debug, Deserialize)]
struct Color {
    hex: String,
    rgb: [u8; 3],
    hsl: [f32; 3],
}

type Palette = HashMap<String, HashMap<String, Color>>;

fn is_dark(name: &str) -> bool {
    name != "latte"
}

fn is_accent(name: &str) -> bool {
    [
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
    ]
    .contains(&name)
}

fn main() -> Result<(), Box<dyn Error>> {
    let palette_path = Path::new(PALETTE_PATH);

    if !palette_path.exists() {
        println!("cargo:warning=no palette json in cache, fetching...");
        download_palette(palette_path)?;
    }

    let palette: Palette = serde_json::from_reader(BufReader::new(File::open(palette_path)?))?;

    let codegen_path = Path::new(&env::var("OUT_DIR")?).join(CODEGEN_PATH);
    let codegen_file = BufWriter::new(File::create(&codegen_path)?);
    let mut code_writer = BufWriter::new(codegen_file);

    let mut flavors_map = phf_codegen::Map::new();
    for (flavor_name, flavor) in palette.into_iter() {
        let mut colors_map = phf_codegen::Map::new();
        for (color_name, color) in flavor.into_iter() {
            colors_map.entry(
                color_name.clone(),
                &format!(
                    r#"Color {{
                        name: "{}",
                        is_accent: {},
                        hex: "{}",
                        rgb: &[{}, {}, {}],
                        hsl: &[{}.0, {}, {}],
                    }}"#,
                    color_name,
                    is_accent(&color_name),
                    color.hex,
                    color.rgb[0],
                    color.rgb[1],
                    color.rgb[2],
                    color.hsl[0],
                    color.hsl[1],
                    color.hsl[2]
                ),
            );
        }
        flavors_map.entry(
            flavor_name.clone(),
            &format!(
                r#"Flavor {{
                    name: "{}",
                    dark: {},
                    colors: {},
                }}"#,
                flavor_name,
                is_dark(&flavor_name),
                colors_map.build()
            ),
        );
    }
    writeln!(
        &mut code_writer,
        "pub static PALETTE: Palette = Palette {{ flavors: {} }};",
        flavors_map.build()
    )?;

    Ok(())
}

fn download_palette(path: &Path) -> Result<(), Box<dyn Error>> {
    let contents = reqwest::blocking::get(PALETTE_URL)?.text()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}
