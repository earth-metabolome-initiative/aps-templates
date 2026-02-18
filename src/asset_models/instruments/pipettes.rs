//! Submodule to initialize pipette-related models in the database.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use aps::aps_volume_measuring_device_models::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the 200μl pipette model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let pipette_200ul_1 = pipette_200ul(&test_user, &mut conn).expect("Failed to create the 200μl pipette model");
/// let pipette_200ul_2 = pipette_200ul(&test_user, &mut conn).expect("Failed to create the 200μl pipette model");
/// assert_eq!(pipette_200ul_1, pipette_200ul_2);
/// ```
pub fn pipette_200ul<C>(
    user: &User,
    conn: &mut C,
) -> Result<
    NestedModel<volume_measuring_device_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<volume_measuring_device_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<volume_measuring_device_models::table, C>,
{
    const PIPETTE_200UL_NAME: &str = "Pipette 200μl";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (PIPETTE_200UL_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    volume_measuring_device_models::table::builder()
        .try_name(PIPETTE_200UL_NAME)
        .expect("Failed to set pipette model name")
        .try_description(
            "A pipette used to manipulate liquids (needs to be equipped with a pipette tip).",
        )
        .expect("Failed to set pipette model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}

/// Returns the 1000µl pipette model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let pipette_1000ul_1 = pipette_1000ul(&test_user, &mut conn).expect("Failed to create the 1000µl pipette model");
/// let pipette_1000ul_2 = pipette_1000ul(&test_user, &mut conn).expect("Failed to create the 1000µl pipette model");
/// assert_eq!(pipette_1000ul_1, pipette_1000ul_2);
/// ```
pub fn pipette_1000ul<C>(
    user: &User,
    conn: &mut C,
) -> Result<
    NestedModel<volume_measuring_device_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<volume_measuring_device_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<volume_measuring_device_models::table, C>,
{
    const PIPETTE_1000UL_NAME: &str = "Pipette 1000µl";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (PIPETTE_1000UL_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    volume_measuring_device_models::table::builder()
        .try_name(PIPETTE_1000UL_NAME)
        .expect("Failed to set pipette model name")
        .try_description(
            "A pipette used to manipulate liquids (needs to be equipped with a pipette tip).",
        )
        .expect("Failed to set pipette model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
