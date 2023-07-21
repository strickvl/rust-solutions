use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Alex Strick van Linschoten")
        .about("Rust echo")
        .get_matches();
}
