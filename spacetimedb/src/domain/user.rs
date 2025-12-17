use anyhow::Result;
use spacetimedb::{table, ReducerContext, Table, TryInsertError};

/// The user information for the owner of one or more binders.
#[table(name = user)]
pub struct User {
    /// Unique identifier of the user.
    #[primary_key]
    #[auto_inc]
    pub id: u32,
    /// Display name of the binder owner.
    name: String,
    /// Email assigned to the user account.
    #[unique]
    pub email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self { id: 0, name, email }
    }
}

#[spacetimedb::reducer]
pub fn add_user(ctx: &ReducerContext, name: String, email: String) -> Result<()> {
    let user = User::new(name, email);

    if let Err(error) = ctx.db.user().try_insert(user) {
        match error {
            TryInsertError::UniqueConstraintViolation(_) => {
                Err(anyhow::anyhow!("User already exists"))
            }
            TryInsertError::AutoIncOverflow(_) => Err(anyhow::anyhow!(
                "Something went wrong. Contact site administrator"
            )),
        }
    } else {
        Ok(())
    }
}

#[spacetimedb::reducer]
pub fn delete_user(ctx: &ReducerContext, id: u32) -> Result<()> {
    if ctx.db.user().id().delete(&id) {
        Ok(())
    } else {
        Err(anyhow::anyhow!("User not found"))
    }
}
