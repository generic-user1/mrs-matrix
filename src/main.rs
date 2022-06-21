use mrs_matrix::anim_loop;
use mrs_matrix::raindrop::{charsets, color_algorithms};

const DEFAULT_COLOR_ALGORITHM: color_algorithms::LightnessDescending =
color_algorithms::LightnessDescending{
    hue: 118.0,
    saturation: 0.82
};

fn main() -> crossterm::Result<()> 
{
    let charset = charsets::AsciiAndSymbols();
    let color_algorithm = DEFAULT_COLOR_ALGORITHM;
    anim_loop(charset, color_algorithm, 0.75, 25)
}