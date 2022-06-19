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
use rand::{self,rngs};
use crate::raindrop::Raindrop;

/// Returns a `Vec<Raindrop>` with one `Raindrop` for each terminal column
/// 
/// `terminal_width` should be the width of the terminal in columns
/// 
/// `terminal_height` should be the height of the terminal in rows
/// 
/// Note that this function is intentionally private because it's unlikely to be generally useful
fn create_raindrops(terminal_width: u16, terminal_height: u16) -> Vec<Raindrop<rngs::ThreadRng>>
{
    let mut raindrop_vec: Vec<Raindrop<rngs::ThreadRng>> = Vec::with_capacity(terminal_width.into());

    for _ in 0..terminal_width {
        let new_rng = rand::thread_rng();
        raindrop_vec.push(Raindrop::new(new_rng, terminal_height));
    }

    raindrop_vec
}

/// The main loop that renders the screen
/// 
/// Returns after receiving any keypress
/// 
/// `target_framerate` should be the number of frames per second to target
/// 
/// # Panics
/// 
/// This function panics if `target_framerate` is zero
pub fn anim_loop(target_framerate: usize) -> crossterm::Result<()>
{
    
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

    let mut raindrop_vector = create_raindrops(term_cols, term_rows);

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

                    raindrop_vector = create_raindrops(term_cols, term_rows);
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