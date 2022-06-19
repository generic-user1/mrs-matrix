//! Raindrop structure + implementation

use rand::{Rng, distributions};
use crossterm::style::{self, Stylize};
use coolor::{self, Hsl};

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
pub struct Raindrop<T>
where T: Rng
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

    // locally cached random number generator
    local_rng: T
}

impl<T> Raindrop<T> 
where T: Rng
{

    /// Returns a (pseudo)randomly generated character
    /// 
    /// Currently only returns ASCII alphanumeric chars, 
    /// but may be extended to return others in the future
    pub fn gen_char(&mut self) -> char 
    {
        self.local_rng.sample(distributions::Alphanumeric).into()  
    }

    /// Returns a new `Raindrop` instance
    /// 
    /// `existing_rng` should implement [Rng](rand::Rng). This is most often 
    /// [ThreadRng](rand::rngs::ThreadRng).
    /// 
    /// `terminal_height` should be the current height of the terminal, in rows
    /// 
    ///# Examples
    /// ```
    /// use mrs_matrix::raindrop::Raindrop;
    /// use crossterm::terminal;
    /// use rand;
    /// 
    /// let term_height = terminal::size().unwrap().1;
    ///
    /// let rng = rand::thread_rng();
    /// 
    /// let new_raindrop_instance = Raindrop::new(rng, term_height);
    /// // do something with instance
    /// ```
    pub fn new(existing_rng: T, terminal_height: u16) -> Self
    {
        // create a new `Raindrop` instance using the passed in existing_rng.
        // use an empty vector for follower content and a zero for row index;
        // these will be overwritten by the call to reinit_state; in fact they could safely be null
        // if rust had a null type
        let mut new_instance  = Self {
            local_rng: existing_rng,
            follower_content: Vec::new(),
            row_index: 0
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
    /// Internally, uses get_styled_char to retrieve the actual character. Then applies a green
    /// tint; brighter if the char is close to the leader, darker if further away.
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
                    //if char is a follower, determine color lightness by subtracting the proportion
                    //of the char's position within the raindrop from 0.9; this results in follower chars
                    //decreasing in brightness as their distance from the leader increases
                    let follower_index: f32 = ((self.row_index - 1) - (row_index as i32)) as f32;
                    let follower_length: f32 = self.follower_content.len() as f32;

                    let follower_proportion = follower_index/follower_length;
                    
                    let char_color = coolor::Color::Hsl(
                        Hsl{     
                            h:118.0, 
                            s:0.82,
                            //use of max ensures lightness is always 0.1 or above 
                            l:((0.9 - follower_proportion).max(0.1))
                        }
                    );
                    
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
    /// This is similar to [move_drop](crate::raindrop::Raindrop::move_drop), with one key difference:
    /// If the `Raindrop` is not visible because it has fallen down below the bottom of the terminal,
    /// [reinit_state](crate::raindrop::Raindrop::reinit_state) is called to re-randomize the `Raindrop` and
    /// move it slightly above the top of the terminal.
    /// 
    /// If the `Raindrop` is not visible because it is above the top of the terminal, or if the `Raindrop` is visible,
    /// this function behaves exactly like [move_drop](crate::raindrop::Raindrop::move_drop).
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

        self.move_drop();
    }

}