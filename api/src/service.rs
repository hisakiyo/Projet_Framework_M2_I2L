use serde_json::Value;

pub async fn fetch_currencies() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest")
        .header("X-CMC_PRO_API_KEY", "")
        .send()
        .await?
        .text()
        .await?;

    let json_res: Value = serde_json::from_str(&res)?;

    let currencies = json_res["data"].as_array().unwrap();

    for currency in currencies {
        let currency_data = json!({
            "name": currency["name"],
            "symbol": currency["symbol"],
            "price": currency["quote"]["USD"]["price"]
        });
        println!("currency_data: {:?}", currency_data);

        client
            .post("http://localhost:8000/currencies")
            .body(currency_data.to_string())
            .send()
            .await?;
    }

    Ok(())
}
