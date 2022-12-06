#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    zero_to_prod::run().await
}
