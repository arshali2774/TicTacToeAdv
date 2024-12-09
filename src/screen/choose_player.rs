
use crate::models::Player;
use crate::models::Tabs;
use crossterm::{cursor,execute,terminal::{self,Clear,ClearType},style::{Color,SetForegroundColor,SetBackgroundColor, ResetColor,Print},event::{self,KeyCode,Event}};
use std::io::{self,Write};

pub fn choose_player()->io::Result<(Player,bool)>{
    let mut stdout = io::stdout();

    let mut tabs = Tabs::new(vec![(17,13,Player::X),(22,13,Player::O)]);

    loop{
        execute!(stdout,cursor::MoveTo(0,0),Clear(ClearType::Purge),SetForegroundColor(Color::Cyan))?;
        print_screen();
        execute!(
            stdout,
            cursor::MoveTo(tabs.positions().0,tabs.positions().1),
            SetBackgroundColor(Color::Red),
            Print(tabs.value().char()),
            ResetColor,
            cursor::MoveTo(tabs.positions().0,tabs.positions().1),
        )?;
        stdout.flush()?;
        if let Event::Key(key_event) = event::read()?{
            match key_event.code{
                KeyCode::Tab => tabs.next(),
                KeyCode::BackTab => tabs.prev(),
                KeyCode::Enter => return Ok((tabs.value().clone(),true)),
                KeyCode::Esc => return Ok((tabs.value().clone(),false)),
                _ => continue,
            }
        }
    }

  
    Ok((Player::X,true))
}

fn print_screen(){
    println!(
        "
     \r    +-------- TIC TAC TOE ---------+
     \r    |                              |
     \r    |    USE TAB TO MOVE CURSOR    |
     \r    |                              |
     \r    |       ENTER TO SELECT        |
     \r    |                              |
     \r    |                              |
     \r    |  +-----------------------+   |
     \r    |  |  CHOOSE YOUR PLAYER   |   |
     \r    |  +-----------------------+   |
     \r    |                              |
     \r    |                              |
     \r    |           <X>  <O>           |
     \r    |                              |
     \r    |                              |
     \r    |  PRESS <ESC> TO QUIT GAME    |
     \r    |                              |
     \r    +------------------------------+
     \n\r"
    );
}