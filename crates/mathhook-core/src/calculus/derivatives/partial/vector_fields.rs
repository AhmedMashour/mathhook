//! Vector field operations including divergence, curl, Laplacian, and conservative field analysis

mod conservative;
mod fluid_dynamics;
mod operations;

#[cfg(test)]
#[path = "vector_fields/tests.rs"]
mod tests;

pub use conservative::ConservativeFields;
pub use fluid_dynamics::FluidDynamicsOperations;
pub use operations::VectorFieldOperations;
