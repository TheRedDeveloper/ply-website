use ply_engine::prelude::*;
use std::cell::RefCell;

const GRID_W: i32 = 20;
const GRID_H: i32 = 15;
const TICK_RATE: f64 = 0.12;
const CORNER: f32 = 12.0;
const HUE_STEP: f32 = 12.0;

#[derive(Clone, Copy, PartialEq)]
enum Dir { Up, Down, Left, Right }

impl Dir {
  fn delta(self) -> (i32, i32) {
    match self {
      Dir::Up    => ( 0, -1),
      Dir::Down  => ( 0,  1),
      Dir::Left  => (-1,  0),
      Dir::Right => ( 1,  0),
    }
  }

  fn opposite(self) -> Self {
    match self {
      Dir::Up    => Dir::Down,
			Dir::Down  => Dir::Up,
      Dir::Left  => Dir::Right,
			Dir::Right => Dir::Left,
    }
  }

  fn from_delta(dx: i32, dy: i32) -> Self {
    match (dx, dy) {
      ( 0, -1) => Dir::Up,
      ( 0,  1) => Dir::Down,
      (-1,  0) => Dir::Left,
      ( 1,  0) => Dir::Right,
      _ => unreachable!(),
    }
  }
}

struct Game {
  snake: Vec<(i32, i32)>,
  dir: Dir,
  next_dir: Dir,
  food: (i32, i32),
  score: u32,
  game_over: bool,
  last_tick: f64,
}

impl Game {
  fn new() -> Self {
    let cx = GRID_W / 2;
    let cy = GRID_H / 2;
    let mut g = Self {
      snake: vec![(cx, cy), (cx - 1, cy), (cx - 2, cy)],
      dir: Dir::Right,
      next_dir: Dir::Right,
      food: (0, 0),
      score: 0,
      game_over: false,
      last_tick: 0.0,
    };
    g.spawn_food();
    g
  }

  fn spawn_food(&mut self) {
    loop {
      let pos = (rand::gen_range(0, GRID_W), rand::gen_range(0, GRID_H));
      if !self.snake.contains(&pos) {
        self.food = pos;
        break;
      }
    }
  }

  fn tick(&mut self) {
    if self.game_over { return; }
    self.dir = self.next_dir;
    let (dx, dy) = self.dir.delta();
    let (hx, hy) = self.snake[0];
    let (nx, ny) = (hx + dx, hy + dy);

    if nx < 0 || nx >= GRID_W || ny < 0 || ny >= GRID_H
      || self.snake.contains(&(nx, ny))
    {
      self.game_over = true;
      return;
    }

    self.snake.insert(0, (nx, ny));
    if (nx, ny) == self.food {
      self.score += 1;
      self.spawn_food();
    } else {
      self.snake.pop();
    }
  }

  fn set_dir(&mut self, d: Dir) {
    if d != self.dir.opposite() {
      self.next_dir = d;
    }
  }
}

thread_local! {
  static GAME: RefCell<Game> = RefCell::new(Game::new());
}

fn hsl(h: f32, s: f32, l: f32) -> Color {
  let h = ((h % 360.0) + 360.0) % 360.0;
  let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
  let x = c * (1.0 - (h / 60.0 % 2.0 - 1.0).abs());
  let (r1, g1, b1) = match (h / 60.0) as u32 {
    0 => (c, x, 0.0),
    1 => (x, c, 0.0),
    2 => (0.0, c, x),
    3 => (0.0, x, c),
    4 => (x, 0.0, c),
    5 => (c, 0.0, x),
		_ => unreachable!(),
  };
  let m = l - c / 2.0;
  ((((r1 + m) * 255.0) as u8, ((g1 + m) * 255.0) as u8, ((b1 + m) * 255.0) as u8)).into()
}

/// Round the leading face of the head.
fn head_corners(dir: Dir) -> (f32, f32, f32, f32) {
  let r = CORNER;
  match dir {  // (tl,  tr,  br,  bl )
    Dir::Right => (0.0, r,   r,   0.0),
    Dir::Left  => (r,   0.0, 0.0, r  ),
    Dir::Up    => (r,   r,   0.0, 0.0),
    Dir::Down  => (0.0, 0.0, r,   r  ),
  }
}

/// Round the trailing face of the tail.
fn tail_corners(dir: Dir) -> (f32, f32, f32, f32) {
  let r = CORNER;
  match dir {  // (tl,  tr,  br,  bl )
    Dir::Right => (0.0, r,   r,   0.0),
    Dir::Left  => (r,   0.0, 0.0, r  ),
    Dir::Up    => (r,   r,   0.0, 0.0),
    Dir::Down  => (0.0, 0.0, r,   r  ),
  }
}

/// Round the outside corner at a turn.
fn turn_corners(enter: Dir, exit: Dir) -> (f32, f32, f32, f32) {
  let r = CORNER;
  match (enter, exit) {      // (tl,  tr,  br,  bl )
    (Dir::Down,  Dir::Right) => (0.0, 0.0, 0.0, r  ),
    (Dir::Down,  Dir::Left)  => (0.0, 0.0, r,   0.0),
    (Dir::Up,    Dir::Right) => (r,   0.0, 0.0, 0.0),
    (Dir::Up,    Dir::Left)  => (0.0, r,   0.0, 0.0),
    (Dir::Right, Dir::Down)  => (0.0, r,   0.0, 0.0),
    (Dir::Right, Dir::Up)    => (0.0, 0.0, r,   0.0),
    (Dir::Left,  Dir::Down)  => (r,   0.0, 0.0, 0.0),
    (Dir::Left,  Dir::Up)    => (0.0, 0.0, 0.0, r  ),
    _                        => (0.0, 0.0, 0.0, 0.0),
  }
}

pub fn run(ui: &mut Ui<'_, ()>) {
  let (snake, dir, food, score, game_over) = GAME.with(|g| {
    let mut game = g.borrow_mut();

    for (key, dir) in [
      (KeyCode::Up, Dir::Up), (KeyCode::W, Dir::Up),
      (KeyCode::Down, Dir::Down), (KeyCode::S, Dir::Down),
      (KeyCode::Left, Dir::Left), (KeyCode::A, Dir::Left),
      (KeyCode::Right, Dir::Right), (KeyCode::D, Dir::Right),
    ] {
      if is_key_pressed(key) { game.set_dir(dir); }
    }

    if game.game_over && is_key_pressed(KeyCode::Space) {
      *game = Game::new();
    }

    let now = get_time();
    if now - game.last_tick >= TICK_RATE {
      game.last_tick = now;
      game.tick();
    }

    (game.snake.clone(), game.dir, game.food, game.score, game.game_over)
  });

  let len = snake.len();

  ui.element()
    .width(grow!())
    .height(grow!())
    .background_color(0x1E1B1B)
    .layout(|l| l.align(CenterX, CenterY))
    .children(|ui| {
      ui.element()
        .width(percent!(0.5))
        .aspect_ratio(GRID_W as f32 / GRID_H as f32)
        .background_color(0x262220)
        .corner_radius(4.0)
        .layout(|l| l.direction(TopToBottom))
        .children(|ui| {
          for y in 0..GRID_H {
            ui.element()
              .width(grow!())
              .height(grow!())
              .children(|ui| {
                for x in 0..GRID_W {
                  let pos = (x, y);
                  let idx = snake.iter().position(|&s| s == pos);
                  let is_food = pos == food;

                  let bg: Color = match idx {
                    Some(0) => 0x6ECB63.into(),
                    Some(i) => hsl((i - 1) as f32 * HUE_STEP, 0.5, 0.55),
                    None if is_food          => 0xFF654D.into(),
                    None if (x + y) % 2 == 0 => 0x2A2725.into(),
                    _                        => 0x262220.into(),
                  };

                  let radius = match idx {
                    Some(0) => head_corners(dir),
                    Some(i) if i == len - 1 && len > 1 => {
                      let (px, py) = snake[len - 2];
                      let (tx, ty) = snake[len - 1];
                      tail_corners(Dir::from_delta(tx - px, ty - py))
                    }
                    Some(i) => {
                      let (px, py) = snake[i - 1];
                      let (cx, cy) = snake[i];
                      let (nx, ny) = snake[i + 1];
                      let enter = Dir::from_delta(cx - px, cy - py);
                      let exit = Dir::from_delta(nx - cx, ny - cy);
                      turn_corners(enter, exit)
                    }
                    None if is_food => (f32::MAX, f32::MAX, f32::MAX, f32::MAX),
                    _ => (0.0, 0.0, 0.0, 0.0),
                  };

                  ui.element()
                    .width(grow!())
                    .height(grow!())
                    .background_color(bg)
                    .corner_radius(radius)
                    .empty();
                }
              });
          }

          // Score badge
          ui.element()
            .width(fit!())
            .height(fit!())
            .background_color((30, 27, 27, 200))
            .corner_radius(12.0)
            .floating(|f| f.attach_parent().anchor((CenterX, Top), (CenterX, Top)).offset(0.0, 8.0).passthrough())
            .layout(|l| l.padding((4, 14, 4, 14)))
            .children(|ui| {
              ui.text(&format!("{score}"), |t| t.font_size(22).color(0xE8E0DC));
            });

          if game_over {
            ui.element()
              .width(fit!())
              .height(fit!())
              .background_color((30, 27, 27, 220))
              .corner_radius(8.0)
              .floating(|f| f.attach_parent().anchor((CenterX, CenterY), (CenterX, CenterY)).passthrough())
              .layout(|l| l.padding((12, 20, 12, 20)).direction(TopToBottom).gap(4).align(CenterX, CenterY))
              .children(|ui| {
                ui.text("Game Over", |t| t.font_size(22).color(0xFF654D));
                ui.text("Press Space to restart", |t| t.font_size(14).color(0x9E9590));
              });
          } else {
            ui.element()
              .width(fit!())
              .height(fit!())
              .background_color((30, 27, 27, 160))
              .corner_radius(8.0)
              .floating(|f| f.attach_parent().anchor((CenterX, Bottom), (CenterX, Bottom)).offset(0.0, -8.0).passthrough())
              .layout(|l| l.padding((4, 10, 4, 10)))
              .children(|ui| {
                ui.text("Arrow keys or WASD", |t| t.font_size(12).color(0x6E6560));
              });
          }
        });
    });
}
