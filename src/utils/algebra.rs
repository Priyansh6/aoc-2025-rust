pub struct GaussianEliminationGF2Result {
    pub matrix: Vec<Vec<bool>>,
    pub pivot_cols: Vec<usize>,
    pub free_cols: Vec<usize>,
}

/// Performs Gaussian elimination over GF(2) on an augmented matrix where each row
/// represents an equation: the first `n` elements are coefficients and the last
/// element is the right-hand side.
///
/// Returns a [`GaussianEliminationResult`] containing the reduced matrix, the indices
/// of pivot columns (determined variables), and the indices of free columns (free variables).
pub fn gaussian_elimination_gf2(mut matrix: Vec<Vec<bool>>) -> GaussianEliminationGF2Result {
    let nrows = matrix.len();
    if nrows == 0 {
        return GaussianEliminationGF2Result {
            matrix,
            pivot_cols: vec![],
            free_cols: vec![],
        };
    }
    let ncols = matrix[0].len() - 1; // exclude augmented column

    let mut pivot_cols = vec![];
    let mut free_cols = vec![];
    let mut row = 0;

    for col in 0..ncols {
        // find a pivot row for this column
        let pivot_row = (row..nrows).find(|&r| matrix[r][col]);
        let Some(pivot_row) = pivot_row else {
            free_cols.push(col); // no pivot, this is a free variable
            continue;
        };

        matrix.swap(row, pivot_row);
        pivot_cols.push(col);

        // eliminate all other rows with a 1 in this column
        let pivot = matrix[row].clone();
        for r in 0..nrows {
            if r != row && matrix[r][col] {
                matrix[r].iter_mut().zip(&pivot).for_each(|(a, b)| *a ^= b);
            }
        }

        row += 1;
    }

    GaussianEliminationGF2Result {
        matrix,
        pivot_cols,
        free_cols,
    }
}
