fn main() {
    let f = 205.0;
    let c = f_to_c(f);
    println!("{f} in degrees celsius is {c}")
}

fn f_to_c(fahrenheit: f64) -> f64 {
    ((fahrenheit - 32.0) * 5.0) / 9.0
}
