# Performance Analysis: Linear vs HashMap for Parser Constants

## Current Scale Analysis

### LaTeX Constants (3 items)
```rust
// Current: 3 string comparisons max
if input == "\\pi" { ... }
if input == "\\infty" { ... } 
if input == "\\e" { ... }
```

### LaTeX Functions (6 items)
```rust
// From constants.rs
("\\sin(", "sin"),
("\\cos(", "cos"), 
("\\tan(", "tan"),
("\\ln(", "ln"),
// etc...
```

### Wolfram Functions (4 items)
```rust
("Sin[", "sin"),
("Cos[", "cos"),
("Tan[", "tan"), 
("Log[", "ln"),
```

## Performance Breakeven Analysis

### Linear Search Performance
- **Best case**: O(1) - first match
- **Average case**: O(n/2) - middle match  
- **Worst case**: O(n) - last match or no match
- **Memory**: 0 allocations
- **Cache locality**: Excellent (code segment)

### HashMap Performance
- **Best/Average case**: O(1) - hash lookup
- **Worst case**: O(n) - hash collisions
- **Memory**: ~48 bytes base + (24 bytes × entries) on 64-bit
- **Cache locality**: Good (heap allocated)

## Breakeven Point Calculation

For small collections, linear search is faster due to:
1. **No hash computation overhead**
2. **Perfect cache locality** 
3. **No heap allocation**
4. **Branch predictor friendly**

**Breakeven point**: ~10-15 items for string keys
**Current sizes**: 3-6 items per category

## Recommendation by Category

### ✅ Keep Linear for Small Sets (< 10 items)
- **Constants**: 3 items → Linear is optimal
- **Simple functions**: 6 items → Linear is fine
- **Detection patterns**: 11 items → Borderline, linear OK

### 🤔 Consider HashMap for Large Sets (> 15 items)
- **Full LaTeX command set**: Could grow to 50+ → HashMap beneficial
- **Unicode symbol mappings**: 100+ → HashMap essential

### 📈 Hybrid Approach for Growth
```rust
// For future expansion
lazy_static! {
    static ref LATEX_CONSTANTS: HashMap<&'static str, MathConstant> = {
        let mut m = HashMap::new();
        m.insert("\\pi", MathConstant::Pi);
        m.insert("\\infty", MathConstant::Infinity);
        m.insert("\\e", MathConstant::E);
        // Easy to add 50+ more
        m
    };
}
```

## Memory Usage Comparison

### Current (3 constants)
```
Code size: ~200 bytes (3 if statements)
Runtime memory: 0 bytes
```

### HashMap (3 constants)  
```
HashMap overhead: ~48 bytes
String keys: ~24 bytes  
Entries: ~72 bytes (3 × 24)
Total: ~144 bytes
```

## Readability Comparison

### Linear (Current)
```rust
✅ Pros:
- Immediately obvious what's supported
- Easy to add/remove entries
- No indirection
- Compile-time constants

❌ Cons:  
- Verbose for large sets
- Easy to forget to add new entries
```

### HashMap
```rust
✅ Pros:
- Scales beautifully
- Centralized definition
- Easy bulk operations

❌ Cons:
- Runtime initialization
- Less obvious what's supported
- Indirection
```

## Final Recommendation

### For Current Codebase: **Keep Linear** ✅
- 3-6 items per category
- Zero allocation
- Better performance  
- More readable

### For Future Growth: **Prepare HashMap Migration**
```rust
// When we hit 15+ items, migrate to:
use once_cell::sync::Lazy;

static LATEX_CONSTANTS: Lazy<HashMap<&str, MathConstant>> = Lazy::new(|| {
    [
        ("\\pi", MathConstant::Pi),
        ("\\infty", MathConstant::Infinity),
        // ... 50+ more
    ].iter().cloned().collect()
});
```

## Conclusion

**Current approach is optimal** for the scale we're at. HashMap would be premature optimization that adds complexity without benefit.

**Migration threshold**: When any category hits 15+ items, switch to HashMap for that category.
