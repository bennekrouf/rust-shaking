extern crate d1_filework;
use d1_filework::{
    get_transactions,
    get_first_transaction_for
};
use failure::Error;
// &'static str is a string that the guaranty of living during the life of the program

fn main() -> Result<(), Error> {
    // let trans = get_transactions_e("test_data/transactions.json").expect("Could not load tx");
    let trans = get_transactions("test_data/transactions.json")?;
    for i in trans {
        println!("{:?}", i)
    }

    let t = get_first_transaction_for("test_data/transactions.json", "TOTO");
    match t {
        Ok(v) => println!("Found transaction: {:?}", v),
        Err(e) => println!("Error {}, Backtrace = : {}", e, e.backtrace())
    }
    Ok(())
}

// Use Option for errors like null pointer
// An Option is like an Enum
// <T> generic type

// pub enum Option<T> {
//     Some(T),
//     None,
// }



// fn main() -> Result<(), TransactionError> {
//     println!("Hello, world!");
//     // let trans = get_transactions_e("test_data/transactions.json").expect("Could not load tx");
//     let trans = get_transactions("test_data/transactions.json")?;
//     for i in trans {
//         println!("{:?}", i)
//     }

//     let t = get_first_transaction_for("test_data/transactions.json", "toto")
//     .ok_or("could not get first transaction")?;
//     println!("My first transaction : {:?}", t);
//     Ok(())
// }