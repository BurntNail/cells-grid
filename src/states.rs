use rand::Rng;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CellState {
    Copy(CellDirection),
    Move(CellDirection),
    Base,
    Trash,
    None,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CellDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Display for CellDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use CellDirection::*;
        f.write_str(match self {
            Up => "Up",
            Down => "Down",
            Left => "Left",
            Right => "Right",
        })
    }
}

impl CellState {
    //region scroll
    pub fn scroll_up(&self) -> Self {
        use CellDirection::*;
        use CellState::*;

        match self {
            Copy(d) => match d {
                Up => Copy(Down),
                Down => Copy(Left),
                Left => Copy(Right),
                Right => Move(Up),
            },
            Move(d) => match d {
                Up => Move(Down),
                Down => Move(Left),
                Left => Move(Right),
                Right => Base,
            },
            Base => Trash,
            Trash => None,
            None => Copy(Up),
        }
    }
    pub fn scroll_down(&self) -> Self {
        use CellDirection::*;
        use CellState::*;

        match self {
            Copy(d) => match d {
                Up => None,
                Down => Copy(Up),
                Left => Copy(Down),
                Right => Copy(Left),
            },
            Move(d) => match d {
                Up => Copy(Right),
                Down => Move(Up),
                Left => Move(Down),
                Right => Move(Left),
            },
            Base => Move(Right),
            Trash => Base,
            None => Trash,
        }
    }
    //endregion

    pub fn get_sprite_name(&self) -> String {
        use CellState::*;

        format!(
            "{}.png",
            match self {
                Copy(d) => format!("Copy{}", d),
                Move(d) => format!("Move{}", d),
                Trash => "Trash".to_string(),
                Base => "Base".to_string(),
                None => "None".to_string(),
            }
        )
    }

    pub fn get_all_states() -> Vec<CellState> {
        use CellDirection::*;
        use CellState::*;

        let all_dirs = vec![Up, Down, Left, Right];

        let mut vec = vec![Trash, Base, None];
        all_dirs.into_iter().for_each(|dir| {
            vec.push(Copy(dir));
            vec.push(Move(dir));
        });
        vec
    }

    pub fn get_rand(list: Vec<CellState>) -> CellState {
        list[rand::thread_rng().gen_range(0..list.len())]
    }
}
