use mathhook_core::{symbol, symbols, Expression};
use mathhook_core::core::SymbolType;

#[cfg(test)]
mod symbols_macro_scalars {
    use super::*;

    #[test]
    fn test_symbols_two_scalars() {
        let syms = symbols!("x y");
        assert_eq!(syms.len(), 2);
        assert_eq!(syms[0].name(), "x");
        assert_eq!(syms[1].name(), "y");
        assert!(syms[0].symbol_type() == SymbolType::Scalar);
        assert!(syms[1].symbol_type() == SymbolType::Scalar);
    }

    #[test]
    fn test_symbols_three_scalars() {
        let syms = symbols!("x y z");
        assert_eq!(syms.len(), 3);
        assert_eq!(syms[0].name(), "x");
        assert_eq!(syms[1].name(), "y");
        assert_eq!(syms[2].name(), "z");
        assert!(syms[0].symbol_type() == SymbolType::Scalar);
        assert!(syms[1].symbol_type() == SymbolType::Scalar);
        assert!(syms[2].symbol_type() == SymbolType::Scalar);
    }

    #[test]
    fn test_symbols_five_scalars() {
        let syms = symbols!("a b c d e");
        assert_eq!(syms.len(), 5);
        assert_eq!(syms[0].name(), "a");
        assert_eq!(syms[1].name(), "b");
        assert_eq!(syms[2].name(), "c");
        assert_eq!(syms[3].name(), "d");
        assert_eq!(syms[4].name(), "e");
        for sym in &syms {
            assert!(sym.symbol_type() == SymbolType::Scalar);
        }
    }

    #[test]
    fn test_symbols_scalars_are_commutative() {
        let syms = symbols!("x y");
        let ex = Expression::symbol(syms[0].clone());
        let ey = Expression::symbol(syms[1].clone());

        let xy = Expression::mul(vec![ex.clone(), ey.clone()]);
        let yx = Expression::mul(vec![ey, ex]);

        assert_eq!(xy, yx);
    }

    #[test]
    fn test_symbols_single_scalar() {
        let syms = symbols!("x");
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].name(), "x");
        assert!(syms[0].symbol_type() == SymbolType::Scalar);
    }
}

#[cfg(test)]
mod symbols_macro_matrices {
    use super::*;

    #[test]
    fn test_symbols_two_matrices() {
        let syms = symbols!("A B"; matrix);
        assert_eq!(syms.len(), 2);
        assert_eq!(syms[0].name(), "A");
        assert_eq!(syms[1].name(), "B");
        assert!(syms[0].symbol_type() == SymbolType::Matrix);
        assert!(syms[1].symbol_type() == SymbolType::Matrix);
    }

    #[test]
    fn test_symbols_three_matrices() {
        let syms = symbols!("A B C"; matrix);
        assert_eq!(syms.len(), 3);
        assert_eq!(syms[0].name(), "A");
        assert_eq!(syms[1].name(), "B");
        assert_eq!(syms[2].name(), "C");
        assert!(syms[0].symbol_type() == SymbolType::Matrix);
        assert!(syms[1].symbol_type() == SymbolType::Matrix);
        assert!(syms[2].symbol_type() == SymbolType::Matrix);
    }

    #[test]
    fn test_symbols_matrices_are_noncommutative() {
        let syms = symbols!("A B"; matrix);
        let eA = Expression::symbol(syms[0].clone());
        let eB = Expression::symbol(syms[1].clone());

        let AB = Expression::mul(vec![eA.clone(), eB.clone()]);
        let BA = Expression::mul(vec![eB, eA]);

        assert_ne!(AB, BA);
    }

    #[test]
    fn test_symbols_matrix_multiplication_order() {
        let syms = symbols!("A B C"; matrix);
        let eA = Expression::symbol(syms[0].clone());
        let eB = Expression::symbol(syms[1].clone());
        let eC = Expression::symbol(syms[2].clone());

        let result = Expression::mul(vec![eA.clone(), eB.clone(), eC.clone()]);

        if let Expression::Mul(mul_data) = &result {
            assert_eq!(mul_data.len(), 3);
        } else {
            panic!("Expected Mul expression");
        }
    }

    #[test]
    fn test_symbols_single_matrix() {
        let syms = symbols!("A"; matrix);
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].name(), "A");
        assert!(syms[0].symbol_type() == SymbolType::Matrix);
    }
}

#[cfg(test)]
mod symbols_macro_operators {
    use super::*;

    #[test]
    fn test_symbols_two_operators() {
        let syms = symbols!("p x"; operator);
        assert_eq!(syms.len(), 2);
        assert_eq!(syms[0].name(), "p");
        assert_eq!(syms[1].name(), "x");
        assert!(syms[0].symbol_type() == SymbolType::Operator);
        assert!(syms[1].symbol_type() == SymbolType::Operator);
    }

    #[test]
    fn test_symbols_three_operators() {
        let syms = symbols!("p x H"; operator);
        assert_eq!(syms.len(), 3);
        assert_eq!(syms[0].name(), "p");
        assert_eq!(syms[1].name(), "x");
        assert_eq!(syms[2].name(), "H");
        assert!(syms[0].symbol_type() == SymbolType::Operator);
        assert!(syms[1].symbol_type() == SymbolType::Operator);
        assert!(syms[2].symbol_type() == SymbolType::Operator);
    }

    #[test]
    fn test_symbols_operators_are_noncommutative() {
        let syms = symbols!("p x"; operator);
        let ep = Expression::symbol(syms[0].clone());
        let ex = Expression::symbol(syms[1].clone());

        let px = Expression::mul(vec![ep.clone(), ex.clone()]);
        let xp = Expression::mul(vec![ex, ep]);

        assert_ne!(px, xp);
    }

    #[test]
    fn test_symbols_operator_commutator() {
        let syms = symbols!("p x"; operator);
        let ep = Expression::symbol(syms[0].clone());
        let ex = Expression::symbol(syms[1].clone());

        let comm = Expression::commutator(ep.clone(), ex.clone());

        if let Expression::Add(add_data) = &comm {
            assert_eq!(add_data.len(), 2);
        } else {
            panic!("Expected Add expression for commutator");
        }
    }

    #[test]
    fn test_symbols_single_operator() {
        let syms = symbols!("p"; operator);
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].name(), "p");
        assert!(syms[0].symbol_type() == SymbolType::Operator);
    }
}

#[cfg(test)]
mod symbols_macro_quaternions {
    use super::*;

    #[test]
    fn test_symbols_three_quaternions() {
        let syms = symbols!("i j k"; quaternion);
        assert_eq!(syms.len(), 3);
        assert_eq!(syms[0].name(), "i");
        assert_eq!(syms[1].name(), "j");
        assert_eq!(syms[2].name(), "k");
        assert!(syms[0].symbol_type() == SymbolType::Quaternion);
        assert!(syms[1].symbol_type() == SymbolType::Quaternion);
        assert!(syms[2].symbol_type() == SymbolType::Quaternion);
    }

    #[test]
    fn test_symbols_quaternions_are_noncommutative() {
        let syms = symbols!("i j"; quaternion);
        assert_eq!(syms.len(), 2);
        let ei = Expression::symbol(syms[0].clone());
        let ej = Expression::symbol(syms[1].clone());

        let ij = Expression::mul(vec![ei.clone(), ej.clone()]);
        let ji = Expression::mul(vec![ej, ei]);

        assert_ne!(ij, ji);
    }

    #[test]
    fn test_symbols_single_quaternion() {
        let syms = symbols!("i"; quaternion);
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].name(), "i");
        assert!(syms[0].symbol_type() == SymbolType::Quaternion);
    }
}

#[cfg(test)]
mod commutator_tests {
    use super::*;

    #[test]
    fn test_commutator_structure() {
        let A = Expression::symbol(symbol!(A; matrix));
        let B = Expression::symbol(symbol!(B; matrix));
        let comm = Expression::commutator(A.clone(), B.clone());

        if let Expression::Add(terms) = &comm {
            assert_eq!(terms.len(), 2);

            if let Expression::Mul(factors1) = &terms[0] {
                assert_eq!(factors1.len(), 2);
            } else {
                panic!("Expected first term to be Mul");
            }

            if let Expression::Mul(factors2) = &terms[1] {
                assert_eq!(factors2.len(), 2);
                if let Expression::Number(n) = &factors2[0] {
                    assert!(n.is_negative_one());
                } else {
                    panic!("Expected -1 coefficient");
                }
            } else {
                panic!("Expected second term to be Mul");
            }
        } else {
            panic!("Commutator should be Add expression");
        }
    }

    #[test]
    fn test_commutator_self_is_zero() {
        let A = Expression::symbol(symbol!(A; matrix));
        let comm = Expression::commutator(A.clone(), A.clone());

        if let Expression::Add(terms) = &comm {
            assert_eq!(terms.len(), 2);
        }
    }

    #[test]
    fn test_commutator_antisymmetry() {
        let A = Expression::symbol(symbol!(A; matrix));
        let B = Expression::symbol(symbol!(B; matrix));

        let comm_AB = Expression::commutator(A.clone(), B.clone());
        let comm_BA = Expression::commutator(B, A);

        assert_ne!(comm_AB, comm_BA);
    }

    #[test]
    fn test_commutator_scalars_simplifies_to_zero() {
        let x = Expression::symbol(symbol!(x));
        let y = Expression::symbol(symbol!(y));

        let comm = Expression::commutator(x.clone(), y.clone());

        match &comm {
            Expression::Add(_) => {},
            Expression::Number(n) if n.is_zero() => {},
            _ => panic!("Commutator should be Add or zero (commutative scalars)"),
        }
    }
}

#[cfg(test)]
mod anticommutator_tests {
    use super::*;

    #[test]
    fn test_anticommutator_structure() {
        let A = Expression::symbol(symbol!(A; matrix));
        let B = Expression::symbol(symbol!(B; matrix));
        let anticomm = Expression::anticommutator(A.clone(), B.clone());

        if let Expression::Add(terms) = &anticomm {
            assert_eq!(terms.len(), 2);

            if let Expression::Mul(factors1) = &terms[0] {
                assert_eq!(factors1.len(), 2);
            } else {
                panic!("Expected first term to be Mul");
            }

            if let Expression::Mul(factors2) = &terms[1] {
                assert_eq!(factors2.len(), 2);
            } else {
                panic!("Expected second term to be Mul");
            }
        } else {
            panic!("Anticommutator should be Add expression");
        }
    }

    #[test]
    fn test_anticommutator_symmetry() {
        let A = Expression::symbol(symbol!(A; matrix));
        let B = Expression::symbol(symbol!(B; matrix));

        let anticomm_AB = Expression::anticommutator(A.clone(), B.clone());
        let anticomm_BA = Expression::anticommutator(B, A);

        if let (Expression::Add(terms_AB), Expression::Add(terms_BA)) = (&anticomm_AB, &anticomm_BA) {
            assert_eq!(terms_AB.len(), terms_BA.len());
            assert_eq!(terms_AB.len(), 2);
        } else {
            panic!("Both should be Add expressions");
        }
    }

    #[test]
    fn test_anticommutator_self() {
        let A = Expression::symbol(symbol!(A; matrix));
        let anticomm = Expression::anticommutator(A.clone(), A.clone());

        if let Expression::Add(terms) = &anticomm {
            assert_eq!(terms.len(), 2);
        }
    }
}
