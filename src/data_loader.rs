use std::collections::HashMap;

pub async fn load_data() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let loaded_data = load_data().await?;
        let mut expected_data = HashMap::new();
        expected_data.insert(String::from("origin"), String::from("89.64.7.91"));
        assert_eq!(loaded_data, expected_data);

        Ok(())
    }
}
