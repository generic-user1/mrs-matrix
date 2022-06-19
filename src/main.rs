use mrs_matrix::anim_loop;
use mrs_matrix::raindrop::Charset;

fn main() -> crossterm::Result<()> 
{
    anim_loop(Charset::PrintableAscii, 25)
}