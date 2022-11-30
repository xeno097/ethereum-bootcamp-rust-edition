use k256::ecdsa::{recoverable, signature::Signer, Signature, SigningKey, VerifyingKey};
use sha3::Digest;
use std::str::FromStr;

const PRIVATE_KEY: &str = "6b911fd37cdf5c81d4c0adb1ab7fa822ed253ab0ad9aa18d77257c88b29b718e";

// 1: Hash Message
pub fn hash_message(message: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = sha3::Keccak256::new();

    hasher.update(message);

    hasher.finalize().into_iter().collect()
}

// 2: Sign Message
pub fn sign_message(message: &[u8]) -> recoverable::Signature {
    // Signing
    let signing_key = SigningKey::from_bytes(&hex::decode(PRIVATE_KEY).unwrap()).unwrap(); // Serialize with `::to_bytes()`

    let hash = hash_message(message);

    signing_key.sign(&hash)
}

// 3: Recover Key
pub fn recover_key(message: &[u8], hex_signature: &str, recovery_id: u8) -> VerifyingKey {
    let raw_signature = Signature::from_str(hex_signature).unwrap();
    let parsed_recovery_id = recoverable::Id::new(recovery_id).unwrap();

    let signature = recoverable::Signature::new(&raw_signature, parsed_recovery_id).unwrap();

    signature
        .recover_verifying_key(&hash_message(message))
        .unwrap()
}

// 4: Key to Address
pub fn get_address(hex_public_key: &str) -> String {
    let byte_address = hex::decode(hex_public_key).unwrap();

    let (_, address) = byte_address.split_at(1);

    let hashed_pub_key = hash_message(address);

    let (_, partial_address) = hashed_pub_key.split_at(12);

    hex::encode(partial_address)
}

#[cfg(test)]
mod tests {

    mod hash_message {
        use crate::week_1::digital_signatures::hash_message;

        #[test]
        fn should_hash_the_given_message() {
            let hello_world_hash =
                "47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad";

            assert_eq!(
                hash_message("hello world"),
                hex::decode(hello_world_hash).unwrap()
            );
        }
    }

    mod sign_message {

        use k256::ecdsa::{recoverable, SigningKey};

        use crate::week_1::digital_signatures::{hash_message, sign_message, PRIVATE_KEY};

        #[test]
        fn should_sign_with_the_given_private_key() {
            let signing_key = SigningKey::from_bytes(&hex::decode(PRIVATE_KEY).unwrap()).unwrap();

            let pub_key = signing_key.verifying_key();

            let message =
                "ECDSA proves knowledge of a secret number in the context of a single message";
            let hash = hash_message(message);

            let signature: recoverable::Signature = sign_message(message.as_bytes());

            let recovered_pubkey = signature
                .recover_verifying_key(&hash)
                .expect("couldn't recover pubkey");

            assert_eq!(recovered_pubkey, pub_key);
        }
    }

    mod recover_key {
        use k256::ecdsa::SigningKey;

        use crate::week_1::digital_signatures::{recover_key, sign_message, PRIVATE_KEY};

        #[test]
        fn should_recover_the_key() {
            let message = "hello world";

            let signature = sign_message(message.as_bytes());

            let signing_key = SigningKey::from_bytes(&hex::decode(PRIVATE_KEY).unwrap()).unwrap();

            let pub_key = signing_key.verifying_key();

            let recovered_key = recover_key(
                message.as_bytes(),
                &format!("{}{}", signature.r(), signature.s()),
                signature.recovery_id().into(),
            );

            assert_eq!(recovered_key, pub_key);
        }
    }

    mod key_to_address {
        use k256::{ecdsa::SigningKey, elliptic_curve::sec1::ToEncodedPoint};

        use crate::week_1::digital_signatures::{get_address, PRIVATE_KEY};

        const EXPECTED_ADDRESS: &str = "16bB6031CBF3a12B899aB99D96B64b7bbD719705";

        #[test]
        fn should_return_the_expected_address() {
            let signing_key = SigningKey::from_bytes(&hex::decode(PRIVATE_KEY).unwrap()).unwrap();

            let pub_key = signing_key.verifying_key();

            let uncompressed_pub_key = pub_key.to_encoded_point(false);

            let address = get_address(&hex::encode(uncompressed_pub_key));

            assert_eq!(address, EXPECTED_ADDRESS.to_lowercase());
        }
    }
}
