#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct State {
    score:      i32,
    frame_rate: f32,
    player:     Player,
    obsticle:   Obsticle,
    mode:       GameMode
}

enum GameMode {
    Menu,
    Playing,
    End
}

impl State {
    fn new() -> Self {
        Self {
            score:      0,
            frame_rate: 0.0,
            player:     Player::new(5, 25),
            obsticle:   Obsticle::new(SCREEN_WIDTH, 0),
            mode:       GameMode::Menu
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_rate += ctx.frame_time_ms;
        if self.frame_rate > FRAME_DURATION {
            self.frame_rate = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, format!("Score {}", self.score));
        self.obsticle.render(ctx, self.player.x);
        if self.player.x > self.obsticle.x {
            self.score += 1;
            self.obsticle = Obsticle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obsticle.hit_obsticle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self, _ctx: &mut BTerm) {
        self.player = Player::new(5, 25);
        self.frame_rate = 0.0;
        self.mode = GameMode::Playing;
        self.obsticle = Obsticle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

    fn menu_mode(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, format!("You are earned {} points!", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.menu_mode(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx)
        }
    }
}

struct Player {
    x:        i32,
    y:        i32,
    velocity: f32
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    #[allow(clippy::cast_possible_truncation)]
    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct Obsticle {
    x:     i32,
    gap_y: i32,
    size:  i32
}

impl Obsticle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score)
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obsticle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
