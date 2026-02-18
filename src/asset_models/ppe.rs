//! Submodule to initialize the `ppe` in the database.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_personal_protective_equipment_models::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::BuilderError;
use diesel_builders::{TableBuilder, prelude::*};

/// Returns the PPE model for gloves, creating it if it does not
/// exist.
///
/// # Arguments
///
/// * `user` - The user who is creating the PPE model.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the insertion fails.
///
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let glove_model1 = glove_model(&test_user, &mut conn).expect("Failed to create the glove model");
/// let glove_model2 = glove_model(&test_user, &mut conn).expect("Failed to create the glove model");
/// assert_eq!(glove_model1, glove_model2);
/// ```
pub fn glove_model<C>(
    user: &User,
    conn: &mut C,
) -> Result<
    NestedModel<personal_protective_equipment_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<personal_protective_equipment_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<personal_protective_equipment_models::table, C>,
{
    const GLOVES_NAME: &str = "Latex Gloves";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (GLOVES_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    personal_protective_equipment_models::table::builder()
        .try_name(GLOVES_NAME)
        .expect("Failed to set physical asset model name")
        .try_description("Latex or nitrile gloves used for personal protection.")
        .expect("Failed to set physical asset model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
