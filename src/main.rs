extern crate bip39;
extern crate bitcoin;
use bip39::{Mnemonic, Seed};
use bitcoin::{
    bip32::{self, DerivationPath, Xpub},
    Network,
};

fn main() {
    let phrase = "vocal tennis filter lend margin town kangaroo spice coffee scatter fault any";
    let mnemonic = Mnemonic::from_phrase(phrase, bip39::Language::English).unwrap();
    println!("Mnemonic: {}", mnemonic.phrase());

    let seed = Seed::new(&mnemonic, "");
    println!("Seed: {:?}", seed.as_bytes());

    let network = Network::Bitcoin;
    let master_key = bip32::Xpriv::new_master(network, seed.as_bytes()).unwrap();
    println!("Master key: {}", master_key);

    let path = "m/44'/61'/0'/0/0";
    let ctx = bitcoin::key::Secp256k1::new();

    let eth_key = master_key
        .derive_priv(&ctx, &path.parse::<DerivationPath>().expect("Valid path"))
        .expect("Valid derive");
    println!("ETH key: {}", eth_key);

    
    let xpub = Xpub::from_priv(&ctx, &eth_key);
    println!("Derived key: {}", xpub);


    let signer =
        alloy::signers::local::LocalSigner::from_slice(&eth_key.to_priv().to_bytes()).unwrap();
    println!("address of signer: {}", signer.address());
}
