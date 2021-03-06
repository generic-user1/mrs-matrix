//! Raindrop structure + implementation

use rand::{self, Rng, rngs, seq::SliceRandom};
use crossterm::style::{self, Stylize};

use self::color_algorithms::ColorAlgorithm;

pub mod charsets;
pub mod color_algorithms;

// shortest length a follower will be
const FOLLOWER_MIN_LENGTH: u16 = 4;

// the longest follower is the terminal height minus this offset
const FOLLOWER_MAX_LENGTH_OFFSET: u16 = 4;

// rows will start with a position offset from 0 by a value 
// that is (pseudo)randomly selected from this range
const START_OFFSET_RANGE: std::ops::RangeInclusive<i32> = -64..=-1;

/// A `Raindrop` describes a single 'falling stream' of randomized characters
/// 
/// Raindrops consist of a 'leader' and a 'follower'.
/// The leader is a continuously (per frame) randomized single character at the bottom of the raindrop.
/// The follower is a string of characters that follow the leader. They have randomized length and content,
/// but unlike leaders, are randomized only once (at instantiation) rather than continuously (per frame)
pub struct Raindrop<'a, T>
where T: ColorAlgorithm
{
    // follower_content is ordered such that index 0 represents
    // the first char above the leader, index 1 represents the second, and so on
    // note that Vec<char> is used instead of String; this is because we care about
    // char-by-char indexing more than we care about the potential waste of 3 bytes per char
    follower_content: Vec<char>,

    // row index representing the terminal row that the leader is on
    // the follower will be on indecies below this value
    // note that this value may be negative or greater than the terminal height;
    // this is why an i32 must be used instead of u16
    row_index: i32,

    // reference to a set of characters that will be selected from
    // when generating pseudorandom characters
    charset: &'a Vec<char>,

    // probability of advancing position on any given frame,
    // defaults to 1.0, but can be any value `n` where `0.0 < n <= 1.0`
    advance_chance: f64,

    // ColorAlgorithm implementor that is used to color follower chars
    color_algorithm: T,

    // locally cached random number generator
    local_rng: rngs::ThreadRng
}

impl<'a, T> Raindrop<'a, T>
where T: ColorAlgorithm
{

    /// Returns a (pseudo)randomly generated character from the internal charset
    pub fn gen_char(&mut self) -> char 
    {
        *(self.charset.choose(&mut self.local_rng).unwrap())
    }

    /// Returns a new `Raindrop` instance
    /// 
    /// `charset` should be a reference to Vector of chars.
    /// 
    /// `color_algorithm` should implement 
    /// [ColorAlgorithm](crate::raindrop::color_algorithms::ColorAlgorithm). It defines
    /// how follower characters will be colored.
    /// 
    /// `advance_chance` is the chance that, on any given frame, this `Raindrop` will 
    /// advance its animation. This can be any real number within the range `[0.0, 1.0)`.
    /// If the `advance_chance` is 1.0, this `Raindrop` will always advance its animation.
    /// 
    /// `terminal_height` should be the current height of the terminal, in rows.
    /// 
    ///# Panics
    /// 
    /// This function panics if `advance_chance` is outside the range `[0.0, 1.0)`
    /// 
    ///# Examples
    /// ```
    /// use mrs_matrix::raindrop::{Raindrop, color_algorithms};
    /// use crossterm::terminal;
    /// 
    /// let charset = vec!['a','b', 'c'];
    /// 
    /// let color_algorithm = color_algorithms::LightnessDescending{
    ///     hue: 118.0,
    ///     saturation: 0.82
    /// };
    /// 
    /// let advance_chance = 0.75;
    /// 
    /// let term_height = terminal::size().unwrap().1;
    /// 
    /// let new_raindrop_instance = Raindrop::new(&charset, color_algorithm, advance_chance, term_height);
    /// // do something with instance
    /// ```
    pub fn new(charset: &'a Vec<char>, color_algorithm: T, advance_chance: f64, terminal_height: u16) -> Self
    {
        
        assert!(advance_chance > 0.0, "Attempted to set advance chance at 0 or below");
        assert!(advance_chance <= 1.0, "Attempted to set advance chance greater than 1");

        // create a new `Raindrop` instance
        // use an empty vector for follower content and a zero for row index;
        // these will be overwritten by the call to reinit_state; in fact they could safely be null
        // if rust had a null type
        let mut new_instance  = Self {
            charset,
            color_algorithm,
            local_rng: rand::thread_rng(),
            follower_content: Vec::new(),
            row_index: 0,
            advance_chance
        };

        // do the work of initializing the state of the raindrop;
        // setting its follower_content and row_index pseudorandomly
        new_instance.reinit_state(terminal_height);

        // return the newly created and initialized instance
        new_instance
    }

    /// Re-initializes the state of the `Raindrop` instance 
    /// 
    /// Uses an internally cached random number generator to generate
    /// pseudorandom follower chars and sets the row index to a pseudorandom value
    /// less than (visually 'above') row 0.
    /// 
    /// `terminal_height` should be the current height of the terminal, in rows
    /// 
    /// # Notes
    /// 
    /// The [Raindrop::new](crate::raindrop::Raindrop::new) function uses this function internally
    /// to set the initial state. Calling this function manually is similar to creating
    /// a new `Raindrop` instance outright, but avoids the need to create a new [Rng].
    pub fn reinit_state(&mut self, terminal_height: u16)
    {
        // determine max follower length by subtracting offset from current terminal height
        let max_follower_length = terminal_height.saturating_sub(FOLLOWER_MAX_LENGTH_OFFSET)
        // ensure max follower length is at least FOLLOWER_MIN_LENGTH + 1
        .max(FOLLOWER_MIN_LENGTH + 1);
 
        // use rng to generate follower_content and row_index
        // first determine follower length
        let follower_length = self.local_rng.gen_range(FOLLOWER_MIN_LENGTH..=max_follower_length);
 
        // create empty vector with capacity great enough to hold all follower chars
        let mut new_follower_content = Vec::with_capacity(follower_length.into());
         
        // generate follower_length chars and place them in new_follower_content vec
        for _ in 0..follower_length{
            new_follower_content.push(self.gen_char());
        }

        // store new follower content
        // this needs to be done seperately from using self.gen_char 
        // to satisfy the borrow checker (as both self.follower_length.push 
        // and self.gen_char mutably borrow self)
        self.follower_content = new_follower_content;
 
        // generate and store new row index value
        // this can be done in a single step
        self.row_index = self.local_rng.gen_range(START_OFFSET_RANGE); 
 
        // don't return anything
    }

    /// Returns the character that should be printed for a given row
    /// 
    /// # Notes
    /// 
    /// This function returns an [Option](Option). When requesting a char for a
    /// row that this instance has no char for (for example, because this raindrop 
    /// is above the provided row), `None` will be returned.
    /// If this instance does have a char for the provided row, `Some(char)` is returned.
    pub fn get_char_at_row(&mut self, row_index: u16) -> Option<char>
    {
        
        // cast provided row index to i32 and bind to a more clear name
        // we only want to accept valid u16 values, but need the value to be an i32 for
        // comparisons and math with self.row_index
        let provided_row_index: i32 = row_index.into();

        // return None immediately if provided row is beyond this Raindrop's row
        if self.row_index < provided_row_index{
            return None;
        }
        
        // return a randomly selected char if provided row index points to the leader of this Raindrop
        // (i.e. if the provided row index and current row index match exactly)
        if self.row_index == provided_row_index {
            return Some(self.gen_char());
        }

        // we already checked if provided row index was greater than row index
        // and if provided row index was equal to row index,
        // so if we reach this point, provided row index must be less than row index

        // find the index within follower_content that provided_row_index should point to,
        // keeping min mind that follower starts 1 row above (less than) row_index
        match TryInto::<usize>::try_into((self.row_index - 1) - provided_row_index) 
        {
            Err(_) => {
                //if follower_index can't be represented as a usize for whatever reason,
                //print a warning to stderr and return None
                eprintln!("Failed to represent follower_index ({}) as a usize; skipping char", 
                    (self.row_index - 1) - provided_row_index);
                return None
            },
            Ok(follower_index) => {
                //return either the char at the follower index, or None if there isn't one
                match self.follower_content.get(follower_index) {
                    Some(&val) => Some(val),
                    None => None
                }
            }
        }
        
    }

    /// Returns the character that should be printed for a given row with appropriate styling
    /// 
    /// Internally, uses `get_styled_char` to retrieve the actual character. Then applies a color
    /// according to this `Raindrop`'s `color_algorithm`
    /// 
    /// The leader of the raindrop will always be styled white (and bolded).
    pub fn get_styled_char_at_row(&mut self, row_index: u16) -> Option<style::StyledContent<char>>
    {
        match self.get_char_at_row(row_index){
            //if get_char_at_row returns None, return None immediately
            None => None,
            Some(unstyled_char) => {
                
                
                if self.row_index == row_index.into() {
                    //if char is the leader, style as white (and bold)
                    Some(unstyled_char.with(style::Color::White)
                    .attribute(style::Attribute::Bold))
                } else {
                    //calculate follower proportion from position_in_follower and follower_length
                    let position_in_follower = ((self.row_index - 1) - (row_index as i32)) as f32;
                    let follower_length: f32 = self.follower_content.len() as f32;

                    let follower_proportion = (position_in_follower/follower_length).min(1.0).max(0.0);
                    
                    let char_color = 
                        self.color_algorithm.gen_color(follower_proportion);
                    
                    Some(unstyled_char.with(char_color.into()))
                }
            }
        } 
    }

    /// Moves the `Raindrop` down one row.
    /// 
    /// To reset to the top, use [reinit_state](crate::raindrop::Raindrop::reinit_state).
    pub fn move_drop(&mut self)
    {
        self.row_index += 1;
    }

    /// Returns `true` if Raindrop displays any chars on a terminal of height `terminal_height`; `false` otherwise
    pub fn is_visible(&self, terminal_height: u16) -> bool
    {

        // if row_index is less than zero, return false immediately
        if self.row_index < 0 {
            return false;
        }

        self.row_index < (terminal_height as i32) + (self.follower_content.len() as i32)

    }

    /// Advance the `Raindrop` by one 'frame'
    /// 
    /// `terminal_height` should be the current height of the terminal, in rows.
    /// 
    /// This is similar to [move_drop](crate::raindrop::Raindrop::move_drop), with two key differences:
    /// - If the `Raindrop` is not visible because it has fallen down below the bottom of the terminal,
    /// [reinit_state](crate::raindrop::Raindrop::reinit_state) is called to re-randomize the `Raindrop` and
    /// move it slightly above the top of the terminal.
    /// 
    /// - If the `Raindrop` has had its `advance_chance` set to some value that is not 1.0, this function
    /// will only have a chance of advancing this raindrop's position. If you want to move the `Raindrop` 
    /// for certain, use the [move_drop](crate::raindrop::Raindrop::move_drop) method
    pub fn advance_animation(&mut self, terminal_height: u16)
    {
        // only perform visibility check if current row is not less than 0
        // if we didn't make this check conditional, advance_animation would continuously call reinit_state
        // as raindrops always start above row 0 but are never visible until they reach row 0
        if !(self.row_index < 0) {
            if !self.is_visible(terminal_height){
                self.reinit_state(terminal_height);
                return;
            }
        }
        
        if self.advance_chance == 1.0 {
            // unconditionally move if advance_chance is 1.0, skipping an uneeded rng call
            self.move_drop();
        }
        else if self.local_rng.gen_bool(self.advance_chance) {
            // if advance_chance is not 1.0, perform rng call to decide whether to move
            self.move_drop();
        }
       
    }

}