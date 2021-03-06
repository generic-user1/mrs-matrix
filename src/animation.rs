//! Functions relating directly to drawing animations on the screen
//! 
use std::io::{stdout, Write};
use std::time::{Instant, Duration};
use crossterm::{
    self,
    event::{self, Event},
    QueueableCommand, 
    style::{Print, PrintStyledContent},
    terminal,
    cursor
};
use crate::raindrop::{Raindrop, color_algorithms::ColorAlgorithm};

/// Returns a `Vec<Raindrop>` with one `Raindrop` for each terminal column
/// 
/// `charset` should be a reference to a Vector of chars. This will be the set of 
/// characters that the raindrops will be generated from.
/// 
/// `advance_chance` is the chance that a `Raindrop` will advance on any given frame.
/// 
/// `terminal_width` should be the width of the terminal in columns
/// 
/// `terminal_height` should be the height of the terminal in rows
/// 
/// Note that this function is intentionally private because it's unlikely to be generally useful
fn create_raindrops<T>(charset: &Vec<char>, color_algorithm: T, 
    advance_chance:f64, terminal_width: u16, terminal_height: u16) 
-> Vec<Raindrop<T>>
where T: ColorAlgorithm
{
    let mut raindrop_vec: Vec<Raindrop<T>> = Vec::with_capacity(terminal_width.into());

    for _ in 0..terminal_width {
        let new_raindrop = Raindrop::new(
            charset, color_algorithm, advance_chance, terminal_height);
        raindrop_vec.push(new_raindrop);
    }

    raindrop_vec
}

/// The main loop that renders the screen
/// 
/// Returns after receiving any keypress
/// 
/// `charset` should be a `Vec<char>`. This will be the set of characters that will be
/// displayed within the animation.
/// 
/// `color_algorithm` should be an instance of a type implementing [ColorAlgorithm], such as
/// [LightnessDescending](crate::raindrop::color_algorithms::LightnessDescending).
/// 
/// `advance_chance` should be the chance (from 0.0 to 1.0) that any one `Raindrop` will advance
/// its movement on any given frame. This value must be within the range `[0.0, 1.0)`.
/// 
/// `target_framerate` should be the number of frames per second to target.
/// 
/// # Panics
/// 
/// This function panics if `charset` is empty (i.e. has a length of zero).
/// 
/// This function panics if `target_framerate` is zero.
/// 
/// This function panics if `advance_chance` is outside the range `[0.0, 1.0)`
/// 
/// # Examples
/// ```
/// use mrs_matrix::animation::anim_loop;
/// use mrs_matrix::raindrop::charsets::{Charset, PrintableAscii};
/// use mrs_matrix::raindrop::color_algorithms::LightnessDescending;
/// 
/// pub fn main() -> crossterm::Result<()>
/// {
///     let charset = PrintableAscii().get_charset();
///     let color_algorithm = LightnessDescending{
///         hue: 118.0,
///         saturation: 0.82
///     };
///     let advance_chance = 0.75;
///     let target_framerate = 25;
///     anim_loop(charset, color_algorithm, advance_chance, target_framerate)
/// }
/// ```
pub fn anim_loop<T: ColorAlgorithm>(charset: Vec<char>, color_algorithm: T,
     advance_chance:f64, target_framerate: usize) -> crossterm::Result<()>
{
    assert!(charset.len() > 0, "cannot run anim_loop with empty character set");
    assert!(target_framerate > 0, 
        "cannot run anim_loop at target framerate of zero");

    let mut out = stdout();

    let (mut term_cols, mut term_rows) = terminal::size()?;

    //enable raw mode to process keypress by keypress
    terminal::enable_raw_mode()?;

    //enter alternate screen, and hide the cursor
    out.queue(terminal::EnterAlternateScreen)?
    .queue(cursor::Hide)?;

    //calculate target frame duration by dividing one second by the number of frames that should be in one second
    let target_frame_duration = Duration::from_secs_f64(1.0/(target_framerate as f64));

    let mut raindrop_vector = 
        create_raindrops(&charset, color_algorithm, advance_chance, 
            term_cols, term_rows);

    let mut start_instant: Instant;
    loop {
        start_instant = Instant::now();

        //reset cursor position
        out.queue(cursor::MoveTo(0,0))?;

        //iterate through all rows
        for row_index in 0..term_rows {

            //strangely, these commands seem to be 1 based, unlike MoveTo
            out.queue(cursor::MoveToRow(row_index + 1))?
            .queue(cursor::MoveToColumn(1))?;

            //iterate through all columns by iterating through raindrop_vector, printing styled chars where applicable
            //note that spaces are printed for columns on this row without a printable char
            for raindrop in raindrop_vector.iter_mut() {
                match raindrop.get_styled_char_at_row(row_index) {
                    None => out.queue(Print(" "))?,
                    Some(styled_char) => out.queue(PrintStyledContent(styled_char))?
                };
            }
        }

        //flush buffer to 'draw'
        out.flush()?;

        //call advance_animation on all the raindrops
        for raindrop in raindrop_vector.iter_mut() {
            raindrop.advance_animation(term_rows);
        }
    
        //wait for enough time to hit target_frame_duration, or no time if frame duration exceeds target
        if event::poll(target_frame_duration.saturating_sub(Instant::now() - start_instant))? {
            match event::read()? {
                //upon recieving a resize event set new column amount
                Event::Resize(new_cols, new_rows) => {
                    term_cols = new_cols;
                    term_rows = new_rows;

                    raindrop_vector = 
                        create_raindrops(&charset, color_algorithm,
                            advance_chance, term_cols, term_rows);
                },
                //stop loop upon recieving a mouse or key event
                _ => break
            }
        }
    }

    //disable raw mode
    terminal::disable_raw_mode()?;

    //be sure to leave the alternate screen and show the cursor again
    out.queue(terminal::LeaveAlternateScreen)?
    .queue(cursor::Show)?;
    out.flush()?;

    Ok(())
}