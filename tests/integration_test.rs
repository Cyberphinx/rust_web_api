use crate::common::TestSetup;

mod common;

#[tokio::test]
async fn can_fetch_hello_world_endpoint() -> Result<(), String> {
    // get common setups for testing
    let setup = TestSetup::setup().map_err(|err| err.message)?;

    // http post request to fetch endpoint url
    let result = setup.http_client.get("http://localhost:4000/").send().await;
    // test results
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Problem fetching hello world endpoint: {}", error)),
    }
}
