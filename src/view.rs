use piston_window::*;

pub struct ProjectileView {
    x: f64,
    y: f64,
}

impl ProjectileView {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x: x, y: y }
    }
    pub fn mirror(&self) -> Self {
        Self { x: 100.0 - self.x, y: self.y }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        rectangle(
            [0.5, 0.75, 0.5, 1.0],
            [self.x * 5.0 - 5.0, 200.0 - 5.0 - self.y * 5.0, 10.0, 10.0],
            c.transform,
            g,
        )
    }
}

pub enum View {
    Lobby { is_left_ready: bool, is_right_ready: bool },
    GetReady { countdown: f64 },
    Game {
        left: f64,
        right: f64,
        projectiles: Vec<ProjectileView>
    },
    End { is_left_dead: bool, is_right_dead: bool },
}

impl View {
    pub fn render(self, c: Context, g: &mut G2d) {
        clear([0.5, 0.5, 0.5, 1.0], g);
        match self {
            View::Lobby { is_left_ready, is_right_ready } => {
                // left
                rectangle(
                    if is_left_ready {
                        [0.5, 0.0, 0.0, 1.0]
                    } else {
                        [0.75, 0.5, 0.5, 1.0]
                    },
                    [25.0, 200.0 - 25.0, 50.0, 50.0],
                    c.transform,
                    g,
                );
                // right
                rectangle(
                    if is_right_ready {
                        [0.0, 0.0, 0.5, 1.0]
                    } else {
                        [0.5, 0.5, 0.75, 1.0]
                    },
                    [500.0 - 25.0 - 50.0, 200.0 - 25.0, 50.0, 50.0],
                    c.transform,
                    g,
                );
            },
            View::GetReady { countdown } => {
                // left
                rectangle(
                    [0.5, 0.0, 0.0, 1.0],
                    [25.0, 200.0 - 25.0, 50.0, 50.0],
                    c.transform,
                    g,
                );
                // right
                rectangle(
                    [0.0, 0.0, 0.5, 1.0],
                    [500.0 - 25.0 - 50.0, 200.0 - 25.0, 50.0, 50.0],
                    c.transform,
                    g,
                );
                // countdown
                rectangle(
                    [0.75, 0.5, 0.75, 1.0],
                    [10.0, 10.0, countdown * 160.0, 10.0],
                    c.transform,
                    g,
                );
            },

            View::Game { left, right, projectiles } => {
                // left
                rectangle(
                    [0.5, 0.0, 0.0, 1.0],
                    [25.0, 200.0 - 25.0 - left * 5.0, 50.0, 50.0],
                    c.transform,
                    g,
                );
                // right
                rectangle(
                    [0.0, 0.0, 0.5, 1.0],
                    [500.0 - 25.0 - 50.0, 200.0 - 25.0 - right * 5.0, 50.0, 50.0],
                    c.transform,
                    g,
                );
                // projectiles
                for projectile in projectiles.iter() {
                    projectile.render(c, g);
                }
            },
            View::End { is_left_dead, is_right_dead} => {
                // left
                rectangle(
                    if is_left_dead {
                        [0.25, 0.0, 0.0, 1.0]
                    } else {
                        [0.5, 0.0, 0.0, 1.0]
                    },
                    [25.0, 200.0 - 25.0, 50.0, 50.0],
                    c.transform,
                    g,
                );
                // right
                rectangle(
                    if is_right_dead {
                        [0.0, 0.0, 0.25, 1.0]
                    } else {
                        [0.0, 0.0, 0.5, 1.0]
                    },
                    [500.0 - 25.0 - 50.0, 200.0 - 25.0, 50.0, 50.0],
                    c.transform,
                    g,
                );
            },
        }
    }
}
