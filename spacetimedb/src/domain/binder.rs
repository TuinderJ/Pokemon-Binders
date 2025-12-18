use crate::domain::{
    user::{user, User},
    AcquiredCard, Card, Offer,
};
use anyhow::{anyhow, Result};
use spacetimedb::{table, ReducerContext, SpacetimeType, Table, TryInsertError};

/// This denotes a binder. The owner of the binder can add a list of desired cards, desired Pokémon, cards they've acquired, or Pokémon they've acquired.
#[table(name = binder)]
pub struct Binder {
    /// Unique identifier of the binder.
    #[primary_key]
    pub id: String,
    /// The owner of the binder.
    owner: User,
    /// The display name of the binder.
    name: String,
    /// A list of Pokémon that the owner wants to collect in this binder.
    desired_pokemons: Vec<Pokemon>,
    /// A list of Pokémon that the owner has collected in this binder.
    acquired_pokemons: Vec<AcquiredPokemon>,
    /// A list of white-lists and/or black-lists for denoting what the owner is looking for.
    /// There can be 1 list for each Pokémon that the owner has on their `desired` list.
    pokemon_restrictions: Vec<PokemonRestriction>,
    /// A list of cards that the owner wants to collect in this binder.
    desired_cards: Vec<Card>,
    /// A list of cards that the owner has collected in this binder.
    acquired_cards: Vec<AcquiredCard>,
    /// A list of offers from viewers of the binder.
    offers: Vec<Offer>,
}

impl Binder {
    pub fn new(id: String, name: String, owner: User) -> Self {
        Self {
            id,
            owner,
            name,
            desired_pokemons: Vec::new(),
            acquired_pokemons: Vec::new(),
            pokemon_restrictions: Vec::new(),
            desired_cards: Vec::new(),
            acquired_cards: Vec::new(),
            offers: Vec::new(),
        }
    }
}

#[spacetimedb::reducer]
pub fn new_binder(ctx: &ReducerContext, id: String, name: String, owner_id: u32) -> Result<()> {
    let owner = ctx.db.user().id().find(owner_id).unwrap();
    let binder = Binder::new(id, name, owner);
    if let Err(error) = ctx.db.binder().try_insert(binder) {
        match error {
            TryInsertError::UniqueConstraintViolation(_) => {
                Err(anyhow::anyhow!("Binder already exists"))
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
pub fn delete_binder(ctx: &ReducerContext, id: String) -> Result<()> {
    if ctx.db.binder().id().delete(id) {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Binder does not exist"))
    }
}

#[spacetimedb::reducer]
pub fn change_binder_owner(
    ctx: &ReducerContext,
    binder_id: String,
    new_owner_id: u32,
) -> Result<()> {
    let table = ctx.db.binder();
    match table.id().find(binder_id) {
        Some(mut binder) => {
            match ctx.db.user().id().find(new_owner_id) {
                Some(owner) => {
                    binder.owner = owner;
                    table.id().update(binder);
                }
                None => {
                    return Err(anyhow!("User was not found"));
                }
            };
        }
        None => return Err(anyhow!("Binder was not found")),
    };

    Ok(())
}

/// A helper struct to be able to select a card or cards that have been acquired when the owner is searching for Pokémon.
#[derive(SpacetimeType)]
struct AcquiredPokemon {
    /// Generic Pokémon selection.
    pokemon: Pokemon,
    /// Specific cards that are in the binder.
    cards_acquired: Vec<AcquiredCard>,
}

/// Information about a specific Pokémon.
#[table(name = pokemon)]
pub struct Pokemon {
    /// National Pokedex Number of the Pokémon.
    #[primary_key]
    pub id: u16,
    /// Name of the Pokémon
    name: String,
    /// URL of the image
    sprite: String,
}

impl Pokemon {
    pub fn new(id: u16, name: String, sprite: String) -> Self {
        Self { id, name, sprite }
    }
}

#[spacetimedb::reducer]
pub fn add_pokemon(ctx: &ReducerContext, id: u16, name: String, sprite: String) {
    ctx.db
        .pokemon()
        .try_insert(Pokemon::new(id, name, sprite))
        .unwrap();
}

/// A white-list or black-list for denoting which cards the owner deems acceptable for a specific Pokémon.
#[derive(SpacetimeType)]
struct PokemonRestriction {
    /// This points to the specific Pokémon that this list is for.
    pokemon: Pokemon,
    /// Whitelist or blacklist to designate if the owner wants to include or exclude specific cards.
    list_type: ListType,
    /// Cards that the binder owner does or does not want to count toward a desired Pokémon.
    list: Vec<Card>,
}

#[derive(SpacetimeType, Debug, Copy, Clone)]
enum ListType {
    WhiteList,
    BlackList,
}
