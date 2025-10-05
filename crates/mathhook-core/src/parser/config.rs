pub struct ParserConfig {
    pub enable_implicit_multiplication: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            enable_implicit_multiplication: true,
        }
    }
}
