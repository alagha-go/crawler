pub async fn get_request(url: String) -> Result<String> {
    Ok(CLIENT.get(&url).send().await?.text().await?)
}