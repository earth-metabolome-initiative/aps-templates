//! Submodule to initialize bead models in the database.

use crate::prelude::reference_namespace;
use aps::aps_bead_models::*;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::User;
use aps::aps_users::*;
use aps_traits::ReusabilityBuilderExt;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the 3mm metal bead model, creating it if it does not exist.
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
/// let bead_3mm_1 = bead_3mm(&test_user, &mut conn).expect("Failed to create the 3mm bead model");
/// let bead_3mm_2 = bead_3mm(&test_user, &mut conn).expect("Failed to create the 3mm bead model");
/// assert_eq!(bead_3mm_1, bead_3mm_2);
/// assert_eq!(
///     bead_3mm_1.get_column::<physical_asset_models::lifecycle_class_id>(),
///     "reusable"
/// );
/// assert_eq!(
///     bead_3mm_1.get_column::<physical_asset_models::recommended_max_use>(),
///     Some(3)
/// );
/// ```
pub fn bead_3mm<C>(
    user: &User,
    conn: &mut C,
) -> Result<NestedModel<bead_models::table>, BuilderError<validation_errors::ValidationError>>
where
    TableBuilder<bead_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<bead_models::table, C>,
{
    const METAL_BEAD_3MM_NAME: &str = "Metal Bead 3mm";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (METAL_BEAD_3MM_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    bead_models::table::builder()
        .try_name(METAL_BEAD_3MM_NAME)
        .expect("Failed to set bead model name")
        .try_description("Metal bead of 3mm used primarily in ball milling procedures.")
        .expect("Failed to set bead model description")
        .try_diameter(3.0_f32)
        .expect("Failed to set bead model diameter")
        .try_reusable(3)
        .expect("Failed to set bead model reusability")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
