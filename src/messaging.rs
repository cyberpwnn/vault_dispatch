use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::BTreeMap;
use solana_program::{
    account_info::{AccountInfo},
    pubkey::Pubkey,
};

/// Represents all data messaging will use on an account. Contains connection data & an optional name
/// of this dispatch data
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MessagingData {
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

    /// The outbox. Contains all messages that have been sent to this user
    pub outbox: Vec<Message>,
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
pub fn gc_conversations(holder:&AccountInfo, older_than:u64)
{
    let mut holder_data = load_data(holder);
    holder_data.connections.values_mut()
        .for_each(|f| f.outbox.retain(|i | i.timestamp > older_than));
    save_data(holder, holder_data);
}

/// ID=4
/// Delete all messages in the given connection older than the specified time
pub fn gc_conversation(holder:&AccountInfo, contact:&AccountInfo, older_than:u64)
{
    let mut holder_data = load_data(holder);
    holder_data.connections.get_mut(contact.key).unwrap()
        .outbox.retain(|i| i.timestamp > older_than);
    save_data(holder, holder_data);
}

/// ID=3
/// Send a message from(account) to(account) at the given time, with the message itself
pub fn send_message(from:&AccountInfo, to:&AccountInfo, timestamp:u64, message:Vec<u8>)
{
    let mut from_data = load_data(from);
    from_data.connections.get_mut(to.key).unwrap().outbox.push(Message{
        timestamp,
        message,
    });
    save_data(from, from_data);
}

/// ID=2
/// Destroy a connection. Removes the requester side connection & message history
pub fn break_connection(requester:&AccountInfo, contact:&AccountInfo)
{
    let mut requester_data = load_data(requester);
    requester_data.connections.remove(contact.key);
    save_data(requester, requester_data);
}

/// ID=1
/// Request a new connection to a user. You must provide your write key so you can read their
/// messages if they accept
pub fn request_connection(requester:&AccountInfo, contact:&AccountInfo, cipher:Vec<u8>)
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
pub fn accept_connection(acceptor:&AccountInfo, requester:&AccountInfo, cipher:Vec<u8>)
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
fn save_data(account:&AccountInfo, data:MessagingData)
{
    data.serialize(&mut &mut account.data.borrow_mut()[..]).unwrap();
}

/// Load data utility
fn load_data(account:&AccountInfo) -> MessagingData {
    return MessagingData::try_from_slice(&account.data.borrow()).unwrap();
}