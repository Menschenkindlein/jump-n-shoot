use std::mem;
use view::ProjectileView;

const PROJECTILE_SPEED: f64 = 75.0;
const ACCELERATION: f64 = 40.0;

struct Jump {
    position: f64,
    speed: f64,
}

pub struct Player {
    jump: Option<Jump>,
    projectiles: Vec<Projectile>,
}

struct Projectile {
    x: f64,
    y: f64,
}

impl Player {
    pub fn new() -> Self {
        Player {
            jump: None,
            projectiles: Vec::new(),
        }
    }

    // return if the player is dead
    pub fn update(&mut self, dt: f64) -> bool {
        if let Some(jump) = mem::replace(&mut self.jump, None) {
            let new_position = jump.position + jump.speed * dt;
            if new_position < 0.0 {
                self.jump = None;
            } else {
                let new_speed = jump.speed - ACCELERATION * dt;
                self.jump = Some(Jump { position: new_position, speed: new_speed });
            }
        }
        let mut is_dead = false;
        let y = self.get_position();
        for projectile in self.projectiles.iter_mut() {
            if projectile.y < y + 5.0 && projectile.y > y - 5.0 && projectile.x < 95.0 && projectile.x > 85.0 {
                is_dead = true;
            }
            projectile.x += PROJECTILE_SPEED * dt;
        }
        self.projectiles.retain(|p| p.x < 100.0);

        is_dead
    }

    pub fn jump(&mut self) {
        if self.jump.is_none() {
            self.jump = Some(Jump { position: 0.0, speed: ACCELERATION });
        }
    }

    pub fn receive_fire(&mut self, position: f64) {
        self.projectiles.push(Projectile { x: 10.0, y: position });
    }

    pub fn get_position(&self) -> f64 {
        if let Some(ref jump) = self.jump {
            jump.position
        } else {
            0.0
        }
    }

    pub fn view_projectiles(&self) -> Vec<ProjectileView> {
        self.projectiles.iter().map(|p| ProjectileView::new(p.x, p.y)).collect()
    }
}
