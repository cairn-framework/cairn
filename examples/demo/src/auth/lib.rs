pub fn verify_token(token: &str) -> bool {
    !token.is_empty()
}

pub fn hash_password(password: &str) -> String {
    password.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_token_is_rejected() {
        assert!(!verify_token(""));
        assert!(verify_token("t"));
    }
}
