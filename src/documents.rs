use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::BTreeMap;
use solana_program::{
    account_info::{AccountInfo},
};

/// Represents all data messaging will use on an account. Contains connection data & an optional name
/// of this dispatch data
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DocumentData {
    pub data: BTreeMap<Vec<u8>, Vec<u8>>,
}

/// ID=66
/// Put data into the document data
pub fn put_pair(account:&AccountInfo, keypair:(Vec<u8>, Vec<u8>)) { put(account, keypair.0, keypair.1); }
pub fn put(account:&AccountInfo, key:Vec<u8>, value:Vec<u8>)
{
    let mut documents = load_data(account);
    documents.data.insert(key, value);
    save_data(account, documents);
}

/// ID=65
/// Delete data from the document data
pub fn delete(account:&AccountInfo, key:Vec<u8>)
{
    let mut documents = load_data(account);
    documents.data.remove(&*key);
    save_data(account, documents);
}

/// ID=64
/// Clear the document data
pub fn clear(account:&AccountInfo)
{
    let mut documents = load_data(account);
    documents.data.clear();
    save_data(account, documents);
}

/// Save data utility
fn save_data(account:&AccountInfo, data:DocumentData)
{
    data.serialize(&mut &mut account.data.borrow_mut()[..]).unwrap();
}

/// Load data utility
fn load_data(account:&AccountInfo) -> DocumentData {
    return DocumentData::try_from_slice(&account.data.borrow()).unwrap();
}