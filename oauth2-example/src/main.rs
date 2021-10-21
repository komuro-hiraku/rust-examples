use oauth2::basic::BasicClient;
use oauth2::{ClientId, ClientSecret, TokenUrl, RedirectUrl, AuthUrl, PkceCodeChallenge, CsrfToken, Scope, AuthorizationCode};
use std::io;
use oauth2::reqwest::http_client;

fn main() {

    // blocking access
    println!("OAuth2.0 Authorization Code Grant flow Sample");

    // TODO: Get Environment Variables
    let client = BasicClient::new(
        ClientId::new("CLIENT_ID".to_string()),
        Some(ClientSecret::new("CLIENT_SECRET".to_string())),
        AuthUrl::new("AUTH_URL".to_string()).unwrap(),
        Some(TokenUrl::new("TOKEN_URL".to_string()).unwrap())
    )
        .set_redirect_uri(RedirectUrl::new("REDIRECT_URL".to_string()).unwrap());

    // PKCE Challenge
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random())
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("profile".to_string()))

        // set PKCE Code challenge
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    println!("Input Get Auth_code");
    let mut auth_code = String::new();
    io::stdin().read_line(&mut auth_code)
        .expect("Failed to read line");

    let token_result =
        client
            .exchange_code(AuthorizationCode::new(auth_code.to_string()))
            .set_pkce_verifier(pkce_verifier)
            .request(http_client).unwrap();

    println!("Token Result: {:?}", token_result);
}
