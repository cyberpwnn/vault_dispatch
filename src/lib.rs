use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::BTreeMap;
use solana_program::{
    account_info::{AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
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
    /// Pending is used for connection requests. After accepting it is set to false
    pub pending: bool,

    /// The write key to allow anyone to write as this contact
    pub cipher: Vec<u8>,

    /// The inbox. Contains all messages sent to this user via the contact
    pub inbox: Vec<Message>,
}

/// Represents a message. Hopefully encrypted.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Message {
    /// The time at which this message was sent
    pub timestamp: u64,

    /// The actual message in bytes
    pub message: Vec<u8>,
}

/// ID=5
/// Delete all messages in every connection older than the specified time
fn gc_conversations(holder:&AccountInfo, older_than:u64)
{
    let mut holder_data = load_data(holder);
    holder_data.connections.values_mut()
        .for_each(|f| f.inbox.retain(|i | i.timestamp > older_than));
    save_data(holder, holder_data);
}

/// ID=4
/// Delete all messages in the given connection older than the specified time
fn gc_conversation(holder:&AccountInfo, contact:&AccountInfo, older_than:u64)
{
    let mut holder_data = load_data(holder);
    holder_data.connections.get_mut(contact.key).unwrap()
        .inbox.retain(|i| i.timestamp > older_than);
    save_data(holder, holder_data);
}

/// ID=3
/// Send a message from(account) to(account) at the given time, with the message itself
fn send_message(from:&AccountInfo, to:&AccountInfo, timestamp:u64, message:Vec<u8>)
{
    let mut to_data = load_data(to);
    to_data.connections.get_mut(from.key).unwrap().inbox.push(Message{
        timestamp,
        message,
    });
    save_data(to, to_data);
}

/// ID=2
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

/// ID=1
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

/// ID=0
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
    data.serialize(&mut &mut account.data.borrow_mut()[..]).unwrap();
}

/// Load data utility
fn load_data(account:&AccountInfo) -> DispatchData {
    return DispatchData::try_from_slice(&account.data.borrow()).unwrap();
}

pub fn read_u32(startindex:u32, data:&[u8]) -> u32
{
    let start = startindex as usize;
    return ((data[start] as u32) << 24) + ((data[start+1] as u32) << 16)
        + ((data[start+2] as u32) << 8) + ((data[start+3] as u32) << 0);
}

pub fn read_u64(startindex:u32, data:&[u8]) -> u64
{
    let start = startindex as usize;
    return ((data[start] as u64) << 56) + ((data[start] as u64) << 48)
        + ((data[start] as u64) << 40) + ((data[start] as u64) << 32)
        + ((data[start] as u64) << 24) + ((data[start] as u64) << 16)
        + ((data[start] as u64) << 8) + ((data[start] as u64) << 0);
}

/// Read from a large byte array a sized vector of bytes
/// Reads the first 4 bytes (u32 length of data)
pub fn read_bytes(startindex:u32, data:&[u8]) -> Vec<u8>
{
    let start = startindex as usize;
    let len = read_u32(startindex, data) as usize;
    let mut v:Vec<u8> = vec!();
    for i in 0..len {
        v.push(data[i+start+4])
    }

    return v;
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    match data[0] {
        0 => accept_connection(&accounts[0], &accounts[1], read_bytes(1, data)),
        1 => request_connection(&accounts[0], &accounts[1], read_bytes(1, data)),
        2 => break_connection(&accounts[0], &accounts[1]),
        3 => send_message(&accounts[0], &accounts[1], read_u64(1, data), read_bytes(9, data)),
        4 => gc_conversation(&accounts[0], &accounts[1], read_u64(1, data)),
        5 => gc_conversations(&accounts[0],read_u64(1, data)),
        _ => msg!("Unsupported Method!")
    }

    Ok(())
}
