/// This sample showcases execution of stored procedure
/// Create stored procedure called test_proc, like so:
/// function f(personToGreet) {
///     var context = getContext();
///     var response = context.getResponse();
///     response.setBody("Hello, " + personToGreet);
/// }
use azure_sdk_cosmos::prelude::*;
use azure_sdk_cosmos::stored_procedure::Parameters;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new(account, TokenType::Master, &master_key)?;

    let client = ClientBuilder::new(authorization_token)?;

    let ret = client
        .with_database(&database)
        .with_collection(&collection)
        .with_stored_procedure(&"test_proc")
        .execute_stored_procedure()
        .with_parameters(Parameters::new().push("Robert")?)
        .execute::<serde_json::Value>()
        .await?;

    println!("Response object:\n{:#?}", ret);
    println!("Response as JSON:\n{}", ret.payload.to_string());

    Ok(())
}
