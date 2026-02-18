//! Submodule to initialize pipette tip models in the database.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_pipette_tip_models::*;
use aps::aps_users::User;
use aps::aps_users::*;
use aps_traits::ReusabilityBuilderExt;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the 200μl pipette tip model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps::aps_physical_asset_models::*;
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// use diesel_builders::get_column::GetColumnExt;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let pipette_tip_200ul_1 = pipette_tip_200ul(&test_user, &mut conn).expect("Failed to create the 200μl pipette tip model");
/// let pipette_tip_200ul_2 = pipette_tip_200ul(&test_user, &mut conn).expect("Failed to create the 200μl pipette tip model");
/// assert_eq!(pipette_tip_200ul_1, pipette_tip_200ul_2);
/// assert_eq!(
///     pipette_tip_200ul_1.get_column::<physical_asset_models::lifecycle_class_id>(),
///     "single_use"
/// );
/// assert!(
///     pipette_tip_200ul_1
///         .get_column::<physical_asset_models::recommended_max_use>()
///         .is_none()
/// );
/// ```
pub fn pipette_tip_200ul<C>(
    user: &User,
    conn: &mut C,
) -> Result<NestedModel<pipette_tip_models::table>, BuilderError<validation_errors::ValidationError>>
where
    TableBuilder<pipette_tip_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<pipette_tip_models::table, C>,
{
    const PIPETTE_TIP_200UL_NAME: &str = "Pipette Tip 200μl";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (PIPETTE_TIP_200UL_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    pipette_tip_models::table::builder()
        .try_name(PIPETTE_TIP_200UL_NAME)
        .expect("Failed to set pipette tip model name")
        .try_description(
            "A 200μl pipette tip used to manipulate and transfer liquids when adapted to a pipette",
        )
        .expect("Failed to set pipette tip model description")
        .try_single_use()
        .expect("Failed to set pipette tip model reusability to single-use")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}

/// Returns the 1ml pipette tip model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps::aps_physical_asset_models::*;
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// use diesel_builders::get_column::GetColumnExt;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let pipette_tip_1000ul_1 = pipette_tip_1000ul(&test_user, &mut conn).expect("Failed to create the 1ml pipette tip model");
/// let pipette_tip_1000ul_2 = pipette_tip_1000ul(&test_user, &mut conn).expect("Failed to create the 1ml pipette tip model");
/// assert_eq!(pipette_tip_1000ul_1, pipette_tip_1000ul_2);
/// assert_eq!(
///     pipette_tip_1000ul_1.get_column::<physical_asset_models::lifecycle_class_id>(),
///     "single_use"
/// );
/// assert!(
///     pipette_tip_1000ul_1
///         .get_column::<physical_asset_models::recommended_max_use>()
///         .is_none()
/// );
/// ```
pub fn pipette_tip_1000ul<C>(
    user: &User,
    conn: &mut C,
) -> Result<NestedModel<pipette_tip_models::table>, BuilderError<validation_errors::ValidationError>>
where
    TableBuilder<pipette_tip_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<pipette_tip_models::table, C>,
{
    const PIPETTE_TIP_1000UL_NAME: &str = "Pipette Tip 1ml";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (PIPETTE_TIP_1000UL_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    pipette_tip_models::table::builder()
        .try_name(PIPETTE_TIP_1000UL_NAME)
        .expect("Failed to set pipette tip model name")
        .try_description(
            "A 1000μl pipette tip used to manipulate and transfer liquids when adapted to a pipette",
        )
        .expect("Failed to set pipette tip model description")
        .try_single_use()
        .expect("Failed to set pipette tip model reusability to single-use")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
