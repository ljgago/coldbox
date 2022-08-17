use crate::cli::Restore;

use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::Network;
use bdk::keys::bip39::{Language, Mnemonic};
use bdk::keys::{DerivableKey, ExtendedKey};
use bdk::Error;

use serde_json::json;

impl Restore {
    pub fn restore(&self, network: Network) -> Result<serde_json::Value, Error> {
        let secp = Secp256k1::new();

        let mnemonic = Mnemonic::parse_in(Language::English, self.mnemonic.clone());
        let xkey: ExtendedKey = (mnemonic.unwrap(), self.password.clone()).into_extended_key()?;

        let xprv = xkey.into_xprv(network).ok_or_else(|| {
            Error::Generic("Privatekey info not found (should not happen)".to_string())
        })?;
        let fingerprint = xprv.fingerprint(&secp);

        Ok(json!({ "fingerprint": fingerprint.to_string(), "xprv": xprv.to_string() }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_restore() {
        // bitcoin network - without password
        let key_restore_cmd = Restore{
            mnemonic: String::from("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"),
            password: Some(String::from("")),
        };
        let result = key_restore_cmd.restore(Network::Bitcoin).unwrap();
        let fingerprint = result.get("fingerprint").unwrap().as_str().unwrap();
        let xprv = result.get("xprv").unwrap().as_str().unwrap();

        assert_eq!(fingerprint, "73c5da0a");
        assert_eq!(xprv, "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu");

        // testnet network - with password
        let key_restore_cmd = Restore{
            mnemonic: String::from("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"),
            password: Some(String::from("demo")),
        };
        let result = key_restore_cmd.restore(Network::Testnet).unwrap();
        let fingerprint = result.get("fingerprint").unwrap().as_str().unwrap();
        let xprv = result.get("xprv").unwrap().as_str().unwrap();

        assert_eq!(fingerprint, "03393bdd");
        assert_eq!(xprv, "tprv8ZgxMBicQKsPeE6XnhCjJ5WHgod5tWXv3W3jujmxAGsgjTr8ewZan8YvomaGTDmQyYkUJuGx4XUq5czx7nytjAGSHEv1XgYqj41X3NCT3xU");
    }
}
