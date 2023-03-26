use bitcoin::{consensus::deserialize, Transaction};

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bitcoin::consensus::deserialize;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let n = Arc::new(tokio::sync::Notify::new());
        let n2 = n.clone();
        let req = ehttp::Request::get("https://api.blockchair.com/bitcoin/raw/transaction/caa13042224074e91dc71193039bce3ef7340983cb0e0cd607326d0d73243064");
        ehttp::fetch(req, move |resp| {
            let val: serde_json::Value = serde_json::from_slice(&resp.unwrap().bytes).unwrap();
            println!("{}", val);
            let raw = val
                .get("data")
                .unwrap()
                .get("caa13042224074e91dc71193039bce3ef7340983cb0e0cd607326d0d73243064")
                .unwrap()
                .get("raw_transaction")
                .unwrap()
                .as_str()
                .unwrap();
            println!("raw {}", raw);
            let raw = hex::decode(raw).unwrap();
            let tx: Result<bitcoin::Transaction, _> = deserialize(&raw);
            println!("tx {:#?}", tx);
            n2.notify_one();
        });
        n.notified().await
    }

    #[tokio::test]
    async fn it_works2() {
        let n = Arc::new(tokio::sync::Notify::new());
        let n2 = n.clone();
        get_tx_then(
            "caa13042224074e91dc71193039bce3ef7340983cb0e0cd607326d0d73243064".into(),
            |resp| println!("{:#?}", resp),
        );
        n.notified().await
    }
}

pub fn get_tx_then(
    tx_id: String,
    on_done: impl FnOnce(anyhow::Result<Transaction>) + Send + 'static,
) {
    let req = ehttp::Request::get(format!(
        "https://api.blockchair.com/bitcoin/raw/transaction/{}",
        tx_id
    ));
    block_chair_api_then(req, move |resp| {
        let r = || -> _ {
            let raw = resp?
                .get("data")
                .ok_or(anyhow::format_err!("none data"))?
                .get(tx_id)
                .ok_or(anyhow::format_err!("none data"))?
                .get("raw_transaction")
                .ok_or(anyhow::format_err!("none data"))?
                .as_str()
                .unwrap()
                .to_string();
            let raw = hex::decode(raw).map_err(|e| anyhow::format_err!("{}", e))?;
            let tx: Result<bitcoin::Transaction, _> = deserialize(&raw);
            anyhow::Ok(tx.map_err(|e| anyhow::format_err!("{}", e))?)
        }();
        on_done(r)
    })
}

pub fn block_chair_api_then(
    req: ehttp::Request,
    on_done: impl FnOnce(anyhow::Result<serde_json::Value>) + Send + 'static,
) {
    ehttp::fetch(req, move |resp| {
        let r = || -> _ {
            let val: serde_json::Value = serde_json::from_slice(&resp.unwrap().bytes)
                .map_err(|e| anyhow::format_err!("{}", e))?;
            anyhow::Ok(val)
        }();
        on_done(r)
    })
}
