/// This denotes a binder. The owner of the binder can add a list of desired cards, desired Pokemon, cards they've acquired, or Pokemon they've acquired.
#[derive()]
struct Binder {
    /// Unique identifier of the binder.
    id: String,
    /// The owner of the binder.
    owner: Owner,
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
}

/// The user information for the owner of one or more binders.
struct Owner {
    /// Unique identifier of the binder owner.
    id: String,
    /// Display name of the binder owner.
    name: String,
    /// A list of binders that are designated as `Trade Binders`.
    trade_binders: Vec<Binder>,
    /// A list of all binders that are owned by this owner.
    binders: Vec<Binder>,
}

/// A helper struct to be able to select a card or cards that have been acquired when the owner is searching for Pokemon.
struct AcquiredPokemon {
    /// Generic Pokemon selection.
    pokemon: Pokemon,
    /// Specific cards that are in the binder.
    cards_acquired: Vec<AcquiredCard>,
}

/// Information about a specific Pokemon.
struct Pokemon {
    /// National Pokedex Number of the Pokemon.
    id: i16,
    /// Name of the Pokemon
    name: String,
}

/// A white-list or black-list for denoting which cards the owner deems acceptable for a specific Pokemon.
struct PokemonRestriction {
    /// This points to the specific Pokemon that this list is for.
    pokemon: Pokemon,
    /// Whitelist or blacklist to designate if the owner wants to include or exclude specific cards.
    list_type: ListType,
    /// Cards that the binder owner does or does not want to count toward a desired Pokemon.
    list: Vec<Card>,
}

enum ListType {
    WhiteList,
    BlackList,
}

/// This is to denote when a binder owner has acquired a specific card.
/// More than one of the same card can be collected in the binder.
struct AcquiredCard {
    /// A specific Pokemon Card.
    card: Card,
    /// Quantity of this card in the binder.
    qty_acquired: i32,
}

/// Information about a specific Pokemon Card.
struct Card {
    /// Unique identifier for the card.
    id: String,
    /// The supertype of the card, such as Pokémon, Energy, or Trainer.
    supertype: String,
    /// A list of subtypes, such as Basic, EX, Mega, Rapid Strike, etc.
    subtypes: Vec<String>,
    /// The set details embedded into the card.
    set: Set,
    /// The number of the card in the set.
    number: String,
    /// The artist of the card.
    artist: String,
    /// The rarity of the card, such as "Common" or "Rare Rainbow".
    rarity: String,
    /// The national pokedex numbers associated with any Pokémon featured on a given card.
    national_pokedex_numbers: Vec<i16>,
    /// The images for a card.
    images: CardImages,
    /// The TCGPlayer information for a given card. ALL PRICES ARE IN US DOLLARS.
    tcg_player: TCGPlayer,
}

/// A set of cards.
struct Set {
    /// A unique identifier for the set.
    id: String,
    /// The name of the set.
    name: String,
    /// The number printed on the card that represents the total. This total does not include secret rares.
    printed_total: i16,
    /// The total number of cards in the set, including secret rares, alternate art, etc.
    total: i16,
    /// The date the set was released (in the USA). Format is YYYY/MM/DD.
    release_date: String,
    /// Any images associated with the set, such as symbol and logo.
    images: SetImages,
}

struct SetImages {
    /// The url to the symbol image.
    symbol: String,
    /// The url to the logo image.
    logo: String,
}

struct CardImages {
    /// The url to the small card image.
    small: String,
    /// The url to the large card image.
    large: String,
}

struct TCGPlayer {
    /// The url to purchase this card.
    url: String,
    /// A date that the price was last updated YYYY/MM/DD.
    updated_at: String,
    /// The prices of this card as of the `updated_at` date.
    prices: CardPrices,
}

struct CardPrices {
    low: f32,
    mid: f32,
    high: f32,
    market: f32,
    direct_low: f32,
}
