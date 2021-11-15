use solana_program::{
    account_info::{AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

mod bits;
mod documents;
mod messaging;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    match data[0] {
        0 => messaging::accept_connection(&accounts[0], &accounts[1], bits::read_bytes(1, data)),
        1 => messaging::request_connection(&accounts[0], &accounts[1], bits::read_bytes(1, data)),
        2 => messaging::break_connection(&accounts[0], &accounts[1]),
        3 => messaging::send_message(&accounts[0], &accounts[1], bits::read_u64(1, data), bits::read_bytes(9, data)),
        4 => messaging::gc_conversation(&accounts[0], &accounts[1], bits::read_u64(1, data)),
        5 => messaging::gc_conversations(&accounts[0], bits::read_u64(1, data)),
        64 => documents::clear(&accounts[0]),
        65 => documents::delete(&accounts[0], bits::read_bytes(1, data)),
        66 => documents::put_pair(&accounts[0], bits::read_bytes_pair(1, data)),
        _ => msg!("Unsupported Method!")
    }

    Ok(())
}
