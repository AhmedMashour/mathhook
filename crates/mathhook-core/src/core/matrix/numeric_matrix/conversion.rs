use super::NumericMatrix;
use crate::core::{Expression, Number};
use crate::matrices::unified::Matrix;

impl NumericMatrix {
    pub fn try_from_expression(expr: &Expression) -> Option<Self> {
        if !Self::can_convert(expr) {
            return None;
        }

        match expr {
            Expression::Matrix(matrix_box) => Self::try_from_matrix(matrix_box.as_ref()),
            _ => None,
        }
    }

    pub fn try_from_matrix(matrix: &Matrix) -> Option<Self> {
        let (rows, cols) = matrix.dimensions();

        if rows == 0 || cols == 0 {
            return None;
        }

        let mut data = Vec::with_capacity(rows * cols);

        for i in 0..rows {
            for j in 0..cols {
                let elem = matrix.get_element(i, j);
                if let Some(val) = expression_to_f64(&elem) {
                    data.push(val);
                } else {
                    return None;
                }
            }
        }

        NumericMatrix::from_flat(rows, cols, data).ok()
    }

    pub fn to_expression(&self) -> Expression {
        let mut rows_vec = Vec::with_capacity(self.rows);

        for i in 0..self.rows {
            let mut row = Vec::with_capacity(self.cols);
            for j in 0..self.cols {
                let val = self.data[i * self.cols + j];
                row.push(Expression::Number(Number::float(val)));
            }
            rows_vec.push(row);
        }

        Expression::Matrix(Box::new(Matrix::dense(rows_vec)))
    }

    pub fn to_matrix(&self) -> Matrix {
        let mut rows_vec = Vec::with_capacity(self.rows);

        for i in 0..self.rows {
            let mut row = Vec::with_capacity(self.cols);
            for j in 0..self.cols {
                let val = self.data[i * self.cols + j];
                row.push(Expression::Number(Number::float(val)));
            }
            rows_vec.push(row);
        }

        Matrix::dense(rows_vec)
    }

    pub fn can_convert(expr: &Expression) -> bool {
        match expr {
            Expression::Matrix(matrix_box) => Self::can_convert_matrix(matrix_box.as_ref()),
            _ => false,
        }
    }

    pub fn can_convert_matrix(matrix: &Matrix) -> bool {
        let (rows, cols) = matrix.dimensions();

        if rows == 0 || cols == 0 {
            return false;
        }

        for i in 0..rows {
            for j in 0..cols {
                let elem = matrix.get_element(i, j);
                if expression_to_f64(&elem).is_none() {
                    return false;
                }
            }
        }

        true
    }
}

fn expression_to_f64(expr: &Expression) -> Option<f64> {
    match expr {
        Expression::Number(num) => num.to_float().ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_try_from_expression_integer() {
        let matrix = Matrix::from_arrays([[1, 2], [3, 4]]);
        let expr = Expression::Matrix(Box::new(matrix));

        let m = NumericMatrix::try_from_expression(&expr).unwrap();

        assert_eq!(m.dimensions(), (2, 2));
        assert!(approx_eq(m.get(0, 0).unwrap(), 1.0));
        assert!(approx_eq(m.get(0, 1).unwrap(), 2.0));
        assert!(approx_eq(m.get(1, 0).unwrap(), 3.0));
        assert!(approx_eq(m.get(1, 1).unwrap(), 4.0));
    }

    #[test]
    fn test_try_from_expression_mixed() {
        let rows = vec![
            vec![Expression::integer(1), Expression::float(2.0)],
            vec![Expression::rational(1, 2), Expression::integer(4)],
        ];
        let matrix = Matrix::dense(rows);
        let expr = Expression::Matrix(Box::new(matrix));

        let m = NumericMatrix::try_from_expression(&expr).unwrap();

        assert_eq!(m.dimensions(), (2, 2));
        assert!(approx_eq(m.get(0, 0).unwrap(), 1.0));
        assert!(approx_eq(m.get(0, 1).unwrap(), 2.0));
        assert!(approx_eq(m.get(1, 0).unwrap(), 0.5));
        assert!(approx_eq(m.get(1, 1).unwrap(), 4.0));
    }

    #[test]
    fn test_try_from_expression_symbolic_fails() {
        let rows = vec![vec![Expression::symbol("x"), Expression::integer(2)]];
        let matrix = Matrix::dense(rows);
        let expr = Expression::Matrix(Box::new(matrix));

        assert!(NumericMatrix::try_from_expression(&expr).is_none());
    }

    #[test]
    fn test_try_from_expression_non_matrix_fails() {
        let expr = Expression::integer(42);
        assert!(NumericMatrix::try_from_expression(&expr).is_none());
    }

    #[test]
    fn test_to_expression() {
        let m = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();

        let expr = m.to_expression();

        match expr {
            Expression::Matrix(matrix_box) => {
                let matrix = matrix_box.as_ref();
                assert_eq!(matrix.dimensions(), (2, 2));

                let elem00 = matrix.get_element(0, 0);
                match elem00 {
                    Expression::Number(Number::Float(f)) => assert!(approx_eq(f, 1.0)),
                    _ => panic!("Expected float number"),
                }

                let elem11 = matrix.get_element(1, 1);
                match elem11 {
                    Expression::Number(Number::Float(f)) => assert!(approx_eq(f, 4.0)),
                    _ => panic!("Expected float number"),
                }
            }
            _ => panic!("Expected matrix expression"),
        }
    }

    #[test]
    fn test_to_matrix() {
        let m = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();

        let matrix = m.to_matrix();

        assert_eq!(matrix.dimensions(), (2, 2));

        match matrix.get_element(0, 0) {
            Expression::Number(Number::Float(f)) => assert!(approx_eq(f, 1.0)),
            _ => panic!("Expected float number"),
        }

        match matrix.get_element(1, 1) {
            Expression::Number(Number::Float(f)) => assert!(approx_eq(f, 4.0)),
            _ => panic!("Expected float number"),
        }
    }

    #[test]
    fn test_round_trip() {
        let original = NumericMatrix::from_flat(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();

        let expr = original.to_expression();
        let restored = NumericMatrix::try_from_expression(&expr).unwrap();

        assert_eq!(original.dimensions(), restored.dimensions());
        for i in 0..original.rows {
            for j in 0..original.cols {
                assert!(approx_eq(
                    original.get(i, j).unwrap(),
                    restored.get(i, j).unwrap()
                ));
            }
        }
    }

    #[test]
    fn test_matrix_round_trip() {
        let original = NumericMatrix::from_flat(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();

        let matrix = original.to_matrix();
        let restored = NumericMatrix::try_from_matrix(&matrix).unwrap();

        assert_eq!(original.dimensions(), restored.dimensions());
        for i in 0..original.rows {
            for j in 0..original.cols {
                assert!(approx_eq(
                    original.get(i, j).unwrap(),
                    restored.get(i, j).unwrap()
                ));
            }
        }
    }

    #[test]
    fn test_can_convert_valid() {
        let rows = vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::float(3.0), Expression::rational(1, 2)],
        ];
        let matrix = Matrix::dense(rows);
        let expr = Expression::Matrix(Box::new(matrix));

        assert!(NumericMatrix::can_convert(&expr));
    }

    #[test]
    fn test_can_convert_symbolic() {
        let rows = vec![vec![Expression::symbol("x"), Expression::integer(2)]];
        let matrix = Matrix::dense(rows);
        let expr = Expression::Matrix(Box::new(matrix));

        assert!(!NumericMatrix::can_convert(&expr));
    }

    #[test]
    fn test_can_convert_non_matrix() {
        assert!(!NumericMatrix::can_convert(&Expression::integer(42)));
        assert!(!NumericMatrix::can_convert(&Expression::symbol("x")));
    }

    #[test]
    fn test_identity_matrix() {
        let matrix = Matrix::identity(3);
        let expr = Expression::Matrix(Box::new(matrix));

        let m = NumericMatrix::try_from_expression(&expr).unwrap();

        assert_eq!(m.dimensions(), (3, 3));
        assert!(approx_eq(m.get(0, 0).unwrap(), 1.0));
        assert!(approx_eq(m.get(1, 1).unwrap(), 1.0));
        assert!(approx_eq(m.get(2, 2).unwrap(), 1.0));
        assert!(approx_eq(m.get(0, 1).unwrap(), 0.0));
        assert!(approx_eq(m.get(1, 0).unwrap(), 0.0));
    }

    #[test]
    fn test_can_convert_matrix_directly() {
        let matrix = Matrix::from_arrays([[1, 2], [3, 4]]);
        assert!(NumericMatrix::can_convert_matrix(&matrix));

        let rows = vec![vec![Expression::symbol("x"), Expression::integer(2)]];
        let matrix = Matrix::dense(rows);
        assert!(!NumericMatrix::can_convert_matrix(&matrix));
    }
}
