  
use crate::engine::constants::STRING_SIZE;
use crate::state::{MerchantAccount, OrderAccount, SubscriptionAccount};

// function that gives size of account 
pub fn get_account_size(min_len: usize, strings: &Vec<&String>) -> usize {
    let mut size = min_len;
    for item in strings {
        size = size + item.chars().count() + STRING_SIZE;
    }
    size
}