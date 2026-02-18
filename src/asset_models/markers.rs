//! Submodule to initialize the `markers` in the database.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_physical_asset_models::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the marker model for cardboard arrows, creating it if it does not
/// exist.
///
/// # Arguments
///
/// * `user` - The user who is creating the marker model.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the insertion fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let marker_arrow_model1 = marker_arrow_model(&test_user, &mut conn).expect("Failed to create the marker arrow model");
/// let marker_arrow_model2 = marker_arrow_model(&test_user, &mut conn).expect("Failed to create the marker arrow model");
/// assert_eq!(marker_arrow_model1, marker_arrow_model2);
/// ```
pub fn marker_arrow_model<C>(
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
    const MARKER_ARROW: &str = "Marker Arrow";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (MARKER_ARROW,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    physical_asset_models::table::builder()
        .try_name(MARKER_ARROW)
        .expect("Failed to set physical asset model name")
        .try_description("Marker arrow to highlight in a photograph a subject of interest.")
        .expect("Failed to set physical asset model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
