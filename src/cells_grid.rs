use crate::cacher::Cacher;
use crate::inp::InputReceived;
use crate::states::{CellDirection, CellState};
use piston_window::*;

pub struct CellsGridGame {
    grid: Vec<Vec<CellState>>,
}

const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 32;

impl CellsGridGame {
    pub fn new(size: Size) -> Self {
        let width = size.width as usize / TILE_WIDTH as usize;
        let height = size.height as usize / TILE_HEIGHT as usize;

        // let all = CellState::get_all_states();
        let grid = vec![vec![CellState::None; width]; height];
        // let grid = grid
        //     .into_iter()
        //     .map(|list| {
        //         list.into_iter()
        //             .map(|_| CellState::get_rand(all.clone()))
        //             .collect()
        //     })
        //     .collect();

        CellsGridGame { grid }
    }

    pub fn render(
        &self,
        win_size: Size,
        cacher: &Cacher,
        c: Context,
        g: &mut G2d,
        _d: &mut GfxDevice,
    ) {
        clear([1.0; 4], g);

        if self.grid.is_empty() {
            return;
        }

        let cell_width = ((win_size.width as usize) / self.grid.len()) as f64;
        let cell_height = ((win_size.height as usize) / self.grid[0].len()) as f64;

        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                let spr_opt = cacher.get(self.grid[x][y].get_sprite_name());
                if let Some(spr) = spr_opt {
                    let trans = c
                        .transform
                        .trans(x as f64 * cell_width, y as f64 * cell_height);
                    image(spr, trans, g);
                }
            }
        }
    }

    pub fn input(&mut self, inp: InputReceived, win_size: Size) {
        if self.grid.is_empty() {
            return;
        }

        use CellDirection::*;
        use CellState::*;
        match inp {
            InputReceived::Mouse(left, (x_real, y_real)) => {
                let (x, y) = self.px_to_grid((x_real, y_real), win_size);
                let cell = self.grid[x][y];

                let width = self.grid.len();
                let height = self.grid[0].len();

                if left {
                    println!("{:?} Clicked at {:?}", cell, (x, y));
                    match cell {
                        Copy(d) => match d {
                            Up | Down => {
                                if y > 0 && y < height - 1 {
                                    if d == Up {
                                        self.change_cell(x, y, self.grid[x][y + 1]);
                                        self.change_cell(x, y - 1, Copy(Up));
                                    } else {
                                        self.change_cell(x, y, self.grid[x][y - 1]);
                                        self.change_cell(x, y + 1, Copy(Down));
                                    }
                                }
                            }
                            Left | Right => {
                                if x > 0 && x < width - 1 {
                                    if d == Left {
                                        self.change_cell(x, y, self.grid[x + 1][y]);
                                        self.change_cell(x - 1, y, Copy(Left));
                                    } else {
                                        self.change_cell(x, y, self.grid[x - 1][y]);
                                        self.change_cell(x + 1, y, Copy(Right));
                                    }
                                }
                            }
                        },
                        Move(d) => match d {
                            Up | Down => {
                                if y > 0 && y < height - 1 {
                                    if d == Up {
                                        if self.grid[x][y - 1] == Trash {
                                            self.change_cell(x, y, None)
                                        } else {
                                            self.change_cell(x, y, self.grid[x][y - 1]);
                                            self.change_cell(x, y - 1, Move(Up));
                                        }
                                    } else {
                                        if self.grid[x][y + 1] == Trash {
                                            self.change_cell(x, y, None)
                                        } else {
                                            self.change_cell(x, y, self.grid[x][y + 1]);
                                            self.change_cell(x, y + 1, Move(Down));
                                        }
                                    }
                                }
                            }
                            Left | Right => {
                                if x > 0 && x < width - 1 {
                                    if d == Left {
                                        if self.grid[x - 1][y] == Trash {
                                            self.change_cell(x, y, None)
                                        } else {
                                            self.change_cell(x, y, self.grid[x - 1][y]);
                                            self.change_cell(x - 1, y, Move(Left));
                                        }
                                    } else {
                                        if self.grid[x - 1][y] == Trash {
                                            self.change_cell(x, y, None)
                                        } else {
                                            self.change_cell(x, y, self.grid[x + 1][y]);
                                            self.change_cell(x + 1, y, Move(Left));
                                        }
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
            InputReceived::Scroll(scroll, (x, y)) => {
                let location = self.px_to_grid((x, y), win_size);
                if scroll > 0.0 {
                    self.grid[location.0][location.1] =
                        self.grid[location.0][location.1].scroll_up();
                } else {
                    self.grid[location.0][location.1] =
                        self.grid[location.0][location.1].scroll_down();
                }
            }
            InputReceived::Keyboard(_) => {
                todo!()
            }
        };
    }

    fn change_cell(&mut self, x: usize, y: usize, new_state: CellState) {
        if self.grid[x][y] != CellState::Trash {
            self.grid[x][y] = new_state;
        }

        //TOOD: Long-Term, Tween movement
    }

    fn px_to_grid(&self, pos: (f64, f64), win_size: Size) -> (usize, usize) {
        if self.grid.is_empty() {
            return (0, 0);
        }
        let cell_width = (win_size.width as usize) / self.grid.len();
        let cell_height = (win_size.height as usize) / self.grid[0].len();

        (pos.0 as usize / cell_width, pos.1 as usize / cell_height)
    }
}
