use crate::lat::{Blockchain, Transaction};
use std::sync::{Arc, Mutex};
use warp::Filter;

#[tokio::main]
pub async fn start_gui() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new(4)));

    let blockchain_filter = warp::any().map(move || blockchain.clone());

    let create_transaction = warp::path!("transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .map(|transaction: Transaction, blockchain: Arc<Mutex<Blockchain>>| {
            let mut blockchain = blockchain.lock().unwrap();
            blockchain.add_block(vec![transaction]);
            warp::reply::json(&blockchain.chain)
        });

    let get_chain = warp::path!("chain")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .map(|blockchain: Arc<Mutex<Blockchain>>| {
            let blockchain = blockchain.lock().unwrap();
            warp::reply::json(&blockchain.chain)
        });

    let routes = create_transaction.or(get_chain);

    println!("Starting server on http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
