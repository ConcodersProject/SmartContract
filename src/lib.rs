//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

#[derive(Serialize, SchemaType, Clone)]
pub struct ItemInput {
    pub name: String,
    pub price: Amount,
    pub total_supply: u64,
    pub image_url: String,
}

#[derive(Serialize, SchemaType, Clone)]
pub struct Item {
    pub name: String,
    pub price: Amount,
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
    /// Errors
    NameLengthError,
    TotalSupplyError,
    ImageUrlError,
    ItemNotFoundError,
    ItemNotOwnedError,
}

/// Init function that creates a new smart contract.
#[init(contract = "market")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
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
    return_value = "u64",
    mutable
)]
fn add_item<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<u64, Error> {
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
    ensure!(item.name.len() > 0, Error::NameLengthError);
    ensure!(item.total_supply > 0, Error::TotalSupplyError);
    ensure!(item.image_url.len() > 0, Error::ImageUrlError);

    // ensure!(
    //     ctx.sender().matches_account(&ctx.owner()),
    //     Error::OwnerError
    // );

    state.items.insert(state.item_count, item);
    state.item_count += 1;
    Ok(state.item_count - 1)
}

#[receive(
    contract = "market",
    name = "buy_item",
    parameter = "Vec<u64>",
    error = "Error",
    return_value = "Vec<(u64,bool)>",
    mutable,
    payable
)]
fn buy_item<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    amount: Amount,
) -> Result<Vec<(u64, bool)>, Error> {
    // Your code
    let mut return_value: Vec<(u64, bool)> = Vec::new();
    let input: Vec<u64> = ctx.parameter_cursor().get()?;
    let mut amt: Amount = amount;
    for item_id in input {
        let state = host.state_mut();
        let mut item = state
            .items
            .get_mut(&item_id)
            .ok_or(Error::ItemNotFoundError)?;
        if item.sold < item.total_supply {
            // Items available
            if item.price <= amount {
                // Sent enough balance
                item.owners.push(ctx.sender());
                item.sold += 1;
                return_value.push((item_id, true));
                amt -= item.price;
            } else {
                return_value.push((item_id, false));
            }
        } else {
            return_value.push((item_id, false));
        }
        item.sold += 1;
        item.owners.push(ctx.sender());
        return_value.push((item_id, true));
    }

    // ensure!(
    //     ctx.sender().matches_account(&ctx.owner()),
    //     Error::OwnerError
    // );

    Ok(return_value)
}

#[receive(
    contract = "market",
    name = "buy_single_item",
    parameter = "u64",
    error = "Error",
    return_value = "bool",
    mutable,
    payable
)]
fn buy_single_item<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    amount: Amount,
) -> Result<bool, Error> {
    // Your code
    let input: u64 = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let mut item = state
        .items
        .get_mut(&input)
        .ok_or(Error::ItemNotFoundError)?;
    if item.sold < item.total_supply {
        // Items available
        if item.price <= amount {
            // Sent enough balance
            item.owners.push(ctx.sender());
            item.sold += 1;
        } else {
        }
    } else {
    }
    item.sold += 1;
    item.owners.push(ctx.sender());

    // ensure!(
    //     ctx.sender().matches_account(&ctx.owner()),
    //     Error::OwnerError
    // );

    Ok(true)
}

#[receive(
    contract = "market",
    name = "transfer_item",
    parameter = "(u64, Address)",
    error = "Error",
    return_value = "bool",
    mutable
)]
fn transfer_item<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<bool, Error> {
    // Your code
    let input: (u64, Address) = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let mut item = state
        .items
        .get_mut(&input.0)
        .ok_or(Error::ItemNotFoundError)?;
    ensure!(
        item.owners.contains(&ctx.sender()),
        Error::ItemNotOwnedError
    );
    item.owners.push(input.1);
    let index = item.owners.iter().position(|x| *x == ctx.sender()).unwrap();
    item.owners.swap_remove(index);

    Ok(true)
}

// Read only views

// Get all listed items
#[receive(
    contract = "market",
    name = "get_all_items",
    // parameter = "u64",
    return_value = "Vec<(u64, Item)>"
)]
fn get_items<'a, 'b, S: HasStateApi>(
    _ctx: &'a impl HasReceiveContext,
    host: &'b impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Vec<(u64, Item)>> {
    // let ids: u64 = _ctx.parameter_cursor().get()?;
    let state = host.state();
    let mut return_items: Vec<(u64, Item)> = vec![];
    for (id, item) in state.items.iter() {
        return_items.push((*id, item.clone()));
    }
    Ok(return_items)
}

// Get a single item by id
#[receive(
    contract = "market",
    name = "get_single_item",
    parameter = "u64",
    return_value = "Item"
)]
fn get_single_item<'a, 'b, S: HasStateApi>(
    _ctx: &'a impl HasReceiveContext,
    host: &'b impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Item> {
    let id: u64 = _ctx.parameter_cursor().get()?;
    let state = host.state();
    let item = state.items.get(&id).ok_or(Error::ItemNotFoundError)?;
    Ok(item.clone())
}

// Get number of items created
#[receive(contract = "market", name = "get_item_count", return_value = "u64")]
fn get_item_count<'a, 'b, S: HasStateApi>(
    _ctx: &'a impl HasReceiveContext,
    host: &'b impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<u64> {
    let state = host.state();
    Ok(state.item_count)
}
