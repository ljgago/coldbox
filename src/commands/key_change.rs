use crate::cli::Change;

use std::str::FromStr;

use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::base58;
use bdk::bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, Fingerprint};
// use bdk::bitcoin::Network;
use bdk::keys::KeyError::Message;
use bdk::Error;

use serde_json::json;

fn prefixes(prefix: &str) -> Option<&str> {
    match prefix {
        // private bitcoin
        "xprv" => Some("0488ade4"),
        "yprv" => Some("049d7878"),
        "zprv" => Some("04b2430c"),
        "Yprv" => Some("0295b005"),
        "Zprv" => Some("02aa7a99"),
        // private testnet
        "tprv" => Some("04358394"),
        "uprv" => Some("044a4e28"),
        "vprv" => Some("045f18bc"),
        "Uprv" => Some("024285b5"),
        "Vprv" => Some("02575048"),
        // public bitcoin
        "xpub" => Some("0488b21e"),
        "ypub" => Some("049d7cb2"),
        "zpub" => Some("04b24746"),
        "Ypub" => Some("0295b43f"),
        "Zpub" => Some("02aa7ed3"),
        // public testnet
        "tpub" => Some("043587cf"),
        "upub" => Some("044a5262"),
        "vpub" => Some("045f1cf6"),
        "Upub" => Some("024289ef"),
        "Vpub" => Some("02575483"),
        _ => None,
    }
}

fn check_format(key: &str, target_format: &str) -> bool {
    let (prefix, _) = key.split_at(4);
    let (_, key_type) = prefix.split_at(1);
    let (_, target_type) = target_format.split_at(1);

    if key_type == target_type {
        true
    } else {
        false
    }
}

fn converter(key: &str, format: &str) -> Option<String> {
    let prefix_origin = prefixes(format);

    if prefix_origin.is_none() {
        return None;
    }

    let prefix_target: Vec<u8> = prefix_origin
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|dig| {
            let d: String = dig.into_iter().collect();
            u8::from_str_radix(&d, 16).unwrap()
        })
        .collect();

    let data = base58::from_check(key).unwrap();
    let data = [&prefix_target, &data[4..]].concat();

    Some(base58::check_encode_slice(&data))
}

pub fn key_info(key: &str) -> Option<(Fingerprint, Fingerprint, u8)> {
    let (prefix, _) = key.split_at(4);
    let secp = Secp256k1::new();

    match prefix {
        "xprv" | "yprv" | "zprv" | "Yprv" | "Zprv" => {
            let xprv_string = converter(key, "xprv").unwrap();
            let xprv = ExtendedPrivKey::from_str(&xprv_string).unwrap();
            Some((xprv.fingerprint(&secp), xprv.parent_fingerprint, xprv.depth))
        }
        "tprv" | "uprv" | "vprv" | "Uprv" | "Vprv" => {
            let tprv_string = converter(key, "tprv").unwrap();
            let tprv = ExtendedPrivKey::from_str(&tprv_string).unwrap();
            Some((tprv.fingerprint(&secp), tprv.parent_fingerprint, tprv.depth))
        }
        "xpub" | "ypub" | "zpub" | "Ypub" | "Zpub" => {
            let xpub_string = converter(key, "xpub").unwrap();
            let xpub = ExtendedPubKey::from_str(&xpub_string).unwrap();
            Some((xpub.fingerprint(), xpub.parent_fingerprint, xpub.depth))
        }
        "tpub" | "upub" | "vpub" | "Upub" | "Vpub" => {
            let tpub_string = converter(key, "tpub").unwrap();
            let tpub = ExtendedPubKey::from_str(&tpub_string).unwrap();
            Some((tpub.fingerprint(), tpub.parent_fingerprint, tpub.depth))
        }
        _ => None,
    }
}

impl Change {
    pub fn change(&self) -> Result<serde_json::Value, Error> {
        if check_format(&self.key, &self.format) == false {
            return Err(Error::Key(Message(format!(
                "Invalid target format to {}",
                &self.format
            ))));
        }

        let info = key_info(&self.key);
        if info.is_none() {
            return Err(Error::Key(Message(format!(
                "Invalid target format to {}",
                &self.format
            ))));
        }

        let key_target = converter(&self.key, &self.format);
        if key_target.is_none() {
            return Err(Error::Key(Message(format!(
                "Invalid target format to {}",
                &self.format
            ))));
        }

        let (fingerprint, parent_fingerprint, depth) = info.unwrap();


        Ok(json!({
            "fingerprint": fingerprint,
            "parent_fingerprint": parent_fingerprint,
            "depth": depth,
            &self.format: key_target.unwrap()
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key_change() {
        let key_change_cmd = Change{
            format: String::from("zpub"),
            key: String::from("xpub6CatWdiZiodmUeTDp8LT5or8nmbKNcuyvz7WyksVFkKB4RHwCD3XyuvPEbvqAQY3rAPshWcMLoP2fMFMKHPJ4ZeZXYVUhLv1VMrjPC7PW6V"),
        };
        let result = key_change_cmd.change().unwrap();
        let zpub = result.get("zpub").unwrap().as_str().unwrap();
        let parent_fingerprint = result.get("parent_fingerprint").unwrap().as_str().unwrap();
        let depth = result.get("depth").unwrap();

        assert_eq!(zpub, "zpub6rFR7y4Q2AijBEqTUquhVz398htDFrtymD9xYYfG1m4wAcvPhXNfE3EfH1r1ADqtfSdVCToUG868RvUUkgDKf31mGDtKsAYz2oz2AGutZYs");
        assert_eq!(parent_fingerprint, "7ef32bdb");
        assert_eq!(depth, 3);

        let key_change_cmd = Change{
            format: String::from("zprv"),
            key: String::from("xpub6C1SYcQFFDMvLQxLHU6wUHGY8EP6dBtjckDGZpj4hbxw4aVuKKNAmkFbpPwzQLibWmXJmamZHKsboCdZzHqTsbeU5vi169HHHNZGdNjG2qa"),
        };
        let result = key_change_cmd.change();

        assert!(result.is_err());
    }
}
