//! Example demonstrating integration with the `serde` crate.
fn main() {
    let value =
        serde_json::to_string_pretty(&catppuccin::PALETTE).expect("palette can be serialized");
    println!("{value}");
}
