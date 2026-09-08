use clap::{ArgEnum, ArgGroup, Parser};
use mrs_matrix::anim_loop;
use mrs_matrix::raindrop::charsets::Charset;
use mrs_matrix::raindrop::{
    charsets,
    color_algorithms::{HueVariation, LightnessDescending},
    RaindropSpeed
};

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
#[clap(group(
    ArgGroup::new("charsetgroup")
    .args(&["charset", "custom-charset"]),
))]
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
    framerate: usize,

    /// Custom character set passed as a string
    #[clap(long)]
    custom_charset: Option<String>
}

fn main() -> crossterm::Result<()> {
    let args = Args::parse();

    let allowed_speeds = if args.sync_scrolling {
        RaindropSpeed::Constant(1.0)
    } else {
        RaindropSpeed::Random((0.25..=1.25).try_into().unwrap())
    };
    let target_framerate = args.framerate;

    let charset = if let Some(charset) = args.custom_charset {
        charset.chars().collect()
    } else {
        match args.charset {
            CharsetType::Alphanumeric => charsets::Alphanumeric().get_charset(),
            CharsetType::PrintableAscii => charsets::PrintableAscii().get_charset(),
            CharsetType::AsciiAndSymbols => charsets::AsciiAndSymbols().get_charset()
        }
    };

    let color_algorithm = match args.color_mode {
        ColorMode::Green => LightnessDescending::try_new(118.0, 1.0).unwrap().into(),

        ColorMode::Blue => LightnessDescending::try_new(244.0, 1.0).unwrap().into(),

        ColorMode::Purple => LightnessDescending::try_new(302.0, 1.0).unwrap().into(),

        ColorMode::Red => LightnessDescending::try_new(0.0, 1.0).unwrap().into(),

        ColorMode::Yellow => LightnessDescending::try_new(51.0, 1.0).unwrap().into(),

        ColorMode::Rainbow => HueVariation::try_new(1.0, 0.5).unwrap().into()
    };
    anim_loop(&charset, color_algorithm, allowed_speeds, target_framerate)
}

/// framerate parser/validator function
fn framerate_in_range(s: &str) -> Result<usize, String> {
    let framerate: usize = s
        .parse()
        .map_err(|_| format!("\"{}\" isn't a valid integer", s))?;

    if framerate == 0 {
        Err("framerate cannot be zero".to_owned())
    } else {
        Ok(framerate)
    }
}
