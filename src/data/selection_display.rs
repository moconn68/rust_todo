use std::fmt::{Display, Formatter};

use super::task_manager::Task;

const ADD_STR: &str = "📝 Add New Task";
const CLEAR_STR: &str = "❌ Clear Finished Tasks";
const QUIT_STR: &str = "👋 Quit";
const COMPLETE_MARKER: &str = "✅";
const INCOMPLETE_MARKER: &str = "⭕️";

#[derive(Debug)]
pub enum SelectionDisplay {
    Add,
    ExistingTask(Task),
    Clear,
    Quit,
}

impl Display for SelectionDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectionDisplay::Add => write!(f, "{ADD_STR}"),
            SelectionDisplay::ExistingTask(task) => {
                let marker = match task.complete {
                    true => COMPLETE_MARKER,
                    false => INCOMPLETE_MARKER,
                };
                write!(f, "{} {}", marker, task.details)
            }
            SelectionDisplay::Clear => write!(f, "{CLEAR_STR}"),
            SelectionDisplay::Quit => write!(f, "{QUIT_STR}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::selection_display::{
        ADD_STR, CLEAR_STR, COMPLETE_MARKER, INCOMPLETE_MARKER, QUIT_STR,
    };

    use super::super::task_manager::Task;
    use super::SelectionDisplay;

    #[test]
    fn test_selection_display_add() {
        let selection = SelectionDisplay::Add;
        assert_eq!(format!("{}", selection), ADD_STR);
    }

    #[test]
    fn test_selection_display_existing_task_complete() {
        let task_details = "Write tests";
        let task = Task {
            details: String::from(task_details),
            complete: true,
        };
        let selection = SelectionDisplay::ExistingTask(task);
        assert_eq!(
            format!("{}", selection),
            format!("{} {}", COMPLETE_MARKER, task_details)
        );
    }

    #[test]
    fn test_selection_display_existing_task_incomplete() {
        let task_details = "Add new features";
        let task = Task {
            details: String::from(task_details),
            complete: false,
        };
        let selection = SelectionDisplay::ExistingTask(task);
        assert_eq!(
            format!("{}", selection),
            format!("{} {}", INCOMPLETE_MARKER, task_details)
        );
    }

    #[test]
    fn test_selection_display_clear() {
        let selection = SelectionDisplay::Clear;
        assert_eq!(format!("{}", selection), CLEAR_STR);
    }

    #[test]
    fn test_selection_display_quit() {
        let selection = SelectionDisplay::Quit;
        assert_eq!(format!("{}", selection), QUIT_STR);
    }
}
