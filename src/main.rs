use std::str::FromStr;

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::Http::new("http://127.0.0.1:8545")?;
    let web3 = web3::Web3::new(transport);
    let contract_addr =
        web3::types::Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3")
            .expect("error on parse contract addr");
    let contract = web3::contract::Contract::from_json(
        web3.eth(),
        contract_addr,
        include_bytes!("greeter.json"),
    );

    println!("Calling accounts...");
    let accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);

    let my_addr = web3::types::Address::from_str("0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266")
        .expect("error on parse my addr");

    match contract {
        Ok(con) => {
            println!("Contract address: {}", con.address());
            println!("Calling greet func...");

            let res = con.query("greet", (), None, web3::contract::Options::default(), None);
            let msg: String = res.await.expect("err on get greet");
            println!("Greet message: {}", msg);

            let res_set = con.call(
                "setGreeting",
                ("Ola from kauly".to_string(),),
                my_addr,
                web3::contract::Options::default(),
            );

            let set_addr = res_set.await.expect("error on set greet");
            println!("Set transaction id: {}", set_addr);

            let res = con.query("greet", (), None, web3::contract::Options::default(), None);
            let msg: String = res.await.expect("err on get greet");
            println!("Greet message: {}", msg);
        }
        Err(e) => println!("Contract error: {}", e),
    }

    Ok(())
}
