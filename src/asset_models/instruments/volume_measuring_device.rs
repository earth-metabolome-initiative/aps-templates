//! Submodule creating the instrument commercial product model for the Measuring
//! devices model.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use aps::aps_volume_measuring_device_models::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};
/// Returns the volume measuring device model instance.
///
/// # Implementation Details
///
/// This function either instantiate a new volume measuring device model in
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the conical tube is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let volume_measuring_device_model1 = volume_measuring_device_model(&test_user, &mut conn)
///     .expect("Failed to create the volume measuring device model");
/// let volume_measuring_device_model2 = volume_measuring_device_model(&test_user, &mut conn)
///     .expect("Failed to create the volume measuring device model");
/// assert_eq!(volume_measuring_device_model1, volume_measuring_device_model2);
/// ```
pub fn volume_measuring_device_model<C>(
    user: &User,
    conn: &mut C,
) -> Result<
    NestedModel<volume_measuring_device_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<volume_measuring_device_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<volume_measuring_device_models::table, C>,
{
    const VOLUME_MEASURING_DEVICE_NAME: &str = "Volume Measuring Device";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (VOLUME_MEASURING_DEVICE_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    volume_measuring_device_models::table::builder()
        .try_name(VOLUME_MEASURING_DEVICE_NAME)
        .expect("Failed to set volume measuring device model name")
        .try_description("A generic volume measuring device")
        .expect("Failed to set volume measuring device model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
