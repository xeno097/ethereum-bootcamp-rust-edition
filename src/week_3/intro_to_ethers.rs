use ethers::signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Wallet};
use k256::ecdsa::SigningKey;

fn create_wallets() -> (Wallet<SigningKey>, Wallet<SigningKey>) {
    let private_key_wallet = "f2f48ee19680706196e2e339e5da3491186e0c4c5030670656b0e0164837257d"
        .parse::<LocalWallet>()
        .expect("Could not create wallet with given private key");

    let mnemonic_wallet = MnemonicBuilder::<English>::default()
        .phrase("plate lawn minor crouch bubble evidence palace fringe bamboo laptop dutch ice")
        .build()
        .unwrap();

    (private_key_wallet, mnemonic_wallet)
}

#[cfg(test)]
mod tests {

    mod wallets {
        use ethers::signers::Signer;

        use crate::week_3::intro_to_ethers::create_wallets;

        #[test]
        fn should_successfully_build_a_wallet_from_a_private_key() {
            // Act
            let (wallet, _) = create_wallets();

            // Assert
            assert_eq!(
                hex::encode(wallet.address()),
                "5409ED021D9299bf6814279A6A1411A7e866A631".to_lowercase()
            );
        }

        #[test]
        fn should_successfully_build_a_wallet_from_a_mnemonic() {
            // Act
            let (_, wallet) = create_wallets();

            // Assert
            assert_eq!(
                hex::encode(wallet.address()),
                "88E9DD325BA8329dDD9825c1d24e8470b25575C1".to_lowercase()
            );
        }
    }
}
