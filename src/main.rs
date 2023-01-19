#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

//-------------------------------------------------------------------------

use pix_engine::prelude::*;
use das_grid::Grid;

//-------------------------------------------------------------------------

const SPRT_WIDTH: usize = 32;
const SPRT_HEIGHT: usize = 32;

const HORIZONTAL_LINE_THICKNESS: usize = 2;
const VERTICAL_LINE_THICKNESS: usize = 2;

const BOARD_DIM: usize = 3; // Change this to change dimensions on the board

const WIN_WIDTH: usize = (SPRT_WIDTH * BOARD_DIM) + (VERTICAL_LINE_THICKNESS * 2);
const WIN_HEIGHT: usize = (SPRT_HEIGHT * BOARD_DIM) + (HORIZONTAL_LINE_THICKNESS * 2);

//-------------------------------------------------------------------------

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Game {
    Ongoing,
    Over,
}

#[allow(dead_code)]
impl Game {
    const fn is_over(self) -> bool {
        matches!(self, Game::Over)
    }

    const fn is_ongoing(self) -> bool {
        matches!(self, Game::Ongoing)
    }
}

//-------------------------------------------------------------------------

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TicTacSign {
    X,
    O,
    N, // None
}

impl std::ops::Not for TicTacSign {
    type Output = Self;

    fn not(self) -> Self::Output {
        assert_ne!(self, Self::N, "`std::ops::Not` used on `TicTacSign::N` value");

        if self == Self::X {
            Self::O
        } else {
            Self::X
        }
    }
}

#[allow(dead_code)]
impl TicTacSign {
    fn flip_return(&mut self) -> Self {
        *self = !*self;

        *self
    }

    const fn is_x(self) -> bool {
        matches!(self, TicTacSign::X)
    }

    const fn is_o(self) -> bool {
        matches!(self, TicTacSign::O)
    }

    const fn is_n(self) -> bool {
        matches!(self, TicTacSign::N)
    }
}

//-------------------------------------------------------------------------

struct TicTacWin {
    winner: TicTacSign,
    strike: Option<Vec<(i32, i32)>>, // Strike line for when the player wins
}

//-------------------------------------------------------------------------

struct TicTacToe {
    game: Game,
    tic_tac_win: Option<TicTacWin>,
    board: Grid<TicTacSign>,
    player_turn: TicTacSign,
    x_image: Image,
    o_image: Image,
}

impl TicTacToe {
    /// Some(s) -> Some player won
    /// None    -> The game goes on
    fn which_player_won(&self, (x, y): (usize, usize)) -> TicTacWin {
        let x: i32 = x.try_into().unwrap();
        let y: i32 = y.try_into().unwrap();

        let player_cell: TicTacSign = *self.board.get((x, y)).unwrap();
        assert_ne!(player_cell, TicTacSign::N, "Position ({}, {}) contains N, which shouldn't happen", x, y);

        // Check horizontal -
        let strike_horiz: bool =
            self.board.get_col(y).unwrap().into_iter().all(|c| c == player_cell);

        // Check vertical |
        let strike_vert: bool =
            self.board.get_row(x).unwrap().into_iter().all(|c| c == player_cell);

        // NOTE: Diagonal checking is very primitive and only works on 3x3 board

        // Check diagonal \
        let strike_diag: bool = [
            self.board.get((0, 0)).unwrap(),
            self.board.get((1, 1)).unwrap(),
            self.board.get((2, 2)).unwrap()
        ].into_iter().all(|x| *x == player_cell);

        // Check diagonal mirror /
        let strike_diag_mirror: bool = [
            self.board.get((2, 0)).unwrap(),
            self.board.get((1, 1)).unwrap(),
            self.board.get((0, 2)).unwrap()
        ].into_iter().all(|x| *x == player_cell);

        let mut winner = TicTacSign::N;
        let mut strike = None;

        if strike_vert || strike_horiz || strike_diag || strike_diag_mirror {
            winner = player_cell;
            let mut strike_vec = Vec::new();
            strike = if strike_vert {
                for n in 0..self.board.rows() {
                    strike_vec.push((x, n));
                }
                Some(strike_vec)
            } else if strike_horiz {
                for n in 0..self.board.cols() {
                    strike_vec.push((n, y));
                }
                Some(strike_vec)
            } else if strike_diag {
                strike_vec.push((0, 0));
                strike_vec.push((1, 1));
                strike_vec.push((2, 2));
                Some(strike_vec)
            } else if strike_diag_mirror {
                strike_vec.push((2, 0));
                strike_vec.push((1, 1));
                strike_vec.push((0, 2));
                Some(strike_vec)
            } else {
                None
            }
        }

        TicTacWin { winner, strike }
    }
}

impl AppState for TicTacToe {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        // Clear the background and allow transparency
        s.background(Color::BLACK);
        s.blend_mode(BlendMode::Blend);
        Ok(())
    }

    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        if let Key::Escape | Key::Q = &event.key {
            s.quit();
        }

        if let Key::R = &event.key {
            self.board.fill_grid(TicTacSign::N);
            self.game = Game::Ongoing;
            self.tic_tac_win = None;
        }

        if self.game.is_over() {
            return Ok(false);
        }

        let (x, y) = match &event.key {
            Key::Kp1 | Key::Num1 if self.board.get((0, 0)).unwrap().is_n() => (0, 0),
            Key::Kp2 | Key::Num2 if self.board.get((1, 0)).unwrap().is_n() => (1, 0),
            Key::Kp3 | Key::Num3 if self.board.get((2, 0)).unwrap().is_n() => (2, 0),

            Key::Kp4 | Key::Num4 if self.board.get((0, 1)).unwrap().is_n() => (0, 1),
            Key::Kp5 | Key::Num5 if self.board.get((1, 1)).unwrap().is_n() => (1, 1),
            Key::Kp6 | Key::Num6 if self.board.get((2, 1)).unwrap().is_n() => (2, 1),

            Key::Kp7 | Key::Num7 if self.board.get((0, 2)).unwrap().is_n() => (0, 2),
            Key::Kp8 | Key::Num8 if self.board.get((1, 2)).unwrap().is_n() => (1, 2),
            Key::Kp9 | Key::Num9 if self.board.get((2, 2)).unwrap().is_n() => (2, 2),

            _ => return Ok(false),
        };

        *self.board.get_mut((x, y)).unwrap() = self.player_turn.flip_return();

        if self.board.get_flatten_grid().into_iter().all(|c| c != TicTacSign::N) {
            println!("No more cells left! It's a draw!");
            self.game = Game::Over;
            return Ok(false);
        }

        self.tic_tac_win = Some(self.which_player_won((x.try_into().unwrap(), y.try_into().unwrap())));
        match self.tic_tac_win.as_ref().unwrap().winner {
            TicTacSign::X => {
                println!("X won! Game Over!");
                self.game = Game::Over;
            }
            TicTacSign::O => {
                println!("O won! Game Over!");
                self.game = Game::Over;
            }
            TicTacSign::N => {}
        }

        Ok(false) // Don't eat up my events D:
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        for (x, y) in self.board.enumerate() {
            let y_offset: usize = match y {
                0 => 0,
                1 => SPRT_HEIGHT + HORIZONTAL_LINE_THICKNESS,
                2 => (SPRT_HEIGHT + HORIZONTAL_LINE_THICKNESS) * 2,
                //_ => panic!("Wait... That's not the correct amount of rows! Shutting down now!"),
                _ => panic!("Invalid y offset"),
            };

            let x_offset: usize = match x {
                0 => 0,
                1 => SPRT_WIDTH + VERTICAL_LINE_THICKNESS,
                2 => (SPRT_WIDTH + VERTICAL_LINE_THICKNESS) * 2,
                //_ => panic!("Wait... That's not the correct amount of cells! Shutting down now!"),
                _ => panic!("Invalid x offset"),
            };

            // White rectangle that makes the outline of the board
            s.fill(Color::WHITE);
            s.rect([
                x_offset.try_into().unwrap(),
                y_offset.try_into().unwrap(),
                SPRT_WIDTH.try_into().unwrap(),
                SPRT_HEIGHT.try_into().unwrap(),
            ])?;

            s.fill(Color::WHITE);
            match self.board.get((x, y)).unwrap() {
                TicTacSign::N => {}

                TicTacSign::X => s.image(
                    &self.x_image,
                    point![x_offset.try_into().unwrap(), y_offset.try_into().unwrap()],
                )?,

                TicTacSign::O => s.image(
                    &self.o_image,
                    point![x_offset.try_into().unwrap(), y_offset.try_into().unwrap()],
                )?,
            }

            if self.tic_tac_win.is_some() {
                let tic_tac_win = self.tic_tac_win.as_ref().unwrap();
                if tic_tac_win.winner != TicTacSign::N {
                    s.stroke(Color::RED);
                    s.stroke_weight(2);

                    let strike_vec = tic_tac_win.strike.as_ref().unwrap();
                    let cell_width = SPRT_WIDTH as i32;
                    let cell_height = SPRT_HEIGHT as i32;

                    for i in 0..strike_vec.len() - 1 {
                        let (x, y) = strike_vec.get(i).unwrap();
                        let (next_x, next_y) = strike_vec.get(i + 1).unwrap();

                        s.line(Line::new([cell_width / 2 + x * cell_width, cell_height / 2 + y * cell_height],
                                         [cell_width / 2 + next_x * cell_width, cell_height / 2 + next_y * cell_height]))?;
                    }
                    s.no_stroke();
                }
            }
        }
        Ok(())
    }

    fn on_stop(&mut self, _: &mut PixState) -> PixResult<()> {
        // Teardown any state or resources before exiting such as deleting temporary files.
        Ok(())
    }
}

//-------------------------------------------------------------------------

// Clamp Vector2 value with min and max and return a new vector2
// NOTE: Required for virtual mouse, to clamp inside virtual game size
//fn clamp_value(value: (f32, f32),  min: (f32, f32),  max: (f32, f32)) -> (f32, f32) {
//    let mut result = value;
//    result.0 = result.0.max(min.0).min(max.0);
//    result.1 = result.1.max(min.1).min(max.1);
//
//    result
//}

//-------------------------------------------------------------------------

fn main() -> PixResult<()> {
    let mut pix = PixEngine::builder()
        .with_dimensions(WIN_WIDTH.try_into().unwrap(), WIN_HEIGHT.try_into().unwrap())
        .with_title("Tic Tac Toe")
        .with_frame_rate()
        .vsync_enabled()
        .resizable()
        .build()?;

    let x_image = Image::from_file("assets/x.png")?;

    let o_image = Image::from_file("assets/o.png")?;

    let mut tic_tac_toe = TicTacToe {
        game: Game::Ongoing,
        tic_tac_win: None,
        player_turn: TicTacSign::O, // O gets flipped and becomes X, thus, X goes first
        board: Grid::new(
            BOARD_DIM.try_into().unwrap(),
            BOARD_DIM.try_into().unwrap(),
            TicTacSign::N,
        ),
        x_image,
        o_image,
    };

    pix.run(&mut tic_tac_toe)
}
