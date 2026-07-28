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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task_keeps_the_title() {
        assert_eq!(create_task("write docs").title, "write docs");
    }

    #[test]
    fn list_tasks_starts_empty() {
        assert!(list_tasks().is_empty());
    }
}
