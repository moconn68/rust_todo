use std::collections::{HashSet, LinkedList};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Task {
    pub complete: bool,
    pub details: String,
}

impl Task {
    pub fn new(details: &str) -> Self {
        Self {
            complete: false,
            details: details.to_string(),
        }
    }
}
pub struct TaskManager {
    tasks: HashSet<Task>,
}
impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: HashSet::new(),
        }
    }

    /// Adds a new task.
    ///
    /// This does not persist the task.
    pub fn add_new_task(&mut self, task_details: &str) {
        let new_task = Task::new(task_details);
        self.save_task(&new_task).expect("Error saving task!");
        self.tasks.insert(new_task);
    }

    /// Toggles the completion status of a task to the opposite of its current value.
    pub fn toggle_completion(&mut self, task: &Task) {
        if let Some(t) = &self.tasks.take(task) {
            self.tasks.insert(Task {
                complete: !t.complete,
                details: t.details.to_string(),
            });
        }
    }

    /// Deletes all completed tasks.
    pub fn remove_completed_tasks(&mut self) {
        self.tasks.retain(|task| !task.complete);
    }

    /// Gets list of tasks, with incomplete items listed before completed items.
    pub fn get_ordered_tasks_list(&self) -> LinkedList<&Task> {
        let mut ordered_tasks: LinkedList<&Task> = LinkedList::new();
        for task in &self.tasks {
            // let task = task.to_owned();
            if !task.complete {
                ordered_tasks.push_front(task);
            } else {
                ordered_tasks.push_back(task);
            }
        }
        ordered_tasks
    }

    /// Loads list of To-Do tasks saved from previous sessions.
    ///
    /// TODO need to integrate with persistence layer (sqlite3)
    pub fn load_persisted_tasks(&mut self) {
        if let Ok(tasks) = mock_load_tasks() {
            self.tasks = tasks;
        }
    }

    /// Saves a single task to disk.
    /// Will overwrite existing task data if the completion state has since changed.
    ///
    /// TODO need to integrate with persistence layer (sqlite3)
    pub fn save_task(&self, _task: &Task) -> Result<(), std::io::Error> {
        // TODO
        Ok(())
    }

    /// Takes the current list of tasks and saves them all to disk.
    /// This function will overwrite all existing saved tasks.
    ///
    /// TODO need to integrate with persistence layer (sqlite3)
    pub fn save_all_tasks(&self) -> Result<(), std::io::Error> {
        // TODO
        Ok(())
    }
}

/// Mock function for loading persistent tasks.
///
/// TODO remove with persistence impl (sqlite3)
fn mock_load_tasks() -> Result<HashSet<Task>, String> {
    Ok(HashSet::new())
}

#[cfg(test)]
mod tests {
    use super::{Task, TaskManager};

    #[test]
    fn default_task_incomplete() {
        let task = Task::new("Test task");
        assert_eq!(task.complete, false);
    }

    #[test]
    fn test_add_task() {
        let task = &Task::new("Test task");
        let mut task_manager = TaskManager::new();
        task_manager.add_new_task("Test task");
        assert!(task_manager.tasks.len() == 1);
        let map_task = task_manager.tasks.iter().next().unwrap();
        assert_eq!(map_task, task);
    }

    #[test]
    fn test_toggle_completion() {
        let task = &Task::new("Test task");
        let mut task_manager = TaskManager::new();
        task_manager.add_new_task("Test task");
        assert!(task_manager.tasks.len() == 1);
        task_manager.toggle_completion(task);
        let map_task = task_manager.tasks.iter().next().unwrap();
        assert!(map_task.complete);
        task_manager.toggle_completion(&map_task.clone());
        let map_task = task_manager.tasks.iter().next().unwrap();
        assert!(!map_task.complete);
    }

    #[test]
    fn test_completed_task_remove() {
        let complete_task = Task {
            complete: true,
            details: "Complete Task".to_string(),
        };
        let incomplete_task = &Task::new("Incomplete Task");
        let mut task_manager = TaskManager::new();
        task_manager.tasks.insert(complete_task);
        task_manager.tasks.insert(incomplete_task.clone());
        task_manager.remove_completed_tasks();
        assert!(task_manager.tasks.len() == 1);
        assert_eq!(task_manager.tasks.iter().next().unwrap(), incomplete_task);
    }

    #[test]
    fn test_completed_tasks_ordered_last() {
        let mut task_manager = TaskManager::new();
        let mut task1 = Task::new("task 1");
        task1.complete = true;
        let task2 = Task::new("task 2");
        let task3 = Task::new("task 3");
        task_manager.tasks.insert(task1.clone());
        task_manager.tasks.insert(task2);
        task_manager.tasks.insert(task3);
        let ordered_tasks = task_manager.get_ordered_tasks_list();
        let ordered_task_vec: Vec<&Task> = ordered_tasks.into_iter().collect();
        assert_eq!(ordered_task_vec[ordered_task_vec.len() - 1], &task1);
    }
}
