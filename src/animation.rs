//! Functions relating directly to drawing animations on the screen
//!
use crate::raindrop::{color_algorithms::ColorAlgorithm, Raindrop, RaindropSpeed};
use crossterm::{
    self, cursor,
    event::{self, Event, KeyEvent, KeyEventKind},
    style::{Print, PrintStyledContent},
    terminal, QueueableCommand
};
use rand::RngExt;

use std::io::{stdout, Write};
use std::time::{Duration, Instant};

/// Returns a `Vec<Raindrop>` with one `Raindrop` for each terminal column
///
/// `rng` is the source of randomness to use.
///
/// `charset` is be the set of characters that the raindrops will be generated from.
///
/// `allowed_speeds` defines what speeds each `Raindrop` is allowed to have.
///
/// `terminal_width` should be the width of the terminal in columns
///
/// `terminal_height` should be the height of the terminal in rows
fn create_raindrops<'a, T: RngExt>(
    rng: &mut T,
    charset: &'a [char],
    color_algorithm: ColorAlgorithm,
    allowed_speeds: RaindropSpeed,
    terminal_width: u16,
    terminal_height: u16
) -> Vec<Raindrop<'a>> {
    let mut raindrop_vec: Vec<Raindrop> = Vec::with_capacity(terminal_width.into());

    for _ in 0..terminal_width {
        let new_raindrop = Raindrop::new(
            rng,
            charset,
            color_algorithm.clone(),
            allowed_speeds.clone(),
            terminal_height
        );
        raindrop_vec.push(new_raindrop);
    }

    raindrop_vec
}

/// The main loop that renders the screen
///
/// Returns after receiving any keypress
///
/// `rng` is the source of randomness to use.
///
/// `charset` will be the set of characters that will be
/// displayed within the animation.
///
/// `color_algorithm` defines how follower characters will be colored.
///
/// `allowed_speeds` defines what speeds each [Raindrop] is allowed to have.
///
/// `target_framerate` should be the number of frames per second to target.
///
/// # Panics
///
/// This function panics if `charset` is empty (i.e. has a length of zero).
///
/// This function panics if `target_framerate` is zero.
///
/// # Examples
/// ```no_run
/// use mrs_matrix::animation::anim_loop;
/// use mrs_matrix::raindrop::charsets::{Charset, PrintableAscii};
/// use mrs_matrix::raindrop::color_algorithms::LightnessDescending;
/// use mrs_matrix::raindrop::RaindropSpeed;
///
/// pub fn main() -> std::io::Result<()>
/// {
///     let charset = PrintableAscii().get_charset();
///     let color_algorithm = LightnessDescending::try_new(118.0, 0.82).unwrap().into();
///     let speed = RaindropSpeed::Constant(0.75);
///     let target_framerate = 25;
///     let mut rng = rand::rng();
///     anim_loop(&mut rng, &charset, color_algorithm, speed, target_framerate)
/// }
/// ```
pub fn anim_loop<T: RngExt>(
    rng: &mut T,
    charset: &[char],
    color_algorithm: ColorAlgorithm,
    allowed_speeds: RaindropSpeed,
    target_framerate: usize
) -> std::io::Result<()> {
    assert!(
        !charset.is_empty(),
        "cannot run anim_loop with empty character set"
    );
    assert!(
        target_framerate > 0,
        "cannot run anim_loop at target framerate of zero"
    );

    let mut out = stdout();

    let (mut term_cols, mut term_rows) = terminal::size()?;

    //enable raw mode to process keypress by keypress
    terminal::enable_raw_mode()?;

    //enter alternate screen, and hide the cursor
    out.queue(terminal::EnterAlternateScreen)?
        .queue(cursor::Hide)?;

    //calculate target frame duration by dividing one second by the number of frames that should be in one second
    let target_frame_duration = Duration::from_secs_f64(1.0 / (target_framerate as f64));

    let mut raindrop_vector = create_raindrops(
        rng,
        charset,
        color_algorithm.clone(),
        allowed_speeds.clone(),
        term_cols,
        term_rows
    );

    let mut start_instant: Instant;
    loop {
        start_instant = Instant::now();

        //reset cursor position
        out.queue(cursor::MoveTo(0, 0))?;

        //iterate through all rows
        for row_index in 0..term_rows {
            out.queue(cursor::MoveToRow(row_index))?
                .queue(cursor::MoveToColumn(0))?;

            //iterate through all columns by iterating through raindrop_vector, printing styled chars where applicable
            //note that spaces are printed for columns on this row without a printable char
            for raindrop in raindrop_vector.iter_mut() {
                match raindrop.get_styled_char_at_row(rng, row_index) {
                    None => out.queue(Print(" "))?,
                    Some(styled_char) => out.queue(PrintStyledContent(styled_char))?
                };
            }
        }

        //flush buffer to 'draw'
        out.flush()?;

        //call advance_animation on all the raindrops
        for raindrop in raindrop_vector.iter_mut() {
            raindrop.advance_animation(rng, term_rows);
        }

        //wait for enough time to hit target_frame_duration, or no time if frame duration exceeds target
        if event::poll(target_frame_duration.saturating_sub(Instant::now() - start_instant))? {
            match event::read()? {
                //upon recieving a resize event set new column amount
                Event::Resize(new_cols, new_rows) => {
                    term_cols = new_cols;
                    term_rows = new_rows;

                    raindrop_vector = create_raindrops(
                        rng,
                        charset,
                        color_algorithm.clone(),
                        allowed_speeds.clone(),
                        term_cols,
                        term_rows
                    );
                }
                //ignore focus and key release events
                //documentation for crossterm indicates that these aren't captured by default, but it seems they're captured here anyway
                Event::FocusGained
                | Event::FocusLost
                | Event::Key(KeyEvent {
                    kind: KeyEventKind::Release,
                    ..
                }) => (),

                //stop loop upon recieving any other event
                _ => {
                    break;
                }
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
