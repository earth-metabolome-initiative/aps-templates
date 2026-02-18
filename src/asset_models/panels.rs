//! Submodule to initialize the `panels` in the database.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_physical_asset_models::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the panel model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let panel_model1 = panel_model(&test_user, &mut conn).expect("Failed to create the panel model");
/// let panel_model2 = panel_model(&test_user, &mut conn).expect("Failed to create the panel model");
/// assert_eq!(panel_model1, panel_model2);
/// ```
pub fn panel_model<C>(
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
    const PANEL_NAME: &str = "Panel";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (PANEL_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    physical_asset_models::table::builder()
        .try_name(PANEL_NAME)
        .expect("Failed to set physical asset model name")
        .try_description("Panel for documenting organisms, typically used in botanical gardens.")
        .expect("Failed to set physical asset model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
