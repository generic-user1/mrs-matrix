use mrs_matrix::anim_loop;
use mrs_matrix::raindrop::{charsets, color_algorithms};

fn main() -> crossterm::Result<()> 
{
    let charset = charsets::AsciiAndSymbols();
    let color_algorithm = color_algorithms::LightnessDescending{
        hue: 118.0,
        saturation: 0.82
    };
    anim_loop(charset, color_algorithm, 0.75, 25)
}