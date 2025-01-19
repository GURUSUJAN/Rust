
/// Task structure
#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub is_done: bool,
}

/// Adds multiple tasks to the list
pub fn add_multiple_tasks(tasks: &mut Vec<Task>, descriptions: &[String]) {
    for description in descriptions {
        let id = tasks.len() + 1;
        tasks.push(Task {
            id,
            description: description.clone(),
            is_done: false,
        });
    }
}

/// Marks tasks as done based on their IDs
pub fn mark_tasks_done(tasks: &mut Vec<Task>, ids: &[usize]) {
    for id in ids {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == *id) {
            task.is_done = true;
        }
    }
}