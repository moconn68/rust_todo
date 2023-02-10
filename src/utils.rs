use dialoguer::Input;

// Internal crates
use super::data::{selection_display::SelectionDisplay, task_manager::TaskManager};

/// Gets current To-Do items, converts to displayable format, and adds them to SelectionDisplay list.
pub fn add_display_tasks(manager: &TaskManager, list: &mut Vec<SelectionDisplay>) {
    for task in manager.get_ordered_tasks_list() {
        list.push(SelectionDisplay::ExistingTask(task.to_owned()));
    }
}

/// Adds default list options to the bottom of the SelectionDisplay CLI list.
pub fn append_default_list_options(list: &mut Vec<SelectionDisplay>) {
    list.push(SelectionDisplay::Add);
    list.push(SelectionDisplay::Clear);
    list.push(SelectionDisplay::Quit);
}

/// Prompts and handles user input from the CLI for inputting a new task.
pub fn handle_add_task_input() -> std::result::Result<String, std::io::Error> {
    let new_task_details: String = Input::new()
        .with_prompt("Enter the details for your new task")
        .report(false)
        .interact_text()?;
    Ok(new_task_details)
}
