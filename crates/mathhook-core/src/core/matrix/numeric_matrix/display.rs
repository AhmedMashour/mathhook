use crate::core::matrix::numeric_matrix::NumericMatrix;
use std::fmt;

impl fmt::Display for NumericMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.rows {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "[")?;
            for j in 0..self.cols {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.data[i * self.cols + j])?;
            }
            write!(f, "]")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_2x2() {
        let m = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(m.to_string(), "[[1, 2], [3, 4]]");
    }

    #[test]
    fn test_display_2x3() {
        let m = NumericMatrix::from_flat(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        assert_eq!(m.to_string(), "[[1, 2, 3], [4, 5, 6]]");
    }

    #[test]
    fn test_display_1x1() {
        let m = NumericMatrix::from_flat(1, 1, vec![42.0]).unwrap();
        assert_eq!(m.to_string(), "[[42]]");
    }

    #[test]
    fn test_display_floats() {
        let m = NumericMatrix::from_flat(2, 2, vec![1.5, 2.5, 3.5, 4.5]).unwrap();
        assert_eq!(m.to_string(), "[[1.5, 2.5], [3.5, 4.5]]");
    }

    #[test]
    fn test_debug() {
        let m = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let debug_str = format!("{:?}", m);
        assert!(debug_str.contains("NumericMatrix"));
        assert!(debug_str.contains("rows"));
        assert!(debug_str.contains("cols"));
        assert!(debug_str.contains("data"));
    }
}
