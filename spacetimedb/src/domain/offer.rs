use spacetimedb::{table, SpacetimeType};
use crate::domain::{Card, User};

/// An offer from a viewer of a binder. This can be a `Cash` or `Trade` type offer.
#[table(name = offer)]
pub struct Offer {
    /// Unique ID of the offer
    #[primary_key]
    pub id: String,
    /// The identity of the person making the offer.
    owner: User,
    /// Either `Cash` or `Trade`.
    offer_type: OfferType,
    /// A list of cards that are being offered.
    offers: Vec<Card>,
    /// A list of cards that are being requested.
    requests: Vec<Card>,
}

#[derive(SpacetimeType)]
enum OfferType {
    Cash,
    Trade,
}
