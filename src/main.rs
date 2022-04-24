/*
RGET by Alexander Abraham,
a.k.a. "Angeldust Duke" a.k.a. "The Black Unicorn".
Licensed under the MIT license.
*/

use rget::cli;

/// We use tokio's
/// asynchronous "main"
/// main macro here to allow
/// an asynchronous "main "function.
#[tokio::main]
async fn main() {
    cli().await;
}
