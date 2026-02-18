//! Submodule creating the instrument commercial product model for the Pipette
//! 200 instrument.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use aps::aps_weighing_device_models::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};
/// Returns the weighing scale.
///
/// # Implementation Details
///
/// This function either instantiate a new weighing scale from
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
/// let weighing_scale1 = weighing_scale(&test_user, &mut conn).expect("Failed to create the weighing scale model");
/// let weighing_scale2 = weighing_scale(&test_user, &mut conn).expect("Failed to create the weighing scale model");
/// assert_eq!(weighing_scale1, weighing_scale2);
/// ```
pub fn weighing_scale<C>(
    user: &User,
    conn: &mut C,
) -> Result<
    NestedModel<weighing_device_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<weighing_device_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<weighing_device_models::table, C>,
{
    const WEIGHING_SCALE_NAME: &str = "Weighing Scale";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (WEIGHING_SCALE_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    weighing_device_models::table::builder()
        .try_name(WEIGHING_SCALE_NAME)
        .expect("Failed to set weighing scale model name")
        .try_description("A weighing scale used to measure the amount of samples.")
        .expect("Failed to set weighing scale model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
