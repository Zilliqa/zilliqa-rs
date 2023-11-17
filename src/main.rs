use jsonrpc::simple_http::{self, SimpleHttpTransport};
use jsonrpc::Client;

fn client(url: &str, user: &str, pass: &str) -> Result<Client, simple_http::Error> {
    let t = SimpleHttpTransport::builder()
        .url(url)?
        .auth(user, Some(pass))
        .build();

    Ok(Client::with_transport(t))
}

fn main() {
    let client = client("127.0.0.1:5555", "user", "pass").expect("failed to create client");
    let request = client.build_request("web3_clientVersion", &[]);
    let response = client.send_request(request).expect("send_request failed");

    // For other commands this would be a struct matching the returned json.
    let result: String = response
        .result()
        .expect("response is an error, use check_error");
    println!("Version: {}", result);
}
