use mrs_matrix::anim_loop;
use mrs_matrix::raindrop::charsets;

fn main() -> crossterm::Result<()> 
{
    anim_loop(charsets::AsciiAndSymbols(), 0.75, 25)
}