//! Submodule defining functions to initialize `ethanol` reagent models.

use super::reagent_model;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_physical_asset_models::*;
use aps::aps_users::User;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the absolute ethanol reagent model, creating it if it does not
/// exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let ethanol_model1 =
///     absolute_ethanol(&test_user, &mut conn).expect("Failed to create ethanol model");
/// let ethanol_model2 =
///     absolute_ethanol(&test_user, &mut conn).expect("Failed to create ethanol model");
/// assert_eq!(ethanol_model1, ethanol_model2);
/// ```
pub fn absolute_ethanol<C>(
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
    const ETHANOL_NAME: &str = "Absolute Ethanol, >= 95%";
    const ETHANOL_DESCRIPTION: &str = "Absolute Ethanol, >= 95%, with 5% isopropanol";
    reagent_model(user, ETHANOL_NAME, ETHANOL_DESCRIPTION, conn)
}
