use catppuccin::Flavour;

fn main() {
    let (r, g, b) = Flavour::Latte.teal().into();
    println!(
        "Latte's teal is #{}, which is rgb({r}, {g}, {b})",
        Flavour::Latte.teal().hex()
    );

    // You can also take from the full palette of colours:
    let mocha = Flavour::Mocha.colours();
    let (r, g, b) = mocha.teal.into();
    println!(
        "Mocha's teal is #{}, which is rgb({r}, {g}, {b})",
        Flavour::Mocha.colours().teal.hex()
    );
}
