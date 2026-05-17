use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EsploraStats {
    funded_txo_sum: u64,
    spent_txo_sum: u64,
}

#[derive(Debug, Deserialize)]
struct EsploraAddress {
    chain_stats: EsploraStats,
    mempool_stats: EsploraStats,
}

#[derive(Debug, serde::Serialize)]
pub struct AddressData {
    pub confirmed_sats: u64,
    pub unconfirmed_sats: u64,
}

fn parse_address_response(json: &str) -> Result<AddressData, String> {
    let esplora: EsploraAddress =
        serde_json::from_str(json).map_err(|e| e.to_string())?;
    Ok(AddressData {
        confirmed_sats: esplora
            .chain_stats
            .funded_txo_sum
            .saturating_sub(esplora.chain_stats.spent_txo_sum),
        unconfirmed_sats: esplora
            .mempool_stats
            .funded_txo_sum
            .saturating_sub(esplora.mempool_stats.spent_txo_sum),
    })
}

pub async fn sync(address: &str) -> Result<AddressData, String> {
    let url = format!("https://mutinynet.com/api/address/{}", address);
    let body = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    parse_address_response(&body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_esplora_address_response() {
        let json = r#"{
            "address": "tb1qtest",
            "chain_stats": {
                "funded_txo_count": 2,
                "funded_txo_sum": 150000,
                "spent_txo_count": 1,
                "spent_txo_sum": 50000,
                "tx_count": 3
            },
            "mempool_stats": {
                "funded_txo_count": 1,
                "funded_txo_sum": 20000,
                "spent_txo_count": 0,
                "spent_txo_sum": 0,
                "tx_count": 1
            }
        }"#;

        let data = parse_address_response(json).unwrap();
        assert_eq!(data.confirmed_sats, 100000);
        assert_eq!(data.unconfirmed_sats, 20000);
    }

    #[tokio::test]
    #[ignore]
    async fn fetches_balance_from_live_signet() {
        // coinbase address on Mutinynet — always funded
        let address = "tb1qd28npep0s8frcm3y7dxqajkcy2m40eysplyr9v";
        let data = sync(address).await.unwrap();
        assert!(data.confirmed_sats > 0, "expected non-zero confirmed balance");
    }
}
