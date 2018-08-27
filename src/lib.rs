extern crate piston_window;

mod player;
mod view;

use std::mem;
use piston_window::keyboard::Key;
use player::Player;
use view::View;

const COUNTDOWN_TIME: f64 = 3.0;

enum GameState {
    Placeholder,
    Lobby {
        is_left_ready: bool,
        is_right_ready: bool,
    },
    GetReady {
        countdown: f64,
    },
    Game {
        left: Player,
        right: Player,
    },
    End {
        is_left_dead: bool,
        is_right_dead: bool,
    },
}

pub struct App {
    game_state: GameState,
}

impl App {
    pub fn new() -> Self {
        App {
            game_state: GameState::Lobby {
                is_left_ready: false,
                is_right_ready: false,
            },
        }
    }

    pub fn update(&mut self, dt: f64) {
        match mem::replace(&mut self.game_state, GameState::Placeholder) {
            GameState::GetReady { countdown } => {
                if countdown < dt {
                    self.game_state = GameState::Game {
                        left: Player::new(),
                        right: Player::new(),
                    }
                } else {
                    self.game_state = GameState::GetReady {
                        countdown: countdown - dt,
                    }
                }
            }
            GameState::Game {
                mut left,
                mut right,
            } => {
                self.game_state =
                    match (left.update(dt), right.update(dt)) {
                        (false, false) => GameState::Game { left: left, right: right },
                        (is_left_dead, is_right_dead) =>
                            GameState::End {
                                is_left_dead: is_left_dead,
                                is_right_dead: is_right_dead,
                            },
                    }
            }
            other => self.game_state = other,
        }
    }

    pub fn key(&mut self, key: Key) {
        if let GameState::Game { ref mut left, ref mut right } = self.game_state {
            match key {
                Key::W => left.jump(),
                Key::D => right.receive_fire(left.get_position()),
                Key::Up => right.jump(),
                Key::Left => left.receive_fire(right.get_position()),
                _ => (),
            }
        } else {
            match (&self.game_state, key) {
                (&GameState::Lobby { is_right_ready, .. }, Key::W)
                    | (&GameState::Lobby { is_right_ready, .. }, Key::D) => {
                        if is_right_ready {
                            self.game_state = GameState::GetReady {
                                countdown: COUNTDOWN_TIME,
                            }
                        } else {
                            self.game_state = GameState::Lobby {
                                is_left_ready: true,
                                is_right_ready: false,
                            }
                        }
                    }
                (&GameState::Lobby { is_left_ready, .. }, Key::Up)
                    | (&GameState::Lobby { is_left_ready, .. }, Key::Left) => {
                        if is_left_ready {
                            self.game_state = GameState::GetReady {
                                countdown: COUNTDOWN_TIME,
                            }
                        } else {
                            self.game_state = GameState::Lobby {
                                is_left_ready: is_left_ready,
                                is_right_ready: true,
                            }
                        }
                    }
                (&GameState::End { .. }, Key::Space) => {
                    self.game_state = GameState::Lobby {
                        is_left_ready: false,
                        is_right_ready: false,
                    }
                }
                _ => (),
            }
        }
    }

    pub fn view(&self) -> View {
        match self.game_state {
            GameState::Game { ref left, ref right } => {
                let mut projectiles = right.view_projectiles();
                for projectile in left.view_projectiles() {
                    projectiles.push(projectile.mirror());
                }
                View::Game {
                    left: left.get_position(),
                    right: right.get_position(),
                    projectiles: projectiles,
                }
            },
            GameState::Lobby { is_left_ready, is_right_ready } =>
                View::Lobby { is_left_ready: is_left_ready, is_right_ready: is_right_ready },
            GameState::GetReady { countdown } => View::GetReady { countdown: countdown },
            GameState::End { is_left_dead, is_right_dead } => View::End { is_left_dead: is_left_dead, is_right_dead: is_right_dead },
            _ => unimplemented!()
        }
    }
}
