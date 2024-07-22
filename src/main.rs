mod blockchain;
mod smart_contracts;

use blockchain::{Blockchain, Transaction, Block};
use rocket::{get, post, routes, serde::json::Json, fs::FileServer, uri, response::Redirect};
use rocket::serde::{Deserialize, Serialize};
use smart_contracts::{SampleContract, SmartContract, ContractParams};
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate rocket;

#[derive(Debug, Serialize, Deserialize)]
struct CreateTransactionRequest {
    sender: String,
    recipient: String,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct MineRequest {
    reward_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExecuteContractRequest {
    key: String,
    value: String,
}

type BlockchainState = Arc<Mutex<Blockchain>>;
type ContractState = Arc<Mutex<SampleContract>>;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Sigil Crypto!"
}

#[get("/home")]
fn home() -> Redirect {
    Redirect::to(uri!("/static/index.html"))
}

#[post("/transaction", format = "json", data = "<transaction>")]
fn create_transaction(transaction: Json<CreateTransactionRequest>, state: &rocket::State<BlockchainState>) -> &'static str {
    let mut blockchain = state.lock().unwrap();
    blockchain.create_transaction(Transaction {
        sender: transaction.sender.clone(),
        recipient: transaction.recipient.clone(),
        amount: transaction.amount,
    });
    "Transaction created"
}

#[post("/mine", format = "json", data = "<mine_request>")]
fn mine(mine_request: Json<MineRequest>, state: &rocket::State<BlockchainState>) -> &'static str {
    let mut blockchain = state.lock().unwrap();
    blockchain.mine_pending_transactions(mine_request.reward_address.clone());
    "Block mined"
}

#[get("/blocks")]
fn get_blocks(state: &rocket::State<BlockchainState>) -> Json<Vec<Block>> {
    let blockchain = state.lock().unwrap();
    Json(blockchain.blocks.clone())
}

#[post("/execute_contract", format = "json", data = "<contract_request>")]
fn execute_contract(contract_request: Json<ExecuteContractRequest>, state: &rocket::State<ContractState>) -> Result<&'static str, String> {
    let contract = state.lock().unwrap(); // Obtains a MutexGuard
    let params = ContractParams {
        key: contract_request.key.clone(),
        value: contract_request.value.clone(),
    };

    match contract.execute(&params) {
        Ok(_) => Ok("Contract executed successfully"),
        Err(e) => Err(format!("Contract execution failed: {}", e)),
    }
}


#[launch]
fn rocket() -> _ {
    env_logger::init();

    let blockchain = Blockchain::new();
    let contract = SampleContract::new();

    rocket::build()
        .manage(Arc::new(Mutex::new(blockchain)))
        .manage(Arc::new(Mutex::new(contract)))
        .mount("/", routes![
            index,
            home,
            create_transaction,
            mine,
            get_blocks,
            execute_contract  // Make sure this matches the function name
        ])
        .mount("/static", FileServer::from("static"))
}
