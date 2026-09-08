//! Algorithms that determine the color of `Raindrop` follower characters
use std::ops::{Range, RangeInclusive};

const UNIT_INTERVAL: RangeInclusive<f32> = 0.0..=1.0;
const DEG_INTERVAL: Range<f32> = 0.0..360.0;

use coolor::{Color, Hsl};

/// Different algorithms for how to color the different characters in a [Raindrop](crate::raindrop::Raindrop)
#[derive(Clone)]
pub enum ColorAlgorithm {
    /// Colors characters with varying lightness according to their distance from the leader
    LightnessDescending(LightnessDescending),
    /// Colors characters with varying saturation according to their distance from the leader
    SaturationDescending(SaturationDescending),
    /// Colors characters with varying hue according to their distance from the leader
    HueVariation(HueVariation)
}
impl ColorAlgorithm {
    ///Returns a [Color] that will be applied to a character
    ///
    /// Passed a `follower_proportion` within the range `[0.0, 1.0]` representing
    /// how far away this char is from the leader (with 1.0 being max distance)
    ///
    ///# Notes
    ///
    /// This function panics if `follower_proportion` is less than 0 or greater than 1.
    pub fn gen_color(&self, follower_proportion: f32) -> Color {
        assert!(
            UNIT_INTERVAL.contains(&follower_proportion),
            "follower_proportion outside of expected bounds [0, 1]"
        );
        match self {
            Self::LightnessDescending(LightnessDescending { hue, saturation }) => {
                //determine color lightness by subtracting the follower_proportion from 0.9;
                //this results in follower chars decreasing in brightness as their distance
                //from the leader increases
                coolor::Color::Hsl(Hsl {
                    h: *hue,
                    s: *saturation,
                    //use of max ensures lightness is always 0.1 or above
                    l: ((0.9 - follower_proportion).max(0.1))
                })
            }
            Self::SaturationDescending(SaturationDescending { hue, lightness }) => {
                //determine color saturation by subtracting the follower_proportion from 1.0;
                //this results in follower chars decreasing in saturation as their distance
                //from the leader increases
                coolor::Color::Hsl(Hsl {
                    h: *hue,
                    l: *lightness,
                    //use of max ensures saturation is always 0.0 or above
                    s: ((1.0 - follower_proportion).max(0.0))
                })
            }
            Self::HueVariation(HueVariation {
                saturation,
                lightness
            }) => {
                //determine color hue by multiplying follower proportion by 360,
                //producing a valid hue value unique for each char position
                coolor::Color::Hsl(Hsl {
                    h: follower_proportion * 360.0,
                    s: *saturation,
                    l: *lightness
                })
            }
        }
    }
}

/// Colors characters with varying lightness according to their distance from the leader
#[derive(Clone)]
pub struct LightnessDescending {
    hue: f32,
    saturation: f32
}
impl LightnessDescending {
    /// Create a new LightnessDescending
    ///
    /// `hue` is the hue degree of the base color. It must be within the range `[0.0, 360.0)`.
    ///
    /// `saturation` is the saturation amount of the base color. It must be within the range `[0.0, 1.0]`.
    ///
    ///# Notes
    ///
    /// If `hue` or `saturation` are outside of their expected ranges, `new` will panic
    pub fn new(hue: f32, saturation: f32) -> Self {
        assert!(
            DEG_INTERVAL.contains(&hue),
            "hue outside of expected bounds [0, 360)"
        );
        assert!(
            UNIT_INTERVAL.contains(&saturation),
            "saturation outside of expected bounds [0, 1]"
        );
        Self { hue, saturation }
    }
}
impl From<LightnessDescending> for ColorAlgorithm {
    fn from(value: LightnessDescending) -> Self {
        ColorAlgorithm::LightnessDescending(value)
    }
}

/// Colors characters with varying saturation according to their distance from the leader
#[derive(Clone)]
pub struct SaturationDescending {
    hue: f32,
    lightness: f32
}
impl SaturationDescending {
    /// Create a new SaturationDescending
    ///
    /// `hue` is the hue degree of the base color. It must be within the range `[0, 360)`.
    ///
    /// `lightness` is the lightness amount of the base color. It must be within the range `[0.0, 1.0]`.
    ///
    ///# Notes
    ///
    /// If `hue` or `lightness` are outside of their expected ranges, `new` will panic
    pub fn new(hue: f32, lightness: f32) -> Self {
        assert!(
            DEG_INTERVAL.contains(&hue),
            "hue outside of expected bounds [0, 360)"
        );
        assert!(
            UNIT_INTERVAL.contains(&lightness),
            "lightness outside of expected bounds [0, 1]"
        );
        Self { hue, lightness }
    }
}
impl From<SaturationDescending> for ColorAlgorithm {
    fn from(value: SaturationDescending) -> Self {
        ColorAlgorithm::SaturationDescending(value)
    }
}

/// Colors characters with varying hue according to their distance from the leader
#[derive(Clone)]
pub struct HueVariation {
    saturation: f32,
    lightness: f32
}
impl HueVariation {
    /// Create a new HueVariation
    ///
    /// `saturation` is the saturation amount of the base color. It must be within the range `[0.0, 1.0]`.
    ///
    /// `lightness` is the lightness amount of the base color. It must be within the range `[0.0, 1.0]`.
    ///
    ///# Notes
    ///  
    /// If `hue` or `lightness` are outside of their expected ranges, `new` will panic
    pub fn new(saturation: f32, lightness: f32) -> Self {
        assert!(
            UNIT_INTERVAL.contains(&saturation),
            "saturation outside of expected bounds [0, 1]"
        );
        assert!(
            UNIT_INTERVAL.contains(&lightness),
            "lightness outside of expected bounds [0, 1]"
        );
        Self {
            saturation,
            lightness
        }
    }
}
impl From<HueVariation> for ColorAlgorithm {
    fn from(value: HueVariation) -> Self {
        ColorAlgorithm::HueVariation(value)
    }
}
