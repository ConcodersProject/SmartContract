//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

#[derive(Serialize, SchemaType, Clone)]
pub struct ItemInput {
    pub name: String,
    pub price: u64,
    pub total_supply: u64,
    pub image_url: String,
}

#[derive(Serialize, SchemaType, Clone)]
pub struct Item {
    pub name: String,
    pub price: u64,
    pub total_supply: u64,
    pub image_url: String,

    pub sold: u64,
    pub creator: Address,
    pub owners: Vec<Address>,
}

/// Your smart contract state.
#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct State<S: HasStateApi> {
    // Your state
    items: StateMap<u64, Item, S>,
    item_count: u64,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
}

/// Init function that creates a new smart contract.
#[init(contract = "market")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    // Your code

    Ok(State {
        items: _state_builder.new_map(),
        item_count: 0,
    })
}

// Our functions and stuff

type NewItemData = ItemInput;
#[receive(
    contract = "market",
    name = "add_item",
    parameter = "ItemInput",
    error = "Error",
    mutable
)]
fn add_item<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), Error> {
    // Your code

    let input: NewItemData = ctx.parameter_cursor().get()?;
    let item = Item {
        name: input.name.trim().to_string(),
        price: input.price,
        total_supply: input.total_supply,
        image_url: input.image_url.trim().to_string(),
        sold: 0,
        creator: ctx.sender(),
        owners: Vec::new(),
    };

    let state = host.state_mut();
    // ensure!(
    //     ctx.sender().matches_account(&ctx.owner()),
    //     Error::OwnerError
    // );

    state.items.insert(0, item);
    Ok(())
}

type ViewItems = Vec<u64>;
/// View function that returns the items vector of the item indexes we want
#[receive(
    contract = "market",
    name = "view",
    parameter = "Vec<u64>",
    return_value = "Vec<Option<Item>>"
)]
fn view<'a, 'b, S: HasStateApi>(
    _ctx: &'a impl HasReceiveContext,
    host: &'b impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Vec<Option<Item>>> {
    let ids: ViewItems = _ctx.parameter_cursor().get()?;
    let state = host.state();
    let mut return_items: Vec<Option<Item>> = vec![];
    for id in ids {
        let item = state.items.get(&id);
        return_items.push(item.as_deref().cloned());
    }
    Ok(return_items)
}
