//! Raindrop structure + implementation

use crossterm::style::{self, Stylize};
use rand::{
    self,
    distr::{uniform::Uniform, Distribution},
    rngs,
    seq::IndexedRandom,
    Rng, RngExt
};

use self::color_algorithms::ColorAlgorithm;

pub mod charsets;
pub mod color_algorithms;

// shortest length a follower will be
const FOLLOWER_MIN_LENGTH: u16 = 4;

// the longest follower is the terminal height minus this offset
const FOLLOWER_MAX_LENGTH_OFFSET: u16 = 4;

// rows will start with a position offset from 0 by a value
// that is (pseudo)randomly selected from this range
const START_OFFSET_RANGE: std::ops::RangeInclusive<f64> = -64.0..=-1.0;

/// A [Raindrop]'s speed
///
/// Raindrops have some speed, measured in rows per frame. For example, a speed of 1.0 means the
/// raindrop will advance by 1 row each frame, a speed of 2.0 means the raindrop will advance by 2 rows each frame,
/// and a speed of 0.5 means the raindrop will advance by half a row each frame - i.e. it will advance one row every 2 frames.
///
/// A raindrop always has a constant speed while on-screen, but its speed is allowed to change after it has dropped off the bottom
/// of the screen and has yet to drop in from the top of the screen.
#[derive(Debug, Clone)]
pub enum RaindropSpeed {
    /// Use a constant speed
    Constant(f64),

    /// Define a range of speeds the [Raindrop] is allowed to have
    /// One specific speed will be chosen at random when the [Raindrop]
    /// begins to fall, and a new speed will be chosen when it reaches the end of the screen
    /// and begins to fall again
    Random(Uniform<f64>)
}

impl RaindropSpeed {
    fn get_speed<T: Rng>(&self, rng: &mut T) -> f64 {
        match self {
            Self::Constant(v) => *v,
            Self::Random(range) => range.sample(rng)
        }
    }
}

/// A `Raindrop` describes a single 'falling stream' of randomized characters
///
/// Raindrops consist of a 'leader' and a 'follower'.
/// The leader is a continuously (per frame) randomized single character at the bottom of the raindrop.
/// The follower is a string of characters that follow the leader. They have randomized length and content,
/// but unlike leaders, are randomized only once (at instantiation) rather than continuously (per frame)
pub struct Raindrop<'a> {
    // follower_content is ordered such that index 0 represents
    // the first char above the leader, index 1 represents the second, and so on
    // note that Vec<char> is used instead of String; this is because we care about
    // char-by-char indexing more than we care about the potential waste of 3 bytes per char
    follower_content: Vec<char>,

    // row index representing the terminal row that the leader is on
    // the follower will be on indecies below this value
    // the integer component of this value represents the actual row the leader is on,
    // the the fractional component is used to support speeds that aren't whole rows at a time
    row_index: f64,

    // reference to a set of characters that will be selected from
    // when generating pseudorandom characters
    charset: &'a [char],

    // speed, in rows, that this raindrop will advance per frame
    current_speed: f64,

    // what speeds this raindrop is allowed to have
    allowed_speeds: RaindropSpeed,

    // ColorAlgorithm that is used to color follower chars
    color_algorithm: ColorAlgorithm,

    // locally cached random number generator
    local_rng: rngs::ThreadRng
}

impl<'a> Raindrop<'a> {
    /// Returns a new `Raindrop` instance
    ///
    /// `charset` specifies what chars the `Raindrop` may use, and must not be empty.
    ///
    /// `color_algorithm` should be a [ColorAlgorithm].
    /// It defines how follower characters will be colored.
    ///
    /// `allowed_speeds` defines what speeds the `Raindrop` is allowed to have.
    /// The speed of a raindrop is how fast it moves down the screen, measured in rows per frame.
    /// See [RaindropSpeed] for details.
    ///
    /// `terminal_height` should be the current height of the terminal, in rows.
    ///
    ///# Examples
    /// ```
    /// use mrs_matrix::raindrop::{Raindrop, RaindropSpeed, color_algorithms};
    /// use crossterm::terminal;
    ///
    /// // this is only necessary because we want to use RaindropSpeed::Random;
    /// // if we used RaindropSpeed::Constant, we wouldn't need to use it.
    /// use rand::distr::uniform::Uniform;
    ///
    /// let charset = vec!['a','b', 'c'];
    ///
    /// let color_algorithm = color_algorithms::LightnessDescending::try_new(118.0, 0.82).unwrap().into();
    ///
    /// let speed = RaindropSpeed::Random(Uniform::try_from(0.75..1.25).unwrap());
    ///
    /// let term_height = terminal::size().unwrap().1;
    ///
    /// let new_raindrop_instance = Raindrop::new(&charset, color_algorithm, speed, term_height);
    /// // do something with instance
    /// ```
    pub fn new(
        charset: &'a [char],
        color_algorithm: ColorAlgorithm,
        speed: RaindropSpeed,
        terminal_height: u16
    ) -> Self {
        assert!(
            !charset.is_empty(),
            "tried to create Raindrop with empty charset"
        );

        // set up attributes that need setting up, pack them into a new instance, and return it
        let mut local_rng = rand::rng();
        let follower_content = Self::gen_follower_content(&mut local_rng, charset, terminal_height);
        let row_index = Self::calc_initial_row(&mut local_rng);
        let current_speed = speed.get_speed(&mut local_rng);

        Self {
            charset,
            color_algorithm,
            local_rng: rand::rng(),
            follower_content,
            row_index,
            current_speed,
            allowed_speeds: speed
        }
    }

    /// Generate a single character from the provided charset
    ///
    /// Charset must not be empty
    fn gen_char<T: RngExt>(rng: &mut T, charset: &'a [char]) -> char {
        *(charset.choose(rng).unwrap())
    }

    /// Generate a vec of characters from the provided charset that is of appropriate size
    ///
    /// Charset must not be empty
    fn gen_follower_content<T: RngExt>(
        rng: &mut T,
        charset: &'a [char],
        terminal_height: u16
    ) -> Vec<char> {
        // determine max follower length by subtracting offset from current terminal height
        let max_follower_length = terminal_height
            .saturating_sub(FOLLOWER_MAX_LENGTH_OFFSET)
            // ensure max follower length is at least FOLLOWER_MIN_LENGTH + 1
            .max(FOLLOWER_MIN_LENGTH + 1);

        // determine actual follower length
        let follower_length = rng.random_range(FOLLOWER_MIN_LENGTH..=max_follower_length);

        // create that many follower chars
        let mut out = Vec::with_capacity(follower_length.into());
        for _ in 0..follower_length {
            out.push(Self::gen_char(rng, charset));
        }
        out
    }

    /// Pseudorandomly determine what row index this `Raindrop` should start at
    ///
    /// This returns f64 because internally that's what `Raindrop` uses for its row index,
    /// but the value of the f64 is always an integer - this means that raindrops with the same speed
    /// will always advance on the same frame as each other (barring floating-point imprecision nonsense)
    fn calc_initial_row<T: RngExt>(rng: &mut T) -> f64 {
        rng.random_range(START_OFFSET_RANGE).trunc()
    }

    /// Re-initializes the state of the `Raindrop` instance
    ///
    /// Uses an internally cached random number generator to generate
    /// pseudorandom follower chars, sets the row index to a pseudorandom value
    /// less than (visually 'above') row 0, and (if the [RaindropSpeed] calls for it)
    /// pseudorandomly determines the new speed of the raindrop.
    ///
    /// `terminal_height` should be the current height of the terminal, in rows
    ///
    /// # Notes
    ///
    /// The [Raindrop::new](crate::raindrop::Raindrop::new) function uses this function internally
    /// to set the initial state. Calling this function manually is similar to creating
    /// a new `Raindrop` instance outright, but avoids the need to create a new [Rng].
    pub fn reinit_state(&mut self, terminal_height: u16) {
        //update the follower
        self.follower_content =
            Self::gen_follower_content(&mut self.local_rng, self.charset, terminal_height);

        // update the row index
        self.row_index = Self::calc_initial_row(&mut self.local_rng);

        // update the speed; note that if our speed is RaindropSpeed::Constant, this will return the same value every time.
        self.current_speed = self.allowed_speeds.get_speed(&mut self.local_rng);
    }

    /// Get the integer portion of this `Raindrop`'s row index
    const fn int_row_index(&self) -> i32 {
        // the `as` here performs a saturating cast, which is what we want
        self.row_index.trunc() as i32
    }

    /// Returns the character that should be printed for a given row
    ///
    /// # Notes
    ///
    /// This function returns `None` when this raindrop has no char for the given row
    /// (because, for example, this raindrop is above the provided row).
    pub fn get_char_at_row(&mut self, row_index: u16) -> Option<char> {
        // cast provided row index to i32 and bind to a more clear name
        // we only want to accept valid u16 values, but want the value to be an i32 for
        // comparisons and math with our own row index
        let provided_row_index = row_index as i32;

        let our_row_index = self.int_row_index();

        // return None immediately if provided row is beyond this Raindrop's row
        if our_row_index < provided_row_index {
            return None;
        }

        // return a randomly selected char if provided row index points to the leader of this Raindrop
        // (i.e. if the provided row index and current row index match exactly)
        if our_row_index == provided_row_index {
            return Some(Self::gen_char(&mut self.local_rng, self.charset));
        }

        // we already checked if provided row index was greater than row index
        // and if provided row index was equal to row index,
        // so if we reach this point, provided row index must be less than row index

        // find the index within follower_content that provided_row_index should point to,
        // keeping min mind that follower starts 1 row above (less than) row_index
        match TryInto::<usize>::try_into((our_row_index - 1) - provided_row_index) {
            Err(_) => {
                //if follower_index can't be represented as a usize for whatever reason,
                //print a warning to stderr and return None
                eprintln!(
                    "Failed to represent follower_index ({}) as a usize; skipping char",
                    (our_row_index - 1) - provided_row_index
                );
                None
            }
            Ok(follower_index) => {
                //return either the char at the follower index, or None if there isn't one
                self.follower_content.get(follower_index).copied()
            }
        }
    }

    /// Returns the character that should be printed for a given row with appropriate styling
    ///
    /// Internally, uses `get_styled_char` to retrieve the actual character. Then applies a color
    /// according to this `Raindrop`'s `color_algorithm`
    ///
    /// The leader of the raindrop will always be styled white (and bolded).
    pub fn get_styled_char_at_row(&mut self, row_index: u16) -> Option<style::StyledContent<char>> {
        match self.get_char_at_row(row_index) {
            //if get_char_at_row returns None, return None immediately
            None => None,
            Some(unstyled_char) => {
                if self.row_index == row_index.into() {
                    //if char is the leader, style as white (and bold)
                    Some(
                        unstyled_char
                            .with(style::Color::White)
                            .attribute(style::Attribute::Bold)
                    )
                } else {
                    //calculate follower proportion from position_in_follower and follower_length
                    let position_in_follower =
                        ((self.int_row_index() - 1) - (row_index as i32)) as f32;
                    let follower_length = self.follower_content.len() as f32;

                    let follower_proportion =
                        (position_in_follower / follower_length).clamp(0.0, 1.0);

                    let char_color = self.color_algorithm.gen_color(follower_proportion);

                    Some(unstyled_char.with(char_color.into()))
                }
            }
        }
    }

    /// Moves the `Raindrop` down by a distance determined by its speed.
    ///
    /// To reset to the top, use [reinit_state](crate::raindrop::Raindrop::reinit_state).
    pub fn move_drop(&mut self) {
        self.row_index += self.current_speed
    }

    /// Returns `true` if Raindrop displays any chars on a terminal of height `terminal_height`; `false` otherwise
    pub fn is_visible(&self, terminal_height: u16) -> bool {
        // if row_index is less than zero, return false immediately
        let row_index = self.int_row_index();
        if row_index < 0 {
            return false;
        }

        row_index < (terminal_height as i32) + (self.follower_content.len() as i32)
    }

    /// Advance the `Raindrop` by one 'frame'
    ///
    /// `terminal_height` should be the current height of the terminal, in rows.
    ///
    /// This is similar to [move_drop](crate::raindrop::Raindrop::move_drop), with one key difference
    /// if the `Raindrop` is not visible because it has fallen down below the bottom of the terminal,
    /// [reinit_state](crate::raindrop::Raindrop::reinit_state) is called to re-randomize the `Raindrop` and
    /// move it slightly above the top of the terminal.
    pub fn advance_animation(&mut self, terminal_height: u16) {
        // only perform visibility check if current row is not less than 0
        // if we didn't make this check conditional, advance_animation would continuously call reinit_state
        // as raindrops always start above row 0 but are never visible until they reach row 0
        if !(self.int_row_index() < 0) && !self.is_visible(terminal_height) {
            self.reinit_state(terminal_height);
            return;
        }

        self.move_drop();
    }
}
