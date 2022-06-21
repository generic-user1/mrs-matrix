//! Algorithms that determine the color of `Raindrop` follower characters

use coolor::{Color, Hsl};

use super::Raindrop;

pub trait ColorAlgorithm: Sized + Copy{
    
    ///Returns a [Color](coolor::Color) that will be applied to a character
    /// 
    /// Passed a reference to an instance of the `Raindrop`, and the position
    /// of the requested character within the follower (with 0 being the closest to the leader)
    fn gen_color(&self, raindrop: &Raindrop<Self>, position_in_follower: usize) -> Color;

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

    fn gen_color(&self, raindrop: &Raindrop<Self>, position_in_follower: usize) -> Color {
        
        assert!(self.hue >= 0.0 && self.hue < 360.0, "hue outside of expected bounds (0, 360]");
        assert!(self.saturation >= 0.0 && self.saturation <= 1.0, 
            "saturation outside of expected bounds (0, 1)");

            //determine color lightness by subtracting the proportion
            //of the char's position within the raindrop from 0.9; this results in follower chars
            //decreasing in brightness as their distance from the leader increases
            let position_in_follower = position_in_follower as f32;
            let follower_length: f32 = raindrop.follower_content.len() as f32;

            let follower_proportion = position_in_follower/follower_length;
            
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
    fn gen_color(&self, raindrop: &Raindrop<Self>, position_in_follower: usize) -> Color {
        assert!(self.hue >= 0.0 && self.hue < 360.0, "hue outside of expected bounds (0, 360]");
        assert!(self.lightness >= 0.0 && self.lightness <= 1.0, 
            "lightness outside of expected bounds (0, 1)");

            //determine color saturation by subtracting the proportion
            //of the char's position within the raindrop from 1.0; this results in follower chars
            //decreasing in saturation as their distance from the leader increases
            let position_in_follower = position_in_follower as f32;
            let follower_length: f32 = raindrop.follower_content.len() as f32;

            let follower_proportion = position_in_follower/follower_length;
            
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
    fn gen_color(&self, raindrop: &Raindrop<Self>, position_in_follower: usize) -> Color {
        assert!(self.saturation >= 0.0 && self.saturation <= 1.0, 
            "saturation outside of expected bounds (0, 1)");
        assert!(self.lightness >= 0.0 && self.lightness <= 1.0, 
            "lightness outside of expected bounds (0, 1)");

            //determine color hue by multiplying follower proportion by 360,
            //producing a valid hue value unique for each char position
            let position_in_follower = position_in_follower as f32;
            let follower_length: f32 = raindrop.follower_content.len() as f32;

            let follower_proportion = position_in_follower/follower_length;
            
            coolor::Color::Hsl(
                Hsl{     
                    h:follower_proportion * 360.0,
                    s:self.saturation, 
                    l:self.lightness
                }
            )
    }
}