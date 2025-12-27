pub mod analyzer;
pub mod classifier;
pub mod config;
pub mod doc_transformer;
pub mod emitter;
pub mod manifest;
pub mod scanner;
pub mod trait_analyzer;
pub mod types;

pub use classifier::{classify_all, classify_type, ClassifiedApi, TypeClassification};
pub use config::{BindingConfig, CustomMapping};
pub use emitter::{Emitter, NodeEmitter, PythonEmitter};
pub use manifest::{BindingManifest, Target, TypeBindingInfo};
pub use scanner::{
    FieldInfo, FunctionInfo, ImplInfo, MethodInfo, MethodSignature, ScannedApi, TypeInfo,
};
pub use trait_analyzer::{
    is_bindable_trait, is_bindable_trait_name, DomainTraitMethod, SupportedTrait, TraitAnalysis,
};
pub use types::{AnalyzedMethod, MappedType};
