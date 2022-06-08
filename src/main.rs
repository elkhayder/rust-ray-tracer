mod models;

use models::colors::Color;

fn main() {
    let c1 = Color {
        r: 1.0,
        g: 0.2,
        b: 0.4,
    };
    let c2 = Color {
        r: 0.9,
        g: 1.0,
        b: 0.1,
    };

    println!("c1 * c2 = {:?}", &c1 * &c2);
}
