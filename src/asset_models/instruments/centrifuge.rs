//! Submodule to initialize the centrifuge model in the database.

use crate::prelude::reference_namespace;
use aps::aps_centrifuge_models::*;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the centrifuge model, creating it if it does not exist.
///
/// # Arguments
///
/// * `user` - The user creating the model.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If insertion fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let centrifuge1 = safelock_centrifuge(&test_user, &mut conn)
///     .expect("Failed to create the centrifuge model");
/// let centrifuge2 = safelock_centrifuge(&test_user, &mut conn)
///     .expect("Failed to create the centrifuge model");
/// assert_eq!(centrifuge1, centrifuge2);
/// ```
pub fn safelock_centrifuge<C>(
    user: &User,
    conn: &mut C,
) -> Result<NestedModel<centrifuge_models::table>, BuilderError<validation_errors::ValidationError>>
where
    TableBuilder<centrifuge_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<centrifuge_models::table, C>,
{
    const CENTRIFUGE_NAME: &str = "Safelock Centrifuge";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (CENTRIFUGE_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    centrifuge_models::table::builder()
        .try_name(CENTRIFUGE_NAME)
        .expect("Failed to set centrifuge model name")
        .try_description("Safelock centrifuge, used to precipitate solid material.")
        .expect("Failed to set centrifuge model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
