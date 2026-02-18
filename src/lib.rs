#![doc = include_str!("../README.md")]

pub mod asset_models;
pub mod reference_namespaces;

pub mod prelude {
    pub use crate::asset_models::*;
    pub use crate::reference_namespaces::*;
}
