use bdk::bitcoin::util::base58;


pub fn prefixes(prefix: &str) -> Option<&str> {
    match prefix {
        // private
        "xprv" => Some("0488ade4"),
        "yprv" => Some("049d7878"),
        "zprv" => Some("04b2430c"),
        "Yprv" => Some("0295b005"),
        "Zprv" => Some("02aa7a99"),
        "tprv" => Some("04358394"),
        "uprv" => Some("044a4e28"),
        "vprv" => Some("045f18bc"),
        "Uprv" => Some("024285b5"),
        "Vprv" => Some("02575048"),
        // public
        "xpub" => Some("0488b21e"),
        "ypub" => Some("049d7cb2"),
        "zpub" => Some("04b24746"),
        "Ypub" => Some("0295b43f"),
        "Zpub" => Some("02aa7ed3"),
        "tpub" => Some("043587cf"),
        "upub" => Some("044a5262"),
        "vpub" => Some("045f1cf6"),
        "Upub" => Some("024289ef"),
        "Vpub" => Some("02575483"),
        _ => None,
    }
}

#[allow(nonstandard_style)]
pub struct AddressPrv {
    // Bitcoin
    pub xprv: String,
    pub yprv: String,
    pub zprv: String,
    pub Yprv: String,
    pub Zprv: String,
    // Testnet
    pub tprv: String,
    pub uprv: String,
    pub vprv: String,
    pub Uprv: String,
    pub Vprv: String,
}

#[allow(nonstandard_style)]
pub struct AddressPub {
    // Bitcoin
    pub xpub: String,
    pub ypub: String,
    pub zpub: String,
    pub Ypub: String,
    pub Zpub: String,
    // Testnet
    pub tpub: String,
    pub upub: String,
    pub vpub: String,
    pub Upub: String,
    pub Vpub: String,
}

pub enum Address<A, B> {
    Prv(A),
    Pub(B),
}

pub fn get_version(key: &str, format: &str) -> Option<String> {
    let prefix_origin: &str = prefixes(format).unwrap();

    let prefix_target: Vec<u8> = prefix_origin
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key_derive() {
        let zpub = get_version("xpub6C1SYcQFFDMvLQxLHU6wUHGY8EP6dBtjckDGZpj4hbxw4aVuKKNAmkFbpPwzQLibWmXJmamZHKsboCdZzHqTsbeU5vi169HHHNZGdNjG2qa", "zpub");
        assert_eq!(zpub, Some("zpub6qfy9wk5YaSt31LZxBgBtTTYUAfzWRsjSyFi8cWqTcihAn8MpdhJ1sZsrosAQA2SL3kvGXxgCeahZmrhRgfVU51fpc6rFxvFppgZQWZ7RWy".to_owned()));
    }
}
