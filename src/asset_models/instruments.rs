//! Submodule defining `instruments`-related instances in the database.

pub mod ball_mill_machine;
pub mod centrifuge;
pub mod freeze_dryer;
pub mod freezer;
pub mod phone;
pub mod pipette_tips;
pub mod pipettes;
pub mod volume_measuring_device;
pub mod weighing_scale;

pub use ball_mill_machine::ball_mill_machine;
pub use centrifuge::safelock_centrifuge;
pub use freeze_dryer::freeze_dryer;
pub use freezer::freezer;
pub use phone::phone_model;
pub use pipette_tips::{pipette_tip_200ul, pipette_tip_1000ul};
pub use pipettes::{pipette_200ul, pipette_1000ul};
pub use volume_measuring_device::volume_measuring_device_model;
pub use weighing_scale::weighing_scale;
