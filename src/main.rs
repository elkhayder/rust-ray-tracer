mod helpers;
mod models;

use models::canvas::Canvas;

use crate::models::colors::Color;

fn main() {
    let mut canvas = Canvas::new(10, 2, Some(Color::new(1.0, 0.8, 0.6)));

    println!("{:?}", canvas);

    canvas.save();
}
