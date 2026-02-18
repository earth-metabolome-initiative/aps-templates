//! Submodule to initialize vial models in the database.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use aps::aps_volumetric_container_models::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the vial 1.5ml model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let vial1 = vial_1_5ml(&test_user, &mut conn).expect("Failed to create vial model");
/// let vial2 = vial_1_5ml(&test_user, &mut conn).expect("Failed to create vial model");
/// assert_eq!(vial1, vial2);
/// ```
pub fn vial_1_5ml<C>(
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
    const VIAL_1_5ML_NAME: &str = "Vial 1.5ml";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (VIAL_1_5ML_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    volumetric_container_models::table::builder()
        .try_name(VIAL_1_5ML_NAME)
        .expect("Failed to set volumetric container model name")
        .try_description("Vial of 1.5 ml used for extracts storage")
        .expect("Failed to set volumetric container model description")
        .try_volume(0.0015_f32)
        .expect("Failed to set volumetric container model volume")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
