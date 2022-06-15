use std::io::{stdout};
use crossterm::{self, ExecutableCommand, style::{Stylize, PrintStyledContent}, terminal, cursor};
use coolor::{Color,Hsl};

fn main() -> crossterm::Result<()> {

    let mut out = stdout();

    let (term_cols, _term_rows) = terminal::size()?;

    //get middle column index by dividing the term_cols by 2 (rounding up), subtracting 1 for zero based indexing
    let mid_col =((term_cols + 1) / 2) - 1;

    println!("writing to column index {} within a terminal {} columns wide", mid_col, term_cols);

    out.execute(cursor::MoveToColumn(mid_col))?
        .execute(PrintStyledContent("H".with(
            Color::from(Hsl::new(118.0, 0.82, 0.50)).into()
        )))?;

    Ok(())
}