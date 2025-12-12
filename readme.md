# Table of contents
- [Client Features](#features-on-the-client)
    - [Owner View](#owner-view)
        - [Binder](#binder)
        - [Collecting](#collecting)
        - [Trading](#trading)
        - [Card Search](#card-search)
    - [Viewer View](#viewer-view)
        - [Binder](#binder-1)
        - [Trading](#trading-1)

# Features on the client
## Owner View
### Binder
- [ ] Register a new binder
- [ ] Transfer ownership
- [ ] Mark a binder as a `Trade Binder`
- [ ] Move everything from one binder to another
- [ ] Request a price update for all cards

### Collecting
- [ ] Search for Pokemon to add to the wishlist
    - [ ] If there are Pokemon requested, mark the Pokemon as collected by adding a new card
- [ ] Search through for Pokemon to mark as collected
- [ ] Search for cards
    - [ ] The user should be able to mark it as collected
    - [ ] The user should be able to mark it as desired
        - [ ] If there are specific cards desired, the user should be able to click on the card and be presented with a popup to confirm if they want to mark it as collected

### Trading

### Card Search
- [ ] Search by card name
- [ ] Search by set
    - [ ] Search by card number in set
- [ ] Search by Pokedex Number
- [ ] Search by supertype (Pokemon, Trainer, etc)
- [ ] Filter by type
- [ ] Filter by set
- [ ] Filter by subtype (V, EX, Mega, etc)

## Viewer view
### Binder
- [ ] View the `requested Pokemon`
- [ ] View the `requested cards`
    - [ ] Be able to toggle viewing cards that are already collected
- [ ] View the `collected cards`
    - This might need to look different if it's a trade binder

### Trading
- [ ] When viewing a requested Pokemon/Card, the user should be able to select one or more cards to `offer`. A modal will appear with options to request `cash` (can be disabled by the binder owner) or a `trade`.
    - [ ] If the user requests a trade, a list will appear with all cards in the trade binder as well as the market value of the card(s) being offered and the cards in the trade list.
        - [ ] The user can select one or more cards to request for the trade.
        - [ ] The total trade value on both sides of the trade should be displayed to the user.
- [ ] Show `Current Offers`
    - [ ] Pressing this button will present the user with a list of current offers that they have requested.
        - [ ] Clicking into a request will allow the user to update the `offer`.
            - [ ] The user can change the `offer type` between `cash` and `trade`.
            - [ ] The user can update the cards offered and requested.
- [ ] The user should get a notification when a card in their offer has had a change. (a card they're offering has been acquired / a card they're requesting has been given away)
    - [ ] This notification should take the user to their offer and clearly show the change that happened. The user can then make updates as they see fit.
