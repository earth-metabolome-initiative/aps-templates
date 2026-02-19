//! Submodule to initialize reagent models in the database.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_physical_asset_models::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns a reagent model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let reagent_model1 = reagent_model(
///     &test_user,
///     "Template Reagent",
///     "Template reagent description.",
///     &mut conn,
/// )
/// .expect("Failed to create reagent model");
/// let reagent_model2 = reagent_model(
///     &test_user,
///     "Template Reagent",
///     "Template reagent description.",
///     &mut conn,
/// )
/// .expect("Failed to create reagent model");
/// assert_eq!(reagent_model1, reagent_model2);
/// ```
pub fn reagent_model<C>(
    user: &User,
    name: &str,
    description: &str,
    conn: &mut C,
) -> Result<
    NestedModel<physical_asset_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<physical_asset_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<physical_asset_models::table, C>,
{
    let reference_namespace = reference_namespace(user, conn)?;

    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (reference_namespace.get_column::<namespaces::id>(), (name,)),
        conn,
    ) {
        return Ok(existing);
    }

    physical_asset_models::table::builder()
        .try_name(name)
        .expect("Failed to set reagent model name")
        .try_description(description)
        .expect("Failed to set reagent model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}

pub mod distilled_water;
pub mod ethanol;
pub mod formic_acid;
pub mod liquid_nitrogen;
pub mod methanol;
pub use distilled_water::distilled_water;
pub use ethanol::absolute_ethanol;
pub use formic_acid::formic_acid;
pub use liquid_nitrogen::liquid_nitrogen;
pub use methanol::methanol_hplc;
