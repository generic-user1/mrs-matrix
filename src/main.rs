use clap::{error::ErrorKind, ArgGroup, CommandFactory, Parser, ValueEnum};
use mrs_matrix::anim_loop;

use mrs_matrix::raindrop::{
    charsets::{self, Charset},
    color_algorithms::{ColorAlgorithm, HueVariation, LightnessDescending},
    RaindropSpeed
};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CharsetType {
    Alphanumeric,
    PrintableAscii,
    AsciiAndSymbols,
    Katakana
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ColorMode {
    Green,
    Blue,
    Purple,
    Red,
    Yellow,
    Rainbow
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(group(
    ArgGroup::new("charsetgroup")
    .args(&["charset", "custom_charset"])
))]
//These two groups "speedmin" and "speedmax" are to enforce that speed can't be used with min_speed or max_speed,
//but min_speed and max_speed can (and in fact must) be used with each other.
//It feels like there ought to be a better way to express this.
#[command(group(ArgGroup::new("speedmin").args(&["speed", "min_speed"])))]
#[command(group(ArgGroup::new("speedmax").args(&["speed", "max_speed"])))]
struct MainArgs {
    /// Defines how characters will be colored.
    #[arg(short, long, value_enum, default_value_t = ColorMode::Green)]
    color_mode: ColorMode,

    /// Defines the character set that will be drawn from.
    #[arg(long, value_enum, default_value_t = CharsetType::AsciiAndSymbols)]
    charset: CharsetType,

    /// Single speed in rows per frame
    #[arg(short, long, default_value_t = 1.0)]
    speed: f64,

    /// Minimum possible speed in rows per frame
    #[arg(short = 'i', long, requires = "max_speed")]
    min_speed: Option<f64>,

    /// Maximum possible speed in rows per frame
    #[arg(short = 'a', long, requires = "min_speed")]
    max_speed: Option<f64>,

    /// Sets the target framerate
    #[arg(short, long, value_parser=framerate_in_range, default_value_t = 25)]
    framerate: usize,

    /// Custom character set passed as a string
    #[arg(long)]
    custom_charset: Option<String>
}

/// Handle transforming the speed-related arguments into a concrete RaindropSpeed
///
/// As part of this work, validate that if min_speed and max_speed are used, the resulting range isn't empty.
/// The entire program bails out if the resulting range is empty (similar to how [Parser::parse] does), because from the user's perspective,
/// we want an empty range to look similar to missing min_speed or max_speed, or otherwise passing an invalid combination of arguments
fn to_raindrop_speed(
    single_speed: f64,
    min_speed: Option<f64>,
    max_speed: Option<f64>
) -> RaindropSpeed {
    match (single_speed, min_speed, max_speed) {
        (_, Some(min_speed), Some(max_speed)) => {
            let range = min_speed..=max_speed;
            if let Ok(range) = range.try_into() {
                RaindropSpeed::Random(range)
            } else {
                MainArgs::command()
                    .error(
                        ErrorKind::ValueValidation,
                        "value for --max-speed was less than value for --min-speed"
                    )
                    .exit()
            }
        }
        (speed, _, _) => RaindropSpeed::Constant(speed)
    }
}

/// Handle transforming the charset-related arguments into a concrete Vec<char>
fn to_charset(custom_charset: Option<String>, charset_type: CharsetType) -> Vec<char> {
    match (custom_charset, charset_type) {
        (Some(charset), _) => charset.get_charset(),
        (_, CharsetType::Alphanumeric) => charsets::Alphanumeric().get_charset(),
        (_, CharsetType::PrintableAscii) => charsets::PrintableAscii().get_charset(),
        (_, CharsetType::AsciiAndSymbols) => charsets::AsciiAndSymbols().get_charset(),
        (_, CharsetType::Katakana) => charsets::Katakana().get_charset()
    }
}

/// Handle transforming the color mode argument into a concrete ColorAlgorithm
fn to_color_algorithm(color_mode: ColorMode) -> ColorAlgorithm {
    match color_mode {
        ColorMode::Green => LightnessDescending::try_new(118.0, 1.0).unwrap().into(),

        ColorMode::Blue => LightnessDescending::try_new(244.0, 1.0).unwrap().into(),

        ColorMode::Purple => LightnessDescending::try_new(302.0, 1.0).unwrap().into(),

        ColorMode::Red => LightnessDescending::try_new(0.0, 1.0).unwrap().into(),

        ColorMode::Yellow => LightnessDescending::try_new(51.0, 1.0).unwrap().into(),

        ColorMode::Rainbow => HueVariation::try_new(1.0, 0.5).unwrap().into()
    }
}

fn main() -> crossterm::Result<()> {
    let args = MainArgs::parse();

    let allowed_speeds = to_raindrop_speed(args.speed, args.min_speed, args.max_speed);

    let target_framerate = args.framerate;

    let charset = to_charset(args.custom_charset, args.charset);

    let color_algorithm = to_color_algorithm(args.color_mode);

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
