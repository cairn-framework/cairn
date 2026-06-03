pub fn verify_token(token: &str) -> bool {
    !token.is_empty()
}

pub fn hash_password(password: &str) -> String {
    password.to_owned()
}
