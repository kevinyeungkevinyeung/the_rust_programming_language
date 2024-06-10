use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    mix(red, yellow);
}
// cargo build --> dev profile
// cargo build --release --> release profile

// cargo doc --open --> open the documentation for ur software on a web
