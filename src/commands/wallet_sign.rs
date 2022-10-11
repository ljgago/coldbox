use crate::cli::Sign;

use bdk::bitcoin::base64;
use bdk::bitcoin::consensus::{deserialize, serialize};
use bdk::bitcoin::psbt::PartiallySignedTransaction;
use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::{Error, SignOptions, Wallet};

use serde_json::json;

impl Sign {
    pub fn sign(
        self,
        network: Network,
        descriptor: String,
        verbose: bool,
    ) -> Result<serde_json::Value, Error> {
        let wallet = Wallet::new(&descriptor, None, network, MemoryDatabase::default())?;

        let psbt = base64::decode(&self.psbt).map_err(|e| Error::Generic(e.to_string()))?;
        let mut psbt: PartiallySignedTransaction = deserialize(&psbt)?;

        let finalized = wallet.sign(&mut psbt, SignOptions::default())?;

        match verbose {
            true => Ok(json!({
                "is_finalized": finalized,
                "psbt": base64::encode(&serialize(&psbt)),
                "deserialized_psbt": psbt
            })),
            false => Ok(json!({
                "is_finalized": finalized,
                "psbt": base64::encode(&serialize(&psbt))
            })),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign() {
        // mnemonic: abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
        // path: "m/84'/1'/0'"
        let descriptor = String::from("wpkh([73c5da0a/84'/1'/0']tprv8fSjiqEQ8YG7Ro7gw2ScwcvweYuuWi1ZzGUtrPz918HvDtBzL5s2voFTrN4y3yUwj5cYD54pLhxk6NKCzHUjcka3zbK\
            jbTEcsuAnkzbjhkL/0/*)");

        let key_sign_cmd = Sign{
            psbt: String::from("cHNidP8BAFICAAAAAQ3TM54hf/xyGNQ3RwZ9zykQsbogN20RNgReU5yir1+IAQAAAAD9////ARAnAAAAAAAAFgAUGFotGcPnrJnJg8Mz1Htu+ejz1V8JwiMATwEENYf\
                PAw70sa+AAAAAPIwgN+5MFiHaDTSNtRFjcJpiLQ0oON3m2EGcUfYwHGIDuI4Pvj9kYzftk7wMDzuEP899JYnl7IhHVOZAICeokLQQc8XaClQAAIABAACAAAAAgAABAHECAAAAAYDXHmXP+m\
                71ecUGq9jRgehhR8fuWtYc89I8qZQE2zemAQAAAAD9////AlKoYwAAAAAAFgAUQsjMYy/RsZmttmBmh8tm3sjDdRudJwAAAAAAABYAFPfWsQ9R/oRyjJCODiuUa9dl46c/A8IjAAEBH50nA\
                AAAAAAAFgAU99axD1H+hHKMkI4OK5Rr12Xjpz8BAwQBAAAAIgYC/0PHIY//lNUR6ikHqAV5i0XvloIezNiePTxpkR+h7SEYc8XaClQAAIABAACAAAAAgAAAAAASAAAAAAA="),
        };

        let result = key_sign_cmd
            .sign(Network::Testnet, descriptor, false)
            .unwrap();
        let is_finalized = result.get("is_finalized").unwrap().as_bool().unwrap();
        let psbt = result.get("psbt").unwrap().as_str().unwrap();

        assert_eq!(is_finalized, true);
        assert_eq!(psbt, "cHNidP8BAFICAAAAAQ3TM54hf/xyGNQ3RwZ9zykQsbogN20RNgReU5yir1+IAQAAAAD9////ARAnAAAAAAAAFgAUGFotGcPnrJnJg8Mz1Htu+ejz1V8JwiMATwEENYfPAw70s\
            a+AAAAAPIwgN+5MFiHaDTSNtRFjcJpiLQ0oON3m2EGcUfYwHGIDuI4Pvj9kYzftk7wMDzuEP899JYnl7IhHVOZAICeokLQQc8XaClQAAIABAACAAAAAgAABAHECAAAAAYDXHmXP+m71ecUGq9jR\
            gehhR8fuWtYc89I8qZQE2zemAQAAAAD9////AlKoYwAAAAAAFgAUQsjMYy/RsZmttmBmh8tm3sjDdRudJwAAAAAAABYAFPfWsQ9R/oRyjJCODiuUa9dl46c/A8IjAAEBH50nAAAAAAAAFgAU99a\
            xD1H+hHKMkI4OK5Rr12Xjpz8BAwQBAAAAIgYC/0PHIY//lNUR6ikHqAV5i0XvloIezNiePTxpkR+h7SEYc8XaClQAAIABAACAAAAAgAAAAAASAAAAAQcAAQhrAkcwRAIgK4MR5PkVnZoOriZAUX\
            rZNH0FR8aMWCOci91vCKcXPMkCIFhPzpRZASact1ejfqjtZ0Bc/crETXIKBawXYAyfsO0GASEC/0PHIY//lNUR6ikHqAV5i0XvloIezNiePTxpkR+h7SEAAA==");
    }
}
