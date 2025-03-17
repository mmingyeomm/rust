//! Example showing how to send an [EIP-4844](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-4844.md) transaction using a keystore file.

use std::path::PathBuf;
use rand::thread_rng;

use alloy::{
    consensus::{SidecarBuilder, SimpleCoder},
    eips::eip4844::DATA_GAS_PER_BLOB,
    network::{EthereumWallet, TransactionBuilder, TransactionBuilder4844},
    primitives::{address, hex, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::LocalSigner,
};
use eyre::Result;
use tempfile::tempdir;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a temporary directory for our keystore
    let dir = tempdir()?;
    let mut rng = thread_rng();

    // Private key of Alice, the first default Anvil account
    let private_key = hex!("X");

    // Password to encrypt the keystore file with
    let password = "test";

    // Create a keystore file from the private key
    let (_, file_path) = LocalSigner::encrypt_keystore(&dir, &mut rng, private_key, password, None)?;
    let keystore_file_path = dir.path().join(file_path);

    println!("Created keystore at: {:?}", keystore_file_path);

    // Now read the keystore file back and create a signer
    let signer = LocalSigner::decrypt_keystore(keystore_file_path, password)?;
    println!("Using keystore for address: {}", signer.address());
    
    let wallet = EthereumWallet::from(signer);

    // Set up provider with Sepolia via Infura
    let rpc_url = "https://sepolia.infura.io/v3/035c5c117".parse()?;
    let provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);

    // Create a sidecar with some data
    let sidecar: SidecarBuilder<SimpleCoder> = SidecarBuilder::from_slice(b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam auctor, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl. Donec euismod, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eget nisl.");
    let sidecar = sidecar.build()?;

    // Target address
    let target_address = address!("08546b36ba6b9E5e09C7FB9e1b2a67a4dfb13652");

    // Build a transaction to send the sidecar to the target address
    let tx = TransactionRequest::default()
        .with_to(target_address)
        .with_blob_sidecar(sidecar);

    // Send the transaction and wait for the broadcast
    let pending_tx = provider.send_transaction(tx).await?;

    println!("Pending transaction... {}", pending_tx.tx_hash());

    // Wait for the transaction to be included and get the receipt
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    assert_eq!(receipt.to, Some(target_address));
    assert_eq!(
        receipt.blob_gas_used.expect("Expected to be EIP-4844 transaction"),
        DATA_GAS_PER_BLOB
    );

    // The temporary directory will be automatically cleaned up when it goes out of scope
    Ok(())
}
