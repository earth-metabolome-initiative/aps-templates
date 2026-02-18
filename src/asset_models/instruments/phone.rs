//! Submodule creating the instrument commercial product model for the Ball Mill
//! instrument.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_phone_device_models::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the smartphone device.
///
/// # Implementation Details
///
/// This function either instantiate a new smartphone from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the smartphone is being created.
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
/// let phone_model1 = phone_model(&test_user, &mut conn).expect("Failed to create the phone model");
/// let phone_model2 = phone_model(&test_user, &mut conn).expect("Failed to create the phone model");
/// assert_eq!(phone_model1, phone_model2);
/// ```
pub fn phone_model<C>(
    user: &User,
    conn: &mut C,
) -> Result<NestedModel<phone_device_models::table>, BuilderError<validation_errors::ValidationError>>
where
    TableBuilder<phone_device_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<phone_device_models::table, C>,
{
    const PHONE_NAME: &str = "Phone";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (PHONE_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    phone_device_models::table::builder()
        .try_name(PHONE_NAME)
        .expect("Failed to set phone model name")
        .try_description(
            "A phone (smartphone) which may be used to take pictures or as a positioning device.",
        )
        .expect("Failed to set phone model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
