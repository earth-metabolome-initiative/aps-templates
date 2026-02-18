//! Submodule to initialize the freezer in the database.

use crate::prelude::reference_namespace;
use aps::aps_freezer_models::*;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the freezer.
///
/// # Implementation Details
///
/// This function either instantiate a new freezer from
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
/// let freezer1 = freezer(&test_user, &mut conn).expect("Failed to create the freezer model");
/// let freezer2 = freezer(&test_user, &mut conn).expect("Failed to create the freezer model");
/// assert_eq!(freezer1, freezer2);
/// ```
pub fn freezer<C>(
    user: &User,
    conn: &mut C,
) -> Result<NestedModel<freezer_models::table>, BuilderError<validation_errors::ValidationError>>
where
    TableBuilder<freezer_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<freezer_models::table, C>,
{
    const FREEZER_NAME: &str = "Freezer -80°C";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (FREEZER_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    freezer_models::table::builder()
        .try_name(FREEZER_NAME)
        .expect("Failed to set freezer model name")
        .try_description(
            "A Freezer -80°C used for long-term storage of samples or freezing of samples prior to freeze-drying steps",
        )
        .expect("Failed to set freezer model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
