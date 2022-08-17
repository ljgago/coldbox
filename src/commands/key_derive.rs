use crate::cli::Derive;

use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::{DerivationPath, KeySource};
use bdk::bitcoin::Network;
use bdk::descriptor::Segwitv0;
use bdk::keys::KeyError::{InvalidNetwork, Message};
use bdk::keys::{DerivableKey, DescriptorKey};
use bdk::Error;

use serde_json::json;

impl Derive {
    pub fn derive(&self, network: Network) -> Result<serde_json::Value, Error> {
        if self.xprv.network != network {
            return Err(Error::Key(InvalidNetwork));
        }

        let secp = Secp256k1::new();
        let derived_xprv = &self.xprv.derive_priv(&secp, &self.path)?;
        let origin: KeySource = (self.xprv.fingerprint(&secp), self.path.clone());

        let derived_xprv_desc_key: DescriptorKey<Segwitv0> =
            derived_xprv.into_descriptor_key(Some(origin), DerivationPath::default())?;

        if let DescriptorKey::Secret(desc_seckey, _, _) = derived_xprv_desc_key {
            let desc_pubkey = desc_seckey
                .as_public(&secp)
                .map_err(|e| Error::Generic(e.to_string()))?;

            Ok(json!({ "xprv": desc_seckey.to_string(), "xpub": desc_pubkey.to_string() }))
        } else {
            Err(Error::Key(Message("Invalid key variant".to_string())))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    use bdk::bitcoin::util::bip32::ExtendedPrivKey;

    // https://github.com/satoshilabs/slips/blob/master/slip-0132.md

    #[test]
    fn test_key_derive() {
        // m/44'/0'/0'
        let key_derive_cmd = Derive{
            path: DerivationPath::from_str("m/44'/0'/0'").unwrap(),
            xprv: ExtendedPrivKey::from_str("xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu").unwrap()
        };
        let result = key_derive_cmd.derive(Network::Bitcoin).unwrap();
        let xprv = result.get("xprv").unwrap().as_str().unwrap();
        let xpub = result.get("xpub").unwrap().as_str().unwrap();

        assert_eq!(xprv, "[73c5da0a/44'/0'/0']xprv9xpXFhFpqdQK3TmytPBqXtGSwS3DLjojFhTGht8gwAAii8py5X6pxeBnQ6ehJiyJ6nDjWGJfZ95WxByFXVkDxHXrqu53WCRGypk2ttuqncb/*");
        assert_eq!(xpub, "[73c5da0a/44'/0'/0']xpub6BosfCnifzxcFwrSzQiqu2DBVTshkCXacvNsWGYJVVhhawA7d4R5WSWGFNbi8Aw6ZRc1brxMyWMzG3DSSSSoekkudhUd9yLb6qx39T9nMdj/*");

        // m/49'/0'/0'
        let key_derive_cmd = Derive{
            path: DerivationPath::from_str("m/49'/0'/0'").unwrap(),
            xprv: ExtendedPrivKey::from_str("xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu").unwrap()
        };
        let result = key_derive_cmd.derive(Network::Bitcoin).unwrap();
        let xprv = result.get("xprv").unwrap().as_str().unwrap();
        let xpub = result.get("xpub").unwrap().as_str().unwrap();

        assert_eq!(xprv, "[73c5da0a/49'/0'/0']xprv9y7S1RkggDtZnP1RSzJ7PwUR4MUfF66Wz2jGv9TwJM52WLGmnnrQLLzBSTi7rNtBk4SGeQHBj5G4CuQvPXSn58BmhvX9vk6YzcMm37VuNYD/*");
        assert_eq!(xpub, "[73c5da0a/49'/0'/0']xpub6C6nQwHaWbSrzs5tZ1q7m5R9cPK9eYpNMFesiXsYrgc1P8bvLLAet9JfHjYXKjToD8cBRswJXXbbFpXgwsswVPAZzKMa1jUp2kVkGVUaJa7/*");

        // m/84'/0'/0'
        let key_derive_cmd = Derive{
            path: DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            xprv: ExtendedPrivKey::from_str("xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu").unwrap()
        };
        let result = key_derive_cmd.derive(Network::Bitcoin).unwrap();
        let xprv = result.get("xprv").unwrap().as_str().unwrap();
        let xpub = result.get("xpub").unwrap().as_str().unwrap();

        assert_eq!(xprv, "[73c5da0a/84'/0'/0']xprv9ybY78BftS5UGANki6oSifuQEjkpyAC8ZmBvBNTshQnCBcxnefjHS7buPMkkqhcRzmoGZ5bokx7GuyDAiktd5HemohAU4wV1ZPMDRmLpBMm/*");
        assert_eq!(xpub, "[73c5da0a/84'/0'/0']xpub6CatWdiZiodmUeTDp8LT5or8nmbKNcuyvz7WyksVFkKB4RHwCD3XyuvPEbvqAQY3rAPshWcMLoP2fMFMKHPJ4ZeZXYVUhLv1VMrjPC7PW6V/*");
    }
}
