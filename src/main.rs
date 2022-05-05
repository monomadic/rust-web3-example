use wasm_bindgen_futures::spawn_local;
use web3::{transports::eip_1193::{Eip1193, Provider}, futures::StreamExt};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    // connect wallet thread
    spawn_local(async {
        let provider = Provider::default().unwrap().unwrap();
        connect_wallet(provider).await.unwrap();
    });

    // on accounts changed
    spawn_local(async {
        let provider = Provider::default().unwrap().unwrap();
        let transport = Eip1193::new(provider);
        let mut stream = transport.accounts_changed_stream();

        while let Some(accounts) = stream.next().await {
            log::info!("accounts changed {:?}", &accounts);
        }
    });
    
    // on chain changed
    spawn_local(async {
        let provider = Provider::default().unwrap().unwrap();
        let transport = Eip1193::new(provider);
        let mut stream = transport.chain_changed_stream();

        while let Some(chainid) = stream.next().await {
            log::info!("chain changed {:?}", &chainid);
        }
    });
}

async fn connect_wallet(provider: Provider) -> web3::Result<()> {
    let transport = Eip1193::new(provider);
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().request_accounts().await.unwrap();

    for account in accounts {
        let balance = web3.eth().balance(account, None).await.unwrap();
        log::info!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}
