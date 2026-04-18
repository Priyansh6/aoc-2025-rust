#![allow(dead_code)]

use crate::utils::parser::{CharParser, ParseError, Parser, StrParser};
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct GridPosition {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone)]
pub struct Grid<T> {
    cells: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    fn from_rows<I, J>(rows: I) -> Result<Self, ParseError>
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = T>,
    {
        let cells: Vec<Vec<T>> = rows
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();

        let height = cells.len();
        if height == 0 {
            return Err("Grid cannot have height 0".to_string().into());
        }
        let width = cells.first().unwrap().len();
        if width == 0 {
            return Err("Grid cannot have width 0".to_string().into());
        }

        Ok(Self {
            cells,
            width,
            height,
        })
    }

    pub fn parser(
        cell: impl CharParser<Output = T>,
    ) -> impl for<'a> Parser<&'a str, Output = Grid<T>> {
        cell.chars().lines().and_then(Grid::from_rows)
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn below(&self, pos: &GridPosition) -> Option<GridPosition> {
        let new_row = pos.row + 1;
        if new_row >= self.height {
            return None;
        }
        Some(GridPosition {
            row: new_row,
            col: pos.col,
        })
    }

    pub fn above(&self, pos: &GridPosition) -> Option<GridPosition> {
        if pos.row == 0 {
            return None;
        }
        Some(GridPosition {
            row: pos.row - 1,
            col: pos.col,
        })
    }

    pub fn right(&self, pos: &GridPosition) -> Option<GridPosition> {
        let new_col = pos.col + 1;
        if new_col >= self.width {
            return None;
        }
        Some(GridPosition {
            row: pos.row,
            col: new_col,
        })
    }

    pub fn left(&self, pos: &GridPosition) -> Option<GridPosition> {
        if pos.col == 0 {
            return None;
        }
        Some(GridPosition {
            row: pos.row,
            col: pos.col - 1,
        })
    }

    pub fn iter_enumerated(&self) -> impl Iterator<Item = (GridPosition, &T)> {
        self.cells.iter().enumerate().flat_map(|(row, row_vec)| {
            row_vec
                .iter()
                .enumerate()
                .map(move |(col, cell)| (GridPosition { row, col }, cell))
        })
    }

    pub fn surrounding_cells(&self, pos: GridPosition) -> impl Iterator<Item = &T> {
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        OFFSETS.iter().filter_map(move |&(r_off, c_off)| {
            let new_row = pos.row.checked_add_signed(r_off)?;
            let new_col = pos.col.checked_add_signed(c_off)?;

            if new_row < self.height && new_col < self.width {
                Some(&self.cells[new_row][new_col])
            } else {
                None
            }
        })
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, elem: &T) -> Option<GridPosition> {
        for (row_i, row) in self.cells.iter().enumerate() {
            for (col_i, cell) in row.iter().enumerate() {
                if elem == cell {
                    return Some(GridPosition {
                        row: row_i,
                        col: col_i,
                    });
                }
            }
        }
        None
    }
}

impl<T> Index<GridPosition> for Grid<T> {
    type Output = T;

    fn index(&self, pos: GridPosition) -> &Self::Output {
        &self.cells[pos.row][pos.col]
    }
}

impl<T> IndexMut<GridPosition> for Grid<T> {
    fn index_mut(&mut self, pos: GridPosition) -> &mut Self::Output {
        &mut self.cells[pos.row][pos.col]
    }
}
