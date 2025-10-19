//! Symbol creation macros for ergonomic variable definition

/// Symbol creation macro with optional type specification
///
/// Creates symbols with explicit type support for noncommutative algebra.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::symbol;
///
/// // Scalar symbols (default, commutative)
/// let x = symbol!(x);
/// let theta = symbol!(theta);
///
/// // Matrix symbols (noncommutative)
/// let A = symbol!(A; matrix);
/// let B = symbol!(B; matrix);
///
/// // Operator symbols (noncommutative)
/// let p = symbol!(p; operator);
/// let x_op = symbol!(x; operator);
///
/// // Quaternion symbols (noncommutative)
/// let i = symbol!(i; quaternion);
/// let j = symbol!(j; quaternion);
/// ```
#[macro_export]
macro_rules! symbol {
    // Scalar (default)
    ($id:ident) => {
        $crate::Symbol::scalar(stringify!($id))
    };
    ($name:literal) => {
        $crate::Symbol::scalar($name)
    };
    ($name:expr) => {
        $crate::Symbol::scalar($name)
    };

    // Matrix type
    ($id:ident; matrix) => {
        $crate::Symbol::matrix(stringify!($id))
    };
    ($name:literal; matrix) => {
        $crate::Symbol::matrix($name)
    };
    ($name:expr; matrix) => {
        $crate::Symbol::matrix($name)
    };

    // Operator type
    ($id:ident; operator) => {
        $crate::Symbol::operator(stringify!($id))
    };
    ($name:literal; operator) => {
        $crate::Symbol::operator($name)
    };
    ($name:expr; operator) => {
        $crate::Symbol::operator($name)
    };

    // Quaternion type
    ($id:ident; quaternion) => {
        $crate::Symbol::quaternion(stringify!($id))
    };
    ($name:literal; quaternion) => {
        $crate::Symbol::quaternion($name)
    };
    ($name:expr; quaternion) => {
        $crate::Symbol::quaternion($name)
    };
}

/// Bulk symbol creation with optional type specification
///
/// Creates multiple symbols at once with the same type. Follows SymPy's `symbols()` API.
/// Returns a Vec of symbols since declarative macros cannot return tuples of varying sizes.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::symbols;
///
/// // Scalar symbols (default, commutative)
/// let syms = symbols!("x y z");
/// assert_eq!(syms.len(), 3);
///
/// // Matrix symbols (noncommutative)
/// let mats = symbols!("A B C"; matrix);
/// assert_eq!(mats.len(), 3);
///
/// // Operator symbols (noncommutative)
/// let ops = symbols!("p x h"; operator);
/// assert_eq!(ops.len(), 3);
///
/// // Quaternion symbols (noncommutative)
/// let quats = symbols!("i j k"; quaternion);
/// assert_eq!(quats.len(), 3);
/// ```
#[macro_export]
macro_rules! symbols {
    // Scalar symbols (default)
    ($names:literal) => {{
        $names.split_whitespace()
            .map(|name| $crate::Symbol::scalar(name))
            .collect::<Vec<_>>()
    }};

    // Matrix symbols
    ($names:literal; matrix) => {{
        $names.split_whitespace()
            .map(|name| $crate::Symbol::matrix(name))
            .collect::<Vec<_>>()
    }};

    // Operator symbols
    ($names:literal; operator) => {{
        $names.split_whitespace()
            .map(|name| $crate::Symbol::operator(name))
            .collect::<Vec<_>>()
    }};

    // Quaternion symbols
    ($names:literal; quaternion) => {{
        $names.split_whitespace()
            .map(|name| $crate::Symbol::quaternion(name))
            .collect::<Vec<_>>()
    }};
}
