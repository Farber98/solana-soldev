use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::program_pack::{IsInitialized, Sealed};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StudentAccountState {
    pub is_initialized: bool,
    pub name: String,
    pub message: String,
}

impl Sealed for StudentAccountState {}

impl IsInitialized for StudentAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
