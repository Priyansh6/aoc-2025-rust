use std::fmt;
use std::ops;
use std::str;

pub struct Grid<T> {
    cells: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

#[derive(PartialEq, Copy, Clone)]
pub struct GridPosition {
    pub row: usize,
    pub col: usize,
}

impl<T> Grid<T> {
    fn from_rows<I, J>(rows: I) -> Result<Self, String>
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
            return Err("Grid cannot have height 0".to_string());
        }
        let width = cells.first().unwrap().len();
        if width == 0 {
            return Err("Grid cannot have width 0".to_string());
        }

        Ok(Self {
            cells,
            width,
            height,
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

impl<T> ops::Index<GridPosition> for Grid<T> {
    type Output = T;

    fn index(&self, pos: GridPosition) -> &Self::Output {
        &self.cells[pos.row][pos.col]
    }
}

impl<T> ops::IndexMut<GridPosition> for Grid<T> {
    fn index_mut(&mut self, pos: GridPosition) -> &mut Self::Output {
        &mut self.cells[pos.row][pos.col]
    }
}

impl<T> str::FromStr for Grid<T>
where
    T: str::FromStr,
    T::Err: fmt::Display,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Result<Vec<Vec<T>>, String> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_string()
                            .parse::<T>()
                            .map_err(|e| format!("Parse grid error: {}", e))
                    })
                    .collect()
            })
            .collect();

        Grid::from_rows(rows?)
    }
}
