//! Advanced matrix operations and decomposition methods

use crate::core::Expression;
use crate::matrices::types::*;
use crate::matrices::unified::Matrix;
use crate::simplify::Simplify;

impl Matrix {
    /// Compute matrix inverse using Gauss-Jordan elimination
    pub(crate) fn gauss_jordan_inverse(&self) -> Matrix {
        let (n, _) = self.dimensions();

        let mut augmented = Vec::new();
        for i in 0..n {
            let mut row = Vec::new();
            for j in 0..n {
                row.push(self.get_element(i, j));
            }
            for j in 0..n {
                if i == j {
                    row.push(Expression::integer(1));
                } else {
                    row.push(Expression::integer(0));
                }
            }
            augmented.push(row);
        }

        for i in 0..n {
            let mut pivot_row = i;
            for k in (i + 1)..n {
                if !augmented[k][i].is_zero() && augmented[pivot_row][i].is_zero() {
                    pivot_row = k;
                }
            }

            if pivot_row != i {
                augmented.swap(i, pivot_row);
            }

            if augmented[i][i].is_zero() {
                return Matrix::Dense(MatrixData { rows: vec![] });
            }

            let pivot = augmented[i][i].clone();
            for elem in augmented[i].iter_mut().take(2 * n) {
                *elem = Expression::mul(vec![
                    elem.clone(),
                    Expression::pow(pivot.clone(), Expression::integer(-1)),
                ])
                .simplify();
            }

            for k in 0..n {
                if k != i {
                    let factor = augmented[k][i].clone();
                    let row_i: Vec<Expression> = augmented[i].iter().take(2 * n).cloned().collect();
                    for (j, elem) in augmented[k].iter_mut().enumerate().take(2 * n) {
                        let subtract_term = Expression::mul(vec![factor.clone(), row_i[j].clone()]);
                        *elem = Expression::add(vec![
                            elem.clone(),
                            Expression::mul(vec![Expression::integer(-1), subtract_term]),
                        ])
                        .simplify();
                    }
                }
            }
        }

        let inverse_rows: Vec<Vec<Expression>> = (0..n)
            .map(|i| (n..(2 * n)).map(|j| augmented[i][j].clone()).collect())
            .collect();

        Matrix::Dense(MatrixData { rows: inverse_rows })
    }
}
