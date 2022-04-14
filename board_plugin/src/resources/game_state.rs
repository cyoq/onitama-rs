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
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PlayerType {
    Human,
    Random,
    AlphaBeta,
}

#[derive(Debug)]
pub struct Player<'a> {
    pub agent: &'a dyn Agent,
    pub player_type: PlayerType,
}

#[derive(Debug)]
pub struct GameState<'a> {
    pub players: [Player<'a>; 2],
    pub turn: u16,
    pub current_player_idx: usize,
    pub curr_color: PlayerColor,
}

impl<'a> GameState<'a> {
    pub fn new(first_player: Player<'a>, second_player: Player<'a>) -> Self {
        Self {
            players: [first_player, second_player],
            turn: 0,
            current_player_idx: 0,
            curr_color: PlayerColor::Red,
        }
    }

    #[inline]
    pub fn get_current_player(&self) -> &'a Player {
        &self.players[self.current_player_idx]
    }

    #[inline]
    pub fn next_turn(&mut self) {
        self.turn += 1;
        self.curr_color.switch();
        self.current_player_idx = (self.current_player_idx + 1) % 2;
    }
}
