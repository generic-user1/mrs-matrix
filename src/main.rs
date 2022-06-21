use mrs_matrix::anim_loop;
use mrs_matrix::raindrop::charsets::Charset;
use mrs_matrix::raindrop::{charsets, color_algorithms};
use clap::{ArgEnum, Parser};

#[derive(Debug, Clone, Copy, ArgEnum)]
enum CharsetType {
    Alphanumeric,
    PrintableAscii,
    AsciiAndSymbols
}

#[derive(Debug, Clone, Copy, ArgEnum)]
enum ColorMode {
    Green,
    Blue,
    Purple,
    Red,
    Yellow,
    Rainbow
}

#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
struct Args {
   
    /// Defines how characters will be colored.
    #[clap(short, long, arg_enum, value_parser, default_value_t = ColorMode::Green)]
    color_mode: ColorMode,

    /// Defines the character set that will be drawn from.
    #[clap(long, arg_enum, value_parser, default_value_t = CharsetType::AsciiAndSymbols)]
    charset: CharsetType,

    /// Run in synchronized scrolling mode
    #[clap(short, long)]
    sync_scrolling: bool,

    /// Sets the target framerate
    #[clap(short, long, value_parser=framerate_in_range, default_value_t = 25)]
    framerate: usize

}

fn main() -> crossterm::Result<()> 
{
    let args = Args::parse();

    let advance_chance = if args.sync_scrolling {1.0} else {0.75};
    let target_framerate = args.framerate;

    let charset = match args.charset {
        CharsetType::Alphanumeric => charsets::Alphanumeric().get_charset(),
        CharsetType::PrintableAscii => charsets::PrintableAscii().get_charset(),
        CharsetType::AsciiAndSymbols => charsets::AsciiAndSymbols().get_charset()
    };

    //we need a seperate call to anim_loop for each possible type of ColorAlgorithm
    //to avoid this, we would need to use a trait object (like Box<dyn ColorAlgorithm>),
    //but that would incur a runtime penalty that we could like to avoid
    
    match args.color_mode {
        ColorMode::Green => {
            let color_algorithm = color_algorithms::LightnessDescending{
                hue: 118.0,
                saturation: 1.0
            };
            anim_loop(charset, color_algorithm, advance_chance, target_framerate)
        },
        
        ColorMode::Blue => {
            let color_algorithm = color_algorithms::LightnessDescending{
                hue: 244.0,
                saturation: 1.0
            };
            anim_loop(charset, color_algorithm, advance_chance, target_framerate)
        },

        ColorMode::Purple => {
            let color_algorithm = color_algorithms::LightnessDescending{
                hue: 302.0,
                saturation: 1.0
            };
            anim_loop(charset, color_algorithm, advance_chance, target_framerate)
        },

        ColorMode::Red => {
            let color_algorithm = color_algorithms::LightnessDescending{
                hue: 0.0,
                saturation: 1.0
            };
            anim_loop(charset, color_algorithm, advance_chance, target_framerate)
        },

        ColorMode::Yellow => {
            let color_algorithm = color_algorithms::LightnessDescending{
                hue: 51.0,
                saturation: 1.0
            };
            anim_loop(charset, color_algorithm, advance_chance, target_framerate)
        }

        ColorMode::Rainbow => {
            let color_algorithm = color_algorithms::HueVariation{
                saturation: 1.0, lightness: 0.5
            };
            anim_loop(charset, color_algorithm, advance_chance, target_framerate)
        }
    }
        
}

/// framerate parser/validator function
fn framerate_in_range(s: &str) -> Result<usize, String>
{
    let framerate: usize = s.parse().map_err(|_| format!("\"{}\" isn't a valid integer", s))?;

    if framerate == 0 {
        Err(format!("framerate cannot be zero"))
    } else {
        Ok(framerate)
    }
}