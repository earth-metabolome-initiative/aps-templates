//! Submodule defining functions to initialize `liquid_nitrogen` reagent models.

use super::reagent_model;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_physical_asset_models::*;
use aps::aps_users::User;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the liquid nitrogen reagent model, creating it if it does not
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
/// let liquid_nitrogen_model1 =
///     liquid_nitrogen(&test_user, &mut conn).expect("Failed to create liquid nitrogen model");
/// let liquid_nitrogen_model2 =
///     liquid_nitrogen(&test_user, &mut conn).expect("Failed to create liquid nitrogen model");
/// assert_eq!(liquid_nitrogen_model1, liquid_nitrogen_model2);
/// ```
pub fn liquid_nitrogen<C>(
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
    const LIQUID_NITROGEN_NAME: &str = "Liquid nitrogen";
    const LIQUID_NITROGEN_DESCRIPTION: &str = "Liquid nitrogen, pure";
    reagent_model(
        user,
        LIQUID_NITROGEN_NAME,
        LIQUID_NITROGEN_DESCRIPTION,
        conn,
    )
}
