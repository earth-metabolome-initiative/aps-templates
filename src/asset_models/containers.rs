//! Submodule to initialize `container` models in the database.

pub mod bottles;
pub mod boxes;
pub mod conical_centrifugal_tubes;
pub mod racks;
pub mod safelock_tubes;
pub mod vial_caps;
pub mod vial_inserts;
pub mod vials;
pub mod wrappers;

pub use bottles::bottle_1l;
pub use boxes::{polystyrene_box, vial_rack_1_5ml};
pub use conical_centrifugal_tubes::conical_centrifugal_tube_50ml;
pub use racks::conical_centrifugal_tube_50ml_rack;
pub use safelock_tubes::safelock_tubes_2ml;
pub use vial_caps::{sealed_cap_vial_1_5ml, splitted_cap_vial_1_5ml};
pub use vial_inserts::vial_insert_200ul;
pub use vials::vial_1_5ml;
pub use wrappers::coffee_filter_wrapper;
