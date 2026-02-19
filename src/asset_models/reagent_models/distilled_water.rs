//! Submodule defining functions to initialize `distilled_water` reagent models.

use super::reagent_model;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_physical_asset_models::*;
use aps::aps_users::User;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the distilled water reagent model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let distilled_water_model1 =
///     distilled_water(&test_user, &mut conn).expect("Failed to create distilled water model");
/// let distilled_water_model2 =
///     distilled_water(&test_user, &mut conn).expect("Failed to create distilled water model");
/// assert_eq!(distilled_water_model1, distilled_water_model2);
/// ```
pub fn distilled_water<C>(
    user: &User,
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
    const DISTILLED_WATER_NAME: &str = "Distilled water";
    const DISTILLED_WATER_DESCRIPTION: &str = "Distilled water, pure";
    reagent_model(
        user,
        DISTILLED_WATER_NAME,
        DISTILLED_WATER_DESCRIPTION,
        conn,
    )
}
