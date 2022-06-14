use termion::color;

fn main() {
    println!("{}{}{}",
        color::Fg(color::Green),
        "Hello, world!", 
        color::Fg(color::Reset));
}
