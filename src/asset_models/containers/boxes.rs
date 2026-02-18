//! Submodule defining standard box and rack-like container models.

use crate::prelude::reference_namespace;
use aps::aps_container_models::*;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the polystyrene box model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let polystyrene_box1 = polystyrene_box(&test_user, &mut conn).expect("Failed to create polystyrene box model");
/// let polystyrene_box2 = polystyrene_box(&test_user, &mut conn).expect("Failed to create polystyrene box model");
/// assert_eq!(polystyrene_box1, polystyrene_box2);
/// ```
pub fn polystyrene_box<C>(
    user: &User,
    conn: &mut C,
) -> Result<NestedModel<container_models::table>, BuilderError<validation_errors::ValidationError>>
where
    TableBuilder<container_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<container_models::table, C>,
{
    const POLYSTYRENE_BOX_NAME: &str = "Polystyrene Box";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (POLYSTYRENE_BOX_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    container_models::table::builder()
        .try_name(POLYSTYRENE_BOX_NAME)
        .expect("Failed to set container model name")
        .try_description("Polystyrene box, a container typically used for liquid nitrogen")
        .expect("Failed to set container model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}

/// Returns the vial rack 1.5ml model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let vial_rack1 = vial_rack_1_5ml(&test_user, &mut conn).expect("Failed to create vial rack model");
/// let vial_rack2 = vial_rack_1_5ml(&test_user, &mut conn).expect("Failed to create vial rack model");
/// assert_eq!(vial_rack1, vial_rack2);
/// ```
pub fn vial_rack_1_5ml<C>(
    user: &User,
    conn: &mut C,
) -> Result<NestedModel<container_models::table>, BuilderError<validation_errors::ValidationError>>
where
    TableBuilder<container_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<container_models::table, C>,
{
    const VIAL_RACK_1_5ML_NAME: &str = "Vial Rack 1.5ml (9x9)";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (VIAL_RACK_1_5ML_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    container_models::table::builder()
        .try_name(VIAL_RACK_1_5ML_NAME)
        .expect("Failed to set container model name")
        .try_description("Vial box, a container typically used for storing vials")
        .expect("Failed to set container model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
