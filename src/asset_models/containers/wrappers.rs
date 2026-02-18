//! Submodule to initialize wrappers (packaging models) in the database.

use crate::prelude::reference_namespace;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_packaging_models::*;
use aps::aps_users::User;
use aps::aps_users::*;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the coffee filter wrapper model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let wrapper1 = coffee_filter_wrapper(&test_user, &mut conn).expect("Failed to create coffee filter wrapper model");
/// let wrapper2 = coffee_filter_wrapper(&test_user, &mut conn).expect("Failed to create coffee filter wrapper model");
/// assert_eq!(wrapper1, wrapper2);
/// ```
pub fn coffee_filter_wrapper<C>(
    user: &User,
    conn: &mut C,
) -> Result<NestedModel<packaging_models::table>, BuilderError<validation_errors::ValidationError>>
where
    TableBuilder<packaging_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<packaging_models::table, C>,
{
    const COFFEE_FILTER_WRAPPER_NAME: &str = "Coffee Filter Wrapper";

    let reference_namespace = reference_namespace(user, conn)?;
    if let Ok(existing) = <(
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    )>::load_nested_first(
        (
            reference_namespace.get_column::<namespaces::id>(),
            (COFFEE_FILTER_WRAPPER_NAME,),
        ),
        conn,
    ) {
        return Ok(existing);
    }

    packaging_models::table::builder()
        .try_name(COFFEE_FILTER_WRAPPER_NAME)
        .expect("Failed to set packaging model name")
        .try_description(
            "Coffee filters used to wrap sample in the field prior to storage in Falcon tubes",
        )
        .expect("Failed to set packaging model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(reference_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
}
