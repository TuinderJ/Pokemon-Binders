use spacetimedb::{table, Identity, SpacetimeType};
use crate::domain::{AcquiredCard, Card, Offer};

/// This denotes a binder. The owner of the binder can add a list of desired cards, desired Pokémon, cards they've acquired, or Pokémon they've acquired.
#[table(name = binder)]
pub struct Binder {
    /// Unique identifier of the binder.
    #[primary_key]
    pub id: String,
    /// The owner of the binder.
    owner: Identity,
    /// The display name of the binder.
    name: String,
    /// A list of Pokémon that the owner wants to collect in this binder.
    desired_pokemon_ids: Vec<String>,
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
    pub id: i16,
    /// Name of the Pokémon
    name: String,
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

