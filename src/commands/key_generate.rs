use crate::cli::Generate;

use bdk::bitcoin::hashes::{sha256, Hash};
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::Network;
use bdk::keys::bip39::{Language, Mnemonic, WordCount};
use bdk::keys::{DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey};
use bdk::miniscript::miniscript;
use bdk::Error;

use rand::thread_rng;
use rand::Rng;

use serde_json::json;

impl Generate {
    pub fn generate(&self, network: Network) -> Result<serde_json::Value, Error> {
        let secp = Secp256k1::new();

        let mnemonic_type = match self.entropy {
            12 => WordCount::Words12,
            _ => WordCount::Words24,
        };

        // radom entropy for generate the final mnemonic phrase
        let entropy_str = (0..self.dicerolls).fold("".to_owned(), |acc, _| {
            let num: u16 = thread_rng().gen_range(0..65535);
            acc + &num.to_string()
        });

        let entropy_hash = sha256::Hash::hash(entropy_str.as_bytes());

        let mnemonic: GeneratedKey<_, miniscript::BareCtx> = Mnemonic::generate_with_entropy(
            (mnemonic_type, Language::English),
            *entropy_hash.as_inner(),
        )
        .map_err(|_| Error::Generic("Mnemonic generation error".to_string()))?;

        let mnemonic = mnemonic.into_key();
        let xkey: ExtendedKey = (mnemonic.clone(), self.password.clone()).into_extended_key()?;

        let xprv = xkey.into_xprv(network).ok_or_else(|| {
            Error::Generic("Privatekey info not found (should not happen)".to_string())
        })?;

        let fingerprint = xprv.fingerprint(&secp);
        let phrase = mnemonic
            .word_iter()
            .fold("".to_string(), |phrase, w| phrase + w + " ")
            .trim()
            .to_string();

        Ok(
            json!({ "fingerprint": fingerprint.to_string(), "mnemonic": phrase, "xprv": xprv.to_string(), /*"xpub": xpub.to_string()*/ }),
        )
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_generate() {

//         assert_eq!(
//             generate(),
//             Ok(json!({ "fingerprint": fingerprint.to_string(), "mnemonic": phrase, "xprv": xprv.to_string() }))
//         );
//     }
// }
