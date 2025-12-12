struct Binder {
    id: String,
    owner: Owner,
    name: String,
    desired_pokemons: Vec<Pokemon>,
    acquired_pokemons: Vec<AcquiredPokemon>,
    pokemon_restrictions: Vec<PokemonRestriction>,
    desired_cards: Vec<Card>,
    acquired_cards: Vec<AcquiredCard>,
}

struct Owner {
    /// UUID of the binder owner
    id: String,
    /// Display name of the binder owner
    name: String,
    /// A list of binders that are designated as `Trade Binders`.
    trade_binders: Vec<Binder>,
    /// A list of all binders that are owned by this owner.
    binders: Vec<Binder>,
}

struct AcquiredPokemon {
    /// Generic Pokemon selection
    pokemon: Pokemon,
    /// Specific cards that are in the binder
    cards_acquired: Vec<AcquiredCard>,
}

struct Pokemon {
    /// National Pokedex Number of the Pokemon
    id: i16,
    /// Name of the Pokemon
    name: String,
}

struct PokemonRestriction {
    /// Generic Pokemon selection
    pokemon: Pokemon,
    /// Cards that the binder owner does not want to count toward a desired Pokemon
    restricted_cards: Vec<Card>,
}

struct AcquiredCard {
    /// A specific Pokemon Card
    card: Card,
    /// Quantity of this card in the binder
    qty_acquired: i32,
}

/// A specific Pokemon Card
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

struct Set {
    id: String,
    name: String,
    printed_total: i16, // The number printed on the card that represents the total. This total does not include secret rares.
    total: i16, // The total number of cards in the set, including secret rares, alternate art, etc.
    release_date: String, // YYYY/MM/DD
    images: SetImages,
}

struct SetImages {
    symbol: String, // URL
    logo: String,   // URL
}

struct CardImages {
    small: String, // URL
    large: String, // URL
}

struct TCGPlayer {
    url: String,        // URL to purchase this card
    updated_at: String, // A date that the price was last updated YYYY/MM/DD
    prices: CardPrices,
}

struct CardPrices {
    low: f32,
    mid: f32,
    high: f32,
    market: f32,
    direct_low: f32,
}
