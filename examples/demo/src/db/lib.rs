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
