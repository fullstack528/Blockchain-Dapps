// pub mod lock;
#![cfg_attr(not(feature = "std"), no_std)]



pub mod utils;
pub mod interact_token;
pub mod constants;
//pub mod lock;

mod vest;
pub use vest::VestContract;

mod address;
pub use address::Address;

#[derive(PartialEq, Debug)]
pub enum VestingError {
    NotEnoughBalance,
    AdminReleaseErrorNotPaused,
    AdminReleaseErrorNothingToWithdraw,
    AdminReleaseErrorNotEnoughTimeElapsed,
    AlreadyPaused,
    AlreadyUnpaused,
}

#[derive(PartialEq, Debug)]
// #[derive(Error, Debug)]
pub enum VestError {
    AlreadyPaused = 1,
    AlreadyUnpaused = 2,
    NotTheAdminAccount = 3,
    NotTheRecipientAccount = 4,
    UnexpectedVestingError = 5,
    NotEnoughBalance = 6,
    PurseTransferError = 7,
    NotPaused = 8,
    NothingToWithdraw = 9,
    NotEnoughTimeElapsed = 10,
    LocalPurseKeyMissing = 11,
    UnexpectedType = 12,
    MissingKey = 13,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    /// ERC20 contract called from within an invalid context.
    InvalidContext,
    /// Spender does not have enough balance.
    InsufficientBalance,
    /// Spender does not have enough allowance approved.
    InsufficientAllowance,
    /// Operation would cause an integer overflow.
    Overflow,
    /// User error.
    User(u16),
}

