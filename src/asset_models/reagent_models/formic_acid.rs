//! Submodule defining functions to initialize `formic_acid` reagent models.

use super::reagent_model;
use aps::aps_namespaced_ownables::*;
use aps::aps_namespaces::*;
use aps::aps_physical_asset_models::*;
use aps::aps_users::User;
use diesel_builders::{BuilderError, TableBuilder, prelude::*};

/// Returns the formic acid reagent model, creating it if it does not exist.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_git_conn, user};
/// use aps_templates::prelude::*;
/// let mut conn = aps_git_conn();
///
/// let test_user = user(&mut conn);
/// let formic_acid_model1 =
///     formic_acid(&test_user, &mut conn).expect("Failed to create formic acid model");
/// let formic_acid_model2 =
///     formic_acid(&test_user, &mut conn).expect("Failed to create formic acid model");
/// assert_eq!(formic_acid_model1, formic_acid_model2);
/// ```
pub fn formic_acid<C>(
    user: &User,
    conn: &mut C,
) -> Result<
    NestedModel<physical_asset_models::table>,
    BuilderError<validation_errors::ValidationError>,
>
where
    TableBuilder<physical_asset_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (
        namespaced_ownables::namespace_id,
        (namespaced_ownables::name,),
    ): LoadNestedFirst<physical_asset_models::table, C>,
{
    const FORMIC_ACID_NAME: &str = "Formic acid";
    const FORMIC_ACID_DESCRIPTION: &str = "Formic acid, pure";
    reagent_model(user, FORMIC_ACID_NAME, FORMIC_ACID_DESCRIPTION, conn)
}
