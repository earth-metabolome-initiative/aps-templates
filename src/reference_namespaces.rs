use aps::aps_namespaces::*;
use aps::aps_ownables::*;
use aps::aps_users::*;
use diesel_builders::BuilderError;
use diesel_builders::prelude::*;

/// Returns the reference namespace.
///
/// # Arguments
///
/// * `user` - The user creating the namespace.
/// * `conn` - A mutable reference to the database connection where the
///   namespace will be created.
///
/// # Panics
///
/// * If the namespace creation fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let reference_namespace1 = reference_namespace(&test_user, &mut conn).expect("Failed to create the reference namespace");
/// let reference_namespace2 = reference_namespace(&test_user, &mut conn).expect("Failed to create the reference namespace");
/// assert_eq!(reference_namespace1, reference_namespace2);
/// ```
pub fn reference_namespace<C>(
    user: &User,
    conn: &mut C,
) -> Result<NestedModel<namespaces::table>, BuilderError<validation_errors::ValidationError>>
where
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
{
    const REFERENCE_NAMESPACE_NAME: &str = "aps_reference";

    if let Ok(existing) =
        <(namespaces::name,)>::load_nested_first((REFERENCE_NAMESPACE_NAME,), conn)
    {
        return Ok(existing);
    }

    namespaces::table::builder()
        .try_name(REFERENCE_NAMESPACE_NAME)
        .expect("Failed to set namespace name")
        .owner_id(user.get_column::<users::id>())
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .insert_nested(conn)
}
