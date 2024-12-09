use crossterm::{terminal::{self,Clear,ClearType},execute,cursor::{self,Hide,Show}};
use std::io::{self,Write};
use tic_tac_toe_advanced::screen::choose_player;
fn main()->io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout,Hide)?;
    loop{
        execute!(stdout,cursor::MoveTo(0,0),Clear(ClearType::All),Clear(ClearType::Purge))?;
        choose_player()?;
        break;
    }
    execute!(stdout,cursor::MoveTo(0,0),Clear(ClearType::All),Clear(ClearType::Purge))?;
    stdout.flush()?;
    execute!(stdout,Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
