mod error;
pub use error::TransactionError;
use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    // Ok(
    //     match serde_json::from_str(&match std::fs::read_to_string(fname) {
    //         Ok(v) => v,
    //         Err(e) => return Err(e.into())
    //     }) {
    //         Ok(v) => v,
    //         Err(e) => return Err(e.into())
    //     }
    // )
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}

pub fn get_first_transaction_for(fname: &str, uname: &str) -> Result<Transaction, failure::Error> {
    let trans = get_transactions(fname)?;
    for t in trans {
        if t.from == uname {
            return Ok(t);
        }
    }
    Err(TransactionError::Mess("Could not find transaction with that name").into())
}


// pub fn get_first_transaction_for(fname: &str, uname: &str) -> Option<Transaction> {
//     let trans = get_transactions(fname).ok()?;
//     for t in trans {
//         if t.from == uname {
//             return Some(t);
//         }
//         return None;
//     }
//     return None
// }


// pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
//     // Err("No Tx".to_string());
//     let s = match std::fs::read_to_string(fname) {
//         Ok(v) => v,
//         Err(e) => return Err(e.to_string())
//     };

//     let t:Vec<Transaction> = match serde_json::from_str(&s) {
//         Ok(v) => v,
//         Err(e) => return Err(e.to_string())
//     };

//     Ok(t)
//     // Ok(Vec::new())
// }

// pub fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, String>{
//     std::fs::read_to_string(fname)
//     .map_err(|e| e.to_string())
//     .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
// }

// pub fn get_transactions_c(fname: &str) -> Result<Vec<Transaction>, TransactionError>{
//     std::fs::read_to_string(fname)
//     .map_err(|e| e.into())
//     .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.into()))
// }

// pub fn get_transactions_d(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
//     Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
// }