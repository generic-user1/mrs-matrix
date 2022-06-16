use std::io::{stdout, Write};
use std::time::{Instant, Duration};
use coolor::{Color,Hsl};
use crossterm::{
    self,
    event::{self, Event},
    QueueableCommand, 
    style::{Stylize, PrintStyledContent},
    terminal::{self, ClearType}, 
    cursor
};



fn main() -> crossterm::Result<()> {

    screen_loop()

}

/// The main loop that renders the screen
/// 
/// Returns after receiving any keypress
fn screen_loop() -> crossterm::Result<()>
{
    let mut out = stdout();

    let mut term_cols = terminal::size()?.0;

    let char_color = Color::from(Hsl::new(118.0, 0.82, 0.50));
    let styled_char = "H".with(char_color.into());

    //enable raw mode to process keypress by keypress
    terminal::enable_raw_mode()?;

    //enter alternate screen and hide the cursor
    out.queue(terminal::EnterAlternateScreen)?
    .queue(cursor::Hide)?;

    const TARGET_FRAME_DURATION: Duration = Duration::from_nanos(16_666_666);

    let mut current_column: u16 = 0;
    let mut left_to_right = true;
    let mut start_instant: Instant;
    loop {
        start_instant = Instant::now();
        out.queue(terminal::Clear(ClearType::CurrentLine))?;
        if left_to_right {
            out.queue(PrintStyledContent(styled_char))?;

            //increment current column, and reverse direction if at maximum
            current_column += 1;
            if current_column >= (term_cols - 1){
                left_to_right = false;
            }
        } else {
            out.queue(cursor::MoveLeft(2))?
            .queue(PrintStyledContent(styled_char))?;

            //decrement current column, and reverse direction if at minimum
            current_column -= 1;
            if current_column <= 0 {
                left_to_right = true;
            }
        }
        out.flush()?;
    
        //wait for enough time to hit TARGET_FRAME_DURATION, or no time if frame duration exceeds target
        if event::poll(TARGET_FRAME_DURATION.saturating_sub(Instant::now() - start_instant))? {
            match event::read()? {
                //upon recieving a resize event set new column amount
                Event::Resize(new_cols, _) => {
                    term_cols = new_cols;
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