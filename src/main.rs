use ggez::{
    conf::WindowMode,
    event::{self, EventHandler, MouseButton},
    graphics::{self, Color, DrawMode, Mesh, Rect, Text},
    mint::Point2,
    timer, Context, ContextBuilder, GameError, GameResult,
};

const CELL_SIZE: (f32, f32) = (20.0, 20.0);
const GRID_SIZE: (f32, f32) = (40.0, 40.0);
const WINDOW_SIZE: (f32, f32) = (CELL_SIZE.0 * GRID_SIZE.0, CELL_SIZE.1 * GRID_SIZE.1);
const BG_COLOR: Color = Color::WHITE;
const CELL_COLOR: Color = Color::BLACK;
const LINE_WIDTH: f32 = 2.0;
const LINE_COLOR: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};
const TEXT_COLOR: Color = Color::BLACK;

struct State {
    grid: Vec<Vec<bool>>,
    fps: u32,
    running: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            grid: vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize],
            fps: 1,
            running: false,
        }
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, self.fps) && self.running {
            let mut coords: Vec<(usize, usize)> = vec![];

            for i in 0..GRID_SIZE.0 as usize {
                let left = if i > 0 {
                    i - 1
                } else {
                    GRID_SIZE.0 as usize - 1
                };
                let right = if i < GRID_SIZE.0 as usize - 1 {
                    i + 1
                } else {
                    0
                };
                for j in 0..GRID_SIZE.1 as usize {
                    let up = if j > 0 {
                        j - 1
                    } else {
                        GRID_SIZE.1 as usize - 1
                    };
                    let down = if j < GRID_SIZE.1 as usize - 1 {
                        j + 1
                    } else {
                        0
                    };

                    let neighbors = self.grid[left][j] as u8
                        + self.grid[left][up] as u8
                        + self.grid[i][up] as u8
                        + self.grid[right][up] as u8
                        + self.grid[right][j] as u8
                        + self.grid[right][down] as u8
                        + self.grid[i][down] as u8
                        + self.grid[left][down] as u8;

                    if (self.grid[i][j] && (neighbors < 2 || neighbors > 3))
                        || (!self.grid[i][j] && neighbors == 3)
                    {
                        coords.push((i, j));
                    }
                }
            }

            for coord in coords {
                self.grid[coord.0][coord.1] ^= true;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BG_COLOR);

        for i in 0..GRID_SIZE.0 as usize {
            for j in 0..GRID_SIZE.1 as usize {
                if self.grid[i][j] {
                    let rect = Mesh::new_rectangle(
                        ctx,
                        DrawMode::fill(),
                        Rect::new(
                            i as f32 * CELL_SIZE.0,
                            j as f32 * CELL_SIZE.1,
                            CELL_SIZE.0,
                            CELL_SIZE.1,
                        ),
                        CELL_COLOR,
                    )?;
                    graphics::draw(ctx, &rect, (Point2 { x: 0.0, y: 0.0 },))?;
                }

                if j == 0 {
                    continue;
                }

                let line = Mesh::new_line(
                    ctx,
                    &vec![
                        Point2 {
                            x: 0.0,
                            y: j as f32 * CELL_SIZE.1,
                        },
                        Point2 {
                            x: WINDOW_SIZE.0,
                            y: j as f32 * CELL_SIZE.1,
                        },
                    ],
                    LINE_WIDTH,
                    LINE_COLOR,
                )?;
                graphics::draw(ctx, &line, (Point2 { x: 0.0, y: 0.0 },))?;
            }

            if i == 0 {
                continue;
            }

            let line = Mesh::new_line(
                ctx,
                &vec![
                    Point2 {
                        x: i as f32 * CELL_SIZE.0,
                        y: 0.0,
                    },
                    Point2 {
                        x: i as f32 * CELL_SIZE.0,
                        y: WINDOW_SIZE.1,
                    },
                ],
                LINE_WIDTH,
                LINE_COLOR,
            )?;
            graphics::draw(ctx, &line, (Point2 { x: 0.0, y: 0.0 },))?;
        }

        let text = Text::new(self.fps.to_string());
        graphics::draw(ctx, &text, (Point2 { x: 0.0, y: 2.0 }, TEXT_COLOR))?;

        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.grid[(x / CELL_SIZE.0).floor() as usize][(y / CELL_SIZE.1).floor() as usize] ^= true;
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        repeat: bool,
    ) {
        if keycode == event::KeyCode::Space && !repeat {
            self.running ^= true;
        }
        if keycode == event::KeyCode::Up {
            self.fps += 1;
        }
        if keycode == event::KeyCode::Down && self.fps > 1 {
            self.fps -= 1;
        }
        if keycode == event::KeyCode::Delete {
            self.grid = vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize];
        }
    }
}

fn main() -> GameResult {
    let state = State::new();

    let (ctx, event_loop) = ContextBuilder::new("Conway's Game of Life", "ezza")
        .window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .build()?;

    event::run(ctx, event_loop, state);
}
