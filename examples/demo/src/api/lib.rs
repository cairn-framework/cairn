pub fn create_task(title: &str) -> Task {
    Task { id: 1, title: title.to_owned() }
}

pub struct Task {
    pub id: u64,
    pub title: String,
}

pub fn list_tasks() -> Vec<Task> {
    vec![]
}
