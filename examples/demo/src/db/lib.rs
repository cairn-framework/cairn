pub fn connect() -> Connection {
    Connection
}

pub struct Connection;

impl Connection {
    pub fn query(&self, _sql: &str) -> Vec<Row> {
        vec![]
    }
}

pub struct Row;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_returns_no_rows_yet() {
        assert!(connect().query("SELECT 1").is_empty());
    }
}
