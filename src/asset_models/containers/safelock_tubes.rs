//! Submodule to initialize safelock tube models in the database.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use aps::aps_volumetric_container_models::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the safelock tube 2ml model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let safelock1 = safelock_tubes_2ml(&test_user, &mut conn).expect("Failed to create safelock tube model");
/// let safelock2 = safelock_tubes_2ml(&test_user, &mut conn).expect("Failed to create safelock tube model");
/// assert_eq!(safelock1, safelock2);
/// ```
pub fn safelock_tubes_2ml<C>(
    user: &User,
    conn: &mut C,
) -> Result<
    NestedModel<volumetric_container_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<volumetric_container_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<volumetric_container_models::table, C>,
{
    const SAFELOCK_2ML_NAME: &str = "Safelock Tube 2ml";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (SAFELOCK_2ML_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    volumetric_container_models::table::builder()
        .try_name(SAFELOCK_2ML_NAME)
        .expect("Failed to set volumetric container model name")
        .try_description("Safelock tube of 2ml, used for sample extraction.")
        .expect("Failed to set volumetric container model description")
        .try_volume(0.002_f32)
        .expect("Failed to set volumetric container model volume")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
