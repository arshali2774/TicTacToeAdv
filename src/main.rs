use std::io::{self, Write};

use crossterm::{
    cursor::{self, Hide, Show},
    event, execute,
    terminal::{self, Clear, ClearType},
};

use std::time::Duration;

use tic_tac_toe_advanced::screen;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    // hide cursor
    execute!(stdout, Hide)?;

    loop {
        // clear screen
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            Clear(ClearType::Purge)
        )?;

        stdout.flush()?;

        let (human_player, continue_game) = screen::choose_player()?;
        drain_events()?;

        if !continue_game {
            break;
        }

        let (winning_player, board, continue_game) = screen::game_play(&human_player)?;
        drain_events()?;

        if !continue_game {
            break;
        }

        let continue_game = screen::end_menu(&winning_player, &human_player, &board)?;
        drain_events()?;

        if !continue_game {
            break;
        }
    }

    // clear screen
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All),
        Clear(ClearType::Purge)
    )?;

    // display cursor
    execute!(stdout, Show)?;

    stdout.flush()?;

    terminal::disable_raw_mode()?;

    Ok(())
}

fn drain_events() -> io::Result<()> {
    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?;
    }

    Ok(())
}
