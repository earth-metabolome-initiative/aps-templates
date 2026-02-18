//! Submodule to initialize vial cap (container sealer) models in the database.

use crate::prelude::reference_namespace;
use aps::aps_container_sealer_models::*;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the splitted cap for vial 1.5ml model, creating it if it does not
/// exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let split1 = splitted_cap_vial_1_5ml(&test_user, &mut conn).expect("Failed to create splitted cap model");
/// let split2 = splitted_cap_vial_1_5ml(&test_user, &mut conn).expect("Failed to create splitted cap model");
/// assert_eq!(split1, split2);
/// ```
pub fn splitted_cap_vial_1_5ml<C>(
    user: &User,
    conn: &mut C,
) -> Result<
    NestedModel<container_sealer_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<container_sealer_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<container_sealer_models::table, C>,
{
    const SPLITTED_CAP_NAME: &str = "Splitted Cap for Vial 1.5ml";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (SPLITTED_CAP_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    container_sealer_models::table::builder()
        .try_name(SPLITTED_CAP_NAME)
        .expect("Failed to set container sealer model name")
        .try_description("Splitted cap for Vial of 1.5 ml used for extracts storage")
        .expect("Failed to set container sealer model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}

/// Returns the sealed cap for vial 1.5ml model, creating it if it does not
/// exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let sealed1 = sealed_cap_vial_1_5ml(&test_user, &mut conn).expect("Failed to create sealed cap model");
/// let sealed2 = sealed_cap_vial_1_5ml(&test_user, &mut conn).expect("Failed to create sealed cap model");
/// assert_eq!(sealed1, sealed2);
/// ```
pub fn sealed_cap_vial_1_5ml<C>(
    user: &User,
    conn: &mut C,
) -> Result<
    NestedModel<container_sealer_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<container_sealer_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<container_sealer_models::table, C>,
{
    const SEALED_CAP_NAME: &str = "Sealed Cap for Vial 1.5ml";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (SEALED_CAP_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    container_sealer_models::table::builder()
        .try_name(SEALED_CAP_NAME)
        .expect("Failed to set container sealer model name")
        .try_description("Sealed cap for Vial of 1.5 ml used for extracts storage")
        .expect("Failed to set container sealer model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
