use spacetimedb::{table, SpacetimeType};

/// This is to denote when a binder owner has acquired a specific card.
/// More than one of the same card can be collected in the binder.
#[table(name = acquired_card)]
pub struct AcquiredCard {
    /// A specific Pokémon Card.
    card: Card,
    /// Quantity of this card in the binder.
    qty_acquired: i32,
}

/// Information about a specific Pokémon Card.
#[table(name = card)]
pub struct Card {
    /// Unique identifier for the card.
    #[primary_key]
    pub id: String,
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
#[table(name = set)]
pub struct Set {
    /// A unique identifier for the set.
    #[primary_key]
    pub id: String,
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

/// Urls to the images related to a set.
#[derive(SpacetimeType)]
struct SetImages {
    /// The url to the symbol image.
    symbol: String,
    /// The url to the logo image.
    logo: String,
}

/// Urls to the images related to a card.
#[derive(SpacetimeType)]
struct CardImages {
    /// The url to the small card image.
    small: String,
    /// The url to the large card image.
    large: String,
}

/// TCGPlayer.com information about the value of a card.
#[derive(SpacetimeType)]
struct TCGPlayer {
    /// The url to purchase this card.
    url: String,
    /// A date that the price was last updated YYYY/MM/DD.
    updated_at: String,
    /// The prices of this card as of the `updated_at` date.
    prices: CardPrices,
}

/// Prices of the card from TCGPlayer.com
#[derive(SpacetimeType)]
struct CardPrices {
    /// The low price of the card.
    low: f32,
    /// The mid price of the card.
    mid: f32,
    /// The high price of the card.
    high: f32,
    /// The market value of the card. This is usually the best representation of what people are willing to pay.
    market: f32,
    /// The direct low price of the card.
    direct_low: f32,
}
