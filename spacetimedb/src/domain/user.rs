use spacetimedb::{table, Identity};
use crate::domain::Binder;

/// The user information for the owner of one or more binders.
#[table(name = user)]
pub struct User {
    /// Unique identifier of the user.
    id: Identity,
    /// Display name of the binder owner.
    name: String,
    /// A list of binders that are designated as `Trade Binders`.
    trade_binders: Vec<Binder>,
    /// A list of all binders that are owned by this owner.
    binders: Vec<Binder>,
}

