//! Submodule to initialize the ball mill machine model in the database.

use crate::prelude::reference_namespace;
use aps::aps_ball_mill_machine_models::*;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the ball mill machine model, creating it if it does not exist.
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
/// let ball_mill_machine1 = ball_mill_machine(&test_user, &mut conn)
///     .expect("Failed to create the ball mill machine model");
/// let ball_mill_machine2 = ball_mill_machine(&test_user, &mut conn)
///     .expect("Failed to create the ball mill machine model");
/// assert_eq!(ball_mill_machine1, ball_mill_machine2);
/// ```
pub fn ball_mill_machine<C>(
    user: &User,
    conn: &mut C,
) -> Result<
    NestedModel<ball_mill_machine_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<ball_mill_machine_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<ball_mill_machine_models::table, C>,
{
    const BALL_MILL_MACHINE_NAME: &str = "Ball Mill Machine";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (BALL_MILL_MACHINE_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    ball_mill_machine_models::table::builder()
        .try_name(BALL_MILL_MACHINE_NAME)
        .expect("Failed to set ball mill machine model name")
        .try_description("A Ball Mill Machine used to grind samples into powder.")
        .expect("Failed to set ball mill machine model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
