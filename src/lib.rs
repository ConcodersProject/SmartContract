//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub price: u64,
    pub image_url: String,
    pub creator: AccountAddress,
    pub owners: Vec<AccountAddress>,
}

/// Your smart contract state.
#[derive(Serialize, SchemaType, Clone)]
pub struct State {
    // Your state
    counter: i8,
    items: Vec<Item>,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    /// Your error
    OwnerError,
    IncrementError,
    DecrementError,
}

/// Init function that creates a new smart contract.
#[init(contract = "counter")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    // Your code

    Ok(State {
        counter: 0,
        items: Vec::new(),
    })
}

// Our functions and stuff

type IncrementVal = i8;
/// Receive function. The input parameter is the boolean variable `throw_error`.
///  If `throw_error == true`, the receive function will throw a custom error.
///  If `throw_error == false`, the receive function executes successfully.
#[receive(
    contract = "counter",
    name = "increment",
    parameter = "i8",
    error = "Error",
    mutable
)]
fn increment<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    // Your code

    let param: IncrementVal = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::OwnerError
    );

    ensure!(param > 0, Error::IncrementError);
    state.counter += param;
    Ok(())
}

#[receive(
    contract = "counter",
    name = "decrement",
    parameter = "i8",
    error = "Error",
    mutable
)]
fn decrement<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    // Your code

    let param: IncrementVal = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::OwnerError
    );

    ensure!(param < 0, Error::DecrementError);
    state.counter -= param;
    Ok(())
}

/// View function that returns the content of the state.
#[receive(contract = "counter", name = "view", return_value = "i8")]
fn view<'a, 'b, S: HasStateApi>(
    _ctx: &'a impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<i8> {
    Ok(host.state().counter)
}
