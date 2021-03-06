use crate::ai::agent::Agent;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerColor {
    Red,
    Blue,
}

impl PlayerColor {
    #[inline]
    pub fn switch(&mut self) {
        *self = match self {
            PlayerColor::Red => PlayerColor::Blue,
            PlayerColor::Blue => PlayerColor::Red,
        };
    }

    #[inline]
    pub fn enemy(&self) -> PlayerColor {
        match self {
            PlayerColor::Red => PlayerColor::Blue,
            PlayerColor::Blue => PlayerColor::Red,
        }
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PlayerType {
    Human,
    Random,
    AlphaBeta,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub agent: Box<dyn Agent>,
    pub player_type: PlayerType,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub players: [Player; 2],
    pub turn: u16,
    pub current_player_idx: usize,
    pub curr_color: PlayerColor,
}

impl GameState {
    pub fn new(red_player: Player, blue_player: Player) -> Self {
        Self {
            players: [red_player, blue_player],
            turn: 0,
            current_player_idx: 0,
            curr_color: PlayerColor::Red,
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.turn = 0;
        self.current_player_idx = 0;
        self.curr_color = PlayerColor::Red;
    }

    #[inline]
    pub fn get_current_player(&self) -> &Player {
        &self.players[self.current_player_idx]
    }

    #[inline]
    pub fn next_turn(&mut self) {
        self.turn += 1;
        self.curr_color.switch();
        self.current_player_idx = (self.current_player_idx + 1) % 2;
    }

    #[inline]
    pub fn undo_next_turn(&mut self) {
        self.turn -= 1;
        self.curr_color.switch();
        self.current_player_idx = (self.current_player_idx + 1) % 2;
    }
}
