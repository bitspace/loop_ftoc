fn main() {
    let fahrenheit = 43;

    println!(
        "{} degrees fahrenheit is {} degrees celsius",
        fahrenheit,
        (fahrenheit - 32) * 5 / 9
    );
}
