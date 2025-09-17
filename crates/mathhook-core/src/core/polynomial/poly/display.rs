use super::IntPoly;

impl Default for IntPoly {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::fmt::Display for IntPoly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        let mut first = true;
        for (i, &c) in self.coeffs.iter().enumerate().rev() {
            if c == 0 {
                continue;
            }

            if !first {
                if c > 0 {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                }
            } else if c < 0 {
                write!(f, "-")?;
            }
            first = false;

            let abs_c = c.abs();
            match i {
                0 => write!(f, "{}", abs_c)?,
                1 if abs_c == 1 => write!(f, "x")?,
                1 => write!(f, "{}x", abs_c)?,
                _ if abs_c == 1 => write!(f, "x^{}", i)?,
                _ => write!(f, "{}x^{}", abs_c, i)?,
            }
        }

        Ok(())
    }
}
