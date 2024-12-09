#[derive(Clone)]
pub enum Player{
    X,
    O,
}

impl Player{
    pub fn char(&self)->char{
        match self{
            Player::X=> 'X',
            Player::O=> 'O',
        }
    }
    pub fn others(&self)->Player{
        match self{
            Player::X=> Player::O,
            Player::O=> Player::X,
        }
    }
}