//! Submodule to initialize rack container models in the database.

use crate::prelude::reference_namespace;
use aps::aps_asset_models::TrySetAssetModelParentModelId;
use aps::aps_container_models::*;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

fn standard_rack<C>(
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
    const STANDARD_RACK_NAME: &str = "Rack";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (STANDARD_RACK_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    container_models::table::builder()
        .try_name(STANDARD_RACK_NAME)
        .expect("Failed to set container model name")
        .try_description("Rack, a common container for organizing samples")
        .expect("Failed to set container model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}

/// Returns the rack model for conical centrifugal tubes of 50ml, creating it
/// if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let rack1 = conical_centrifugal_tube_50ml_rack(&test_user, &mut conn).expect("Failed to create conical centrifugal tube rack model");
/// let rack2 = conical_centrifugal_tube_50ml_rack(&test_user, &mut conn).expect("Failed to create conical centrifugal tube rack model");
/// assert_eq!(rack1, rack2);
/// ```
pub fn conical_centrifugal_tube_50ml_rack<C>(
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
    const CCT_RACK_50ML_NAME: &str = "Conical Centrifugal Tube 50ml Rack";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (CCT_RACK_50ML_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    let standard_rack = standard_rack(user, conn)?;
    container_models::table::builder()
        .try_name(CCT_RACK_50ML_NAME)
        .expect("Failed to set container model name")
        .try_description("Rack for storing conical centrifugal tubes of 50ml")
        .expect("Failed to set container model description")
        .try_parent_model_id(standard_rack.get_column::<container_models::id>())
        .expect("Failed to set container model parent model")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
