//! Program instruction processor for

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult, 
    pubkey::Pubkey,
};

// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey, 
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult{
    // actual implementation
    
    // get the accont where data is stored
    let account_iter = &mut accounts.iter()?;
    let data_account = next_account_info(account_iter)?;

    // account must be the owner of the program to modify the data of data_account
    if account.owner != _program_id{
        msg!("The program is not owned by owner.");
        return Err(ProgramError::InvalidProgramId);
    }

    // get the length of data accounts
    let data_account_length = data_account.data_len();

    // unpack the instruction data
    let (tag, rest) = instruction_data.split_first().ok_or(InvalidInstruction)?;
    // tag decides the action to be taken
    Ok(match tag {
        0 =>{
            // use 0 for account creation - fetch data related to account creation
            // first 64 bits (8 bytes) are used for uid
            let (uid, rest) = rest.split_at(8)?;
            // next 256 bits represents a public key of the user
            let (public_key, rest) = Self::unpack_pubkey(rest)?;
            // call the function to perform write in data account
            Self::write_to_data_account(uid, public_key);
        },
        1 => {
            // some other task can be added from here
        },
        _ => return Err(InvalidInstruction.into()),
    }) 
}

// function that unpacks public key from the input
pub fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
    if input.len() >= 32 {
        let (key, rest) = input.split_at(32);
        let pk = Pubkey::new(key);
        Ok((pk, rest))
    } else {
        Err(TokenError::InvalidInstruction.into())
    }
}

pub fn write_to_data_account(){
    // TO DO
}