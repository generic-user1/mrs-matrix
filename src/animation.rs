//! Functions relating directly to drawing animations on the screen
//! 
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
use crate::raindrop::Raindrop;


/// The main loop that renders the screen (WIP)
/// 
/// Returns after receiving any keypress
/// 
/// Currently just a demo
pub fn anim_loop() -> crossterm::Result<()>
{
    let mut out = stdout();

    let mut term_cols = terminal::size()?.0;

    let char_color = Color::from(Hsl::new(118.0, 0.82, 0.50));
    let styled_char = "H".with(char_color.into());

    //enable raw mode to process keypress by keypress
    terminal::enable_raw_mode()?;

    //enter alternate screen, hide the cursor, clear the screen, and reset the cursor position
    out.queue(terminal::EnterAlternateScreen)?
    .queue(cursor::Hide)?

    //these last two are needed to get to a known state because 
    //the alternate screen buffer may be persistent across different sessions of this program
    .queue(terminal::Clear(ClearType::All))?
    .queue(cursor::MoveTo(0,0))?;

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
            if current_column >= term_cols{
                //cap current_column at its max value for right to left
                current_column = term_cols - 1;
                left_to_right = false;
            }
        } else {            
            //move left one space
            out.queue(cursor::MoveLeft(1))?
            //print a char (which will move the cursor right 1 space),
            .queue(PrintStyledContent(styled_char))?
            //then move left another space
            .queue(cursor::MoveLeft(1))?;


            //decrement current column, and reverse direction if at minimum
            current_column -= 1;
            if current_column <= 0 {
                //cap current_column at its min value for left to right
                current_column = 0;
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

//todo: remove this demo entirely
/// TEMPORARY; WILL BE REMOVED
pub fn raindrop_demo() -> crossterm::Result<()>
{
    let mut out = stdout();

    let termheight = terminal::size()?.1;

    let mut test_drop = Raindrop::new(rand::thread_rng(), termheight);
    
    test_drop.set_height(termheight - 5);


    let mut printclosure = || -> crossterm::Result<()> {
        let mut out = stdout();
        
        out.queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0,0))?;

        for row_index in 0..termheight
        {
            if let Some(styled_char) = test_drop.get_styled_char_at_row(row_index){
                out.queue(PrintStyledContent(styled_char))?
                .queue(cursor::MoveLeft(1))?;
            }
            out.queue(cursor::MoveDown(1))?;
            
        }
        out.flush()?;
        Ok(())
    };

    printclosure()?;

    std::thread::sleep(std::time::Duration::from_millis(5000));

    out.flush()?;

    printclosure()?;
    std::thread::sleep(std::time::Duration::from_millis(5000));


    Ok(())
}