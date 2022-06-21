//! Algorithms that determine the color of `Raindrop` follower characters

use coolor::{Color, Hsl};

pub trait ColorAlgorithm: Sized + Copy{
    
    ///Returns a [Color](coolor::Color) that will be applied to a character
    /// 
    /// Passed a `follower_proportion` within the range `[0.0, 1.0]` representing
    /// how far away this char is from the leader (with 1.0 being max distance)
    /// 
    ///# Notes
    /// 
    /// This function should panic if `follower_proportion` is less than 0 or greater than 1.
    fn gen_color(&self, follower_proportion: f32) -> Color;

}

/// Colors characters with varying lightness according to their distance from the leader
/// 
/// `hue` is the hue degree of the base color. It must be within the range `(0.0, 360.0]`.
/// 
/// `saturation` is the saturation amount of the base color. It must be within the range `(0.0, 1.0)`.
/// 
///# Notes
/// 
/// If `hue` or `saturation` are outside of their expected ranges, `gen_color` will panic
#[derive(Copy, Clone)]
pub struct LightnessDescending {
    pub hue: f32,
    pub saturation: f32
}
impl ColorAlgorithm for LightnessDescending {

    fn gen_color(&self, follower_proportion: f32) -> Color {
        assert!(follower_proportion >= 0.0 && follower_proportion <= 1.0,
            "follower_proportion outside of expected bounds (0, 1)");
        assert!(self.hue >= 0.0 && self.hue < 360.0, "hue outside of expected bounds (0, 360]");
        assert!(self.saturation >= 0.0 && self.saturation <= 1.0, 
            "saturation outside of expected bounds (0, 1)");

            //determine color lightness by subtracting the follower_proportion from 0.9; 
            //this results in follower chars decreasing in brightness as their distance 
            //from the leader increases
            
            coolor::Color::Hsl(
                Hsl{     
                    h:self.hue, 
                    s:self.saturation,
                    //use of max ensures lightness is always 0.1 or above 
                    l:((0.9 - follower_proportion).max(0.1))
                }
            )
    }

}

/// Colors characters with varying saturation according to their distance from the leader
/// 
/// `hue` is the hue degree of the base color. It must be within the range `(0.0, 360.0]`.
/// 
/// `lightness` is the lightness amount of the base color. It must be within the range `(0.0, 1.0)`.
/// 
///# Notes
/// 
/// If `hue` or `lightness` are outside of their expected ranges, `gen_color` will panic
#[derive(Clone, Copy)]
pub struct SaturationDescending{
    pub hue: f32,
    pub lightness: f32
}
impl ColorAlgorithm for SaturationDescending {
    fn gen_color(&self, follower_proportion: f32) -> Color {
        assert!(follower_proportion >= 0.0 && follower_proportion <= 1.0,
            "follower_proportion outside of expected bounds (0, 1)");
        assert!(self.hue >= 0.0 && self.hue < 360.0, "hue outside of expected bounds (0, 360]");
        assert!(self.lightness >= 0.0 && self.lightness <= 1.0, 
            "lightness outside of expected bounds (0, 1)");

            //determine color saturation by subtracting the follower_proportion from 1.0; 
            //this results in follower chars decreasing in saturation as their distance 
            //from the leader increases
            coolor::Color::Hsl(
                Hsl{     
                    h:self.hue, 
                    l:self.lightness,
                    //use of max ensures saturation is always 0.0 or above 
                    s:((1.0 - follower_proportion).max(0.0))
                }
            )
    }
}

/// Colors characters with varying hue according to their distance from the leader
/// 
/// `saturation` is the saturation amount of the base color. It must be within the range `(0.0, 1.0)`.
/// 
/// `lightness` is the lightness amount of the base color. It must be within the range `(0.0, 1.0)`.
///
///# Notes
///  
/// If `hue` or `lightness` are outside of their expected ranges, `gen_color` will panic
#[derive(Clone, Copy)]
pub struct HueVariation {
    pub saturation: f32,
    pub lightness: f32
}
impl ColorAlgorithm for HueVariation {
    fn gen_color(&self, follower_proportion: f32) -> Color {
        assert!(follower_proportion >= 0.0 && follower_proportion <= 1.0,
            "follower_proportion outside of expected bounds (0, 1)");
        assert!(self.saturation >= 0.0 && self.saturation <= 1.0, 
            "saturation outside of expected bounds (0, 1)");
        assert!(self.lightness >= 0.0 && self.lightness <= 1.0, 
            "lightness outside of expected bounds (0, 1)");

            //determine color hue by multiplying follower proportion by 360,
            //producing a valid hue value unique for each char position
            coolor::Color::Hsl(
                Hsl{     
                    h:follower_proportion * 360.0,
                    s:self.saturation, 
                    l:self.lightness
                }
            )
    }
}