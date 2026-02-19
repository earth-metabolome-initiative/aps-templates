//! Submodule defining functions to initialize `methanol` reagent models.

use super::reagent_model;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_physical_asset_models::*;
use aps::aps_users::User;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the methanol reagent model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let methanol_model1 =
///     methanol_hplc(&test_user, &mut conn).expect("Failed to create methanol model");
/// let methanol_model2 =
///     methanol_hplc(&test_user, &mut conn).expect("Failed to create methanol model");
/// assert_eq!(methanol_model1, methanol_model2);
/// ```
pub fn methanol_hplc<C>(
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
    const METHANOL_NAME: &str = "Methanol, >= 99.8%, HPLC grade";
    const METHANOL_DESCRIPTION: &str = "Methanol, >= 99.8%, HPLC grade";
    reagent_model(user, METHANOL_NAME, METHANOL_DESCRIPTION, conn)
}
