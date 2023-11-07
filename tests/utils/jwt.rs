use muttr_server::utils::jwt::get_claims_from_token;
use reqwest::Response;
use uuid::Uuid;

pub fn token_in_response_matches_user(user_id: Uuid, res: Response) -> Result<String, String> {
    let token = match res.headers().get("Authorization") {
        Some(t) => match t.to_str() {
            Ok(t) => t.to_string(),
            Err(e) => {
                return Err(format!("Failed to parse token to str: {:?}", e))
            }
        }
        None => return Err("Authorization header not present in response".to_string()),
    };
    let claims = get_claims_from_token(token.clone()).expect("Invalid token received");
    if claims.sub == user_id.to_string() {
        Ok(token)
    } else {
        Err("Token subscriber does not match user id".to_string())
    }
}