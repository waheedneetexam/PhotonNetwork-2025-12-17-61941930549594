use ic_cdk::api::caller;
use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
};
use bitcoin::{Address, Network, PublicKey};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};
use std::cell::RefCell;



// --- HELPER FUNCTION (Only define this once!) ---
fn get_key_id() -> EcdsaKeyId {
    EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        // using "test_key_1" as requested by your previous error log
        name: "test_key_1".to_string(), 
    }
}


#[ic_cdk::update]
async fn get_btc_address() -> String {
    let user_principal = caller();

    // 1. Get ECDSA Public Key from the Management Canister
    let (response,) = ecdsa_public_key(EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path: vec![user_principal.as_slice().to_vec()],
        key_id: get_key_id(),
    })
    .await
    .expect("Failed to fetch public key");

    // 2. Parse the Public Key
    let public_key = PublicKey::from_slice(&response.public_key)
        .expect("Invalid public key from ICP");

    // 3. Generate the Address 
    // We add .expect() here because bitcoin v0.30 returns a Result
    let address = Address::p2wpkh(&public_key, Network::Testnet)
        .expect("Failed to create address");

    // 4. Return as String
    address.to_string()
}

ic_cdk::export_candid!();