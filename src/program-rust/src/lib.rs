use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::BTreeMap;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Represents all data dispatch will use on an account. Contains connection data & an optional name
/// of this dispatch data
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DispatchData {
    pub connections: BTreeMap<Pubkey, Connection>,
    pub name: String,
}

/// Represents a connection to another dispatch account. This is required for Encryption
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Connection {
    //! Pending is used for connection requests. After accepting it is set to false
    pub pending: bool,

    //! The write key to allow anyone to write as this contact
    pub cipher: Vec<u8>,

    //! The inbox. Contains all messages sent to this user via the contact
    pub inbox: Vec<Message>,
}

/// Represents a message. Hopefully encrypted.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Message {
    //! The time at which this message was sent
    pub timestamp: u64,

    //! The actual message in bytes
    pub message: Vec<u8>,
}

/// Delete all messages in every connection older than the specified time
fn gc_conversations(holder:&AccountInfo, older_than:u64)
{
    let mut holder_data = load_data(holder);
    for connection in holder_data.connections.keys() {
        connection.inbox.retain(|&i| i.timestamp > older_than);
    }
    save_data(holder, holder_data);
}

/// Delete all messages in the given connection older than the specified time
fn gc_conversation(holder:&AccountInfo, contact:&AccountInfo, older_than:u64)
{
    let mut holder_data = load_data(holder);
    let mut connection = holder_data.connections.get_mut(contact.key).unwrap();
    connection.inbox.retain(|&i| i.timestamp > older_than);
    save_data(holder, holder_data);
}

/// Send a message from(account) to(account) at the given time, with the message itself
fn send_message(from:&AccountInfo, to:&AccountInfo, timestamp:u64, message:Vec<u8>)
{
    let mut to_data = load_data(to);
    let mut connection = to_data.connections.get_mut(from.key).unwrap();
    connection.inbox.push(Message{
        timestamp,
        message,
    });
    save_data(to, to_data);
}

/// Destroy a connection. Removes the source and destination connections & message history
fn break_connection(requester:&AccountInfo, contact:&AccountInfo)
{
    let mut contact_data = load_data(contact);
    let mut requester_data = load_data(requester);
    contact_data.connections.remove(requester.key);
    requester_data.connections.remove(contact.key);
    save_data(contact, contact_data);
    save_data(requester, requester_data);
}

/// Request a new connection to a user. You must provide your write key so you can read their
/// messages if they accept
fn request_connection(requester:&AccountInfo, contact:&AccountInfo, cipher:Vec<u8>)
{
    let mut contact_data = load_data(contact);
    contact_data.connections.insert(*requester.key, Connection {
        cipher,
        pending: true,
        inbox: vec!()
    });
    save_data(contact, contact_data);
}

/// Accept a connection request by setting your connection to non-pending & providing your own
/// connection data to their account with your write key
fn accept_connection(acceptor:&AccountInfo, requester:&AccountInfo, cipher:Vec<u8>)
{
    let mut acceptor_data = load_data(acceptor);
    let mut requester_data = load_data(requester);
    requester_data.connections.insert(*acceptor.key, Connection {
        cipher,
        pending: false,
        inbox: vec!()
    });
    let mut connection = acceptor_data.connections.get_mut(requester.key).unwrap();
    connection.pending = false;
    save_data(requester, requester_data);
    save_data(acceptor, acceptor_data);
}

/// Save data utility
fn save_data(account:&AccountInfo, data:DispatchData)
{
    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
}

/// Load data utility
fn load_data(account:&AccountInfo) -> DispatchData {
    return DispatchData::try_from_slice(&account.data.borrow())?;
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");
    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;
}
