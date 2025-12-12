use spacetimedb::{table, SpacetimeType};
use crate::domain::{AcquiredCard, Card, Offer, User};

/// This denotes a binder. The owner of the binder can add a list of desired cards, desired Pokemon, cards they've acquired, or Pokemon they've acquired.
#[table(name = binder)]
pub struct Binder {
    /// Unique identifier of the binder.
    id: String,
    /// The owner of the binder.
    owner: User,
    /// The display name of the binder.
    name: String,
    /// A list of Pokemon that the owner wants to collect in this binder.
    desired_pokemons: Vec<Pokemon>,
    /// A list of Pokemon that the owner has collected in this binder.
    acquired_pokemons: Vec<AcquiredPokemon>,
    /// A list of white-lists and/or black-lists for denoting what the owner is looking for.
    /// There can be 1 list for each Pokemon that the owner has on their `desired` list.
    pokemon_restrictions: Vec<PokemonRestriction>,
    /// A list of cards that the owner wants to collect in this binder.
    desired_cards: Vec<Card>,
    /// A list of cards that the owner has collected in this binder.
    acquired_cards: Vec<AcquiredCard>,
    /// A list of offers from viewers of the binder.
    offers: Vec<Offer>,
}

/// A helper struct to be able to select a card or cards that have been acquired when the owner is searching for Pokemon.
#[table(name = acquired_pokemon)]
struct AcquiredPokemon {
    /// Generic Pokemon selection.
    pokemon: Pokemon,
    /// Specific cards that are in the binder.
    cards_acquired: Vec<AcquiredCard>,
}

/// Information about a specific Pokemon.
#[table(name = pokemon)]
struct Pokemon {
    /// National Pokedex Number of the Pokemon.
    id: i16,
    /// Name of the Pokemon
    name: String,
}

/// A white-list or black-list for denoting which cards the owner deems acceptable for a specific Pokemon.
#[table(name = pokemon_restriction)]
struct PokemonRestriction {
    /// This points to the specific Pokemon that this list is for.
    pokemon: Pokemon,
    /// Whitelist or blacklist to designate if the owner wants to include or exclude specific cards.
    list_type: ListType,
    /// Cards that the binder owner does or does not want to count toward a desired Pokemon.
    list: Vec<Card>,
}

#[derive(SpacetimeType, Debug, Copy, Clone)]
enum ListType {
    WhiteList,
    BlackList,
}

