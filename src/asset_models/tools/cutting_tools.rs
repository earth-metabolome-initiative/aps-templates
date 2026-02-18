//! Submodule to initialize the `cutting_tools` in the database.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_physical_asset_models::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the physical asset model for a scalpel, creating it if it does not
/// exist.
///
/// # Arguments
///
/// * `user` - The user who is creating the physical asset model.
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
/// let scalpel_model1 = scalpel_model(&test_user, &mut conn).expect("Failed to create the scalpel model");
/// let scalpel_model2 = scalpel_model(&test_user, &mut conn).expect("Failed to create the scalpel model");
/// assert_eq!(scalpel_model1, scalpel_model2);
/// ```
pub fn scalpel_model<C>(
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
    const SCALPEL_NAME: &str = "Scalpel";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (SCALPEL_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    physical_asset_models::table::builder()
        .try_name(SCALPEL_NAME)
        .expect("Failed to set physical asset model name")
        .try_description("A scalpel used to cut samples.")
        .expect("Failed to set physical asset model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}

/// Returns the physical asset model for a pair of scissors, creating it if it
/// does not exist.
///
/// # Arguments
///
/// * `user` - The user who is creating the physical asset model.
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
/// let scissor_model1 = scissor_model(&test_user, &mut conn).expect("Failed to create the scissor model");
/// let scissor_model2 = scissor_model(&test_user, &mut conn).expect("Failed to create the scissor model");
/// assert_eq!(scissor_model1, scissor_model2);
/// ```
pub fn scissor_model<C>(
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
    const SCISSORS_NAME: &str = "Scissors";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (SCISSORS_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    physical_asset_models::table::builder()
        .try_name(SCISSORS_NAME)
        .expect("Failed to set physical asset model name")
        .try_description("A pair of scissors.")
        .expect("Failed to set physical asset model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
