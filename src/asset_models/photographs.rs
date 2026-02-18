//! Submodule to initialize the `photographs` in the database.

use crate::prelude::reference_namespace;
use aps::aps_digital_asset_models::*;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the photograph asset model, creating it if it does not exist.
///
/// # Arguments
///
/// * `user` - The user who is creating the digital asset model.
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
/// let photograph_model1 = photograph_model(&test_user, &mut conn).expect("Failed to create the photograph model");
/// let photograph_model2 = photograph_model(&test_user, &mut conn).expect("Failed to create the photograph model");
/// assert_eq!(photograph_model1, photograph_model2);
/// ```
pub fn photograph_model<C>(
    user: &User,
    conn: &mut C,
) -> Result<
    NestedModel<digital_asset_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<digital_asset_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<digital_asset_models::table, C>,
{
    const PHOTOGRAPH_NAME: &str = "Photograph";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (PHOTOGRAPH_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    digital_asset_models::table::builder()
        .try_name(PHOTOGRAPH_NAME)
        .expect("Failed to set digital asset model name")
        .try_description("Photograph for documenting organisms and their habitats")
        .expect("Failed to set digital asset model description")
        .try_mime_type("image/jpeg")
        .expect("Failed to set digital asset model mime type")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
