//! Submodule to initialize the freeze dryer in the database.

use crate::prelude::reference_namespace;
use aps::aps_freeze_dryer_models::*;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the freeze dryer.
///
/// # Implementation Details
///
/// This function either instantiate a new freeze dryer from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the freeze dryer is being created.
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
/// let freeze_dryer1 = freeze_dryer(&test_user, &mut conn).expect("Failed to create the freeze dryer model");
/// let freeze_dryer2 = freeze_dryer(&test_user, &mut conn).expect("Failed to create the freeze dryer model");
/// assert_eq!(freeze_dryer1, freeze_dryer2);
/// ```
pub fn freeze_dryer<C>(
    user: &User,
    conn: &mut C,
) -> Result<NestedModel<freeze_dryer_models::table>, BuilderError<validation_errors::ValidationError>>
where
    TableBuilder<freeze_dryer_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<freeze_dryer_models::table, C>,
{
    const FREEZE_DRYER_NAME: &str = "Freeze dryer";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (FREEZE_DRYER_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    freeze_dryer_models::table::builder()
        .try_name(FREEZE_DRYER_NAME)
        .expect("Failed to set freeze dryer model name")
        .try_description(
            "A freeze dryer (or lyophilisator) used to sublimate water content of samples.",
        )
        .expect("Failed to set freeze dryer model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
