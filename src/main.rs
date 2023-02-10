use dialoguer::{console::Term, theme::ColorfulTheme, Select};

// Internal crates
mod data;
mod utils;
use data::{selection_display::SelectionDisplay, task_manager::TaskManager};
fn main() -> std::io::Result<()> {
    let mut task_manager = TaskManager::new();
    task_manager.load_persisted_tasks();
    loop {
        // Main loop until user selects Quit option

        let mut selection_options: Vec<SelectionDisplay> = vec![];
        utils::add_display_tasks(&task_manager, &mut selection_options);
        utils::append_default_list_options(&mut selection_options);

        let selection_idx = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("To-Do List")
            .report(false)
            .items(&selection_options)
            .default(0)
            .interact_on_opt(&Term::stderr())?
        {
            Some(idx) => idx,
            None => break,
        };

        let selection_item = selection_options
            .get(selection_idx)
            .expect("User selected option not in list!");

        match selection_item {
            SelectionDisplay::Add => {
                let new_task = utils::handle_add_task_input()?;
                task_manager.add_new_task(&new_task);
            }
            SelectionDisplay::ExistingTask(task) => task_manager.toggle_completion(task),
            SelectionDisplay::Clear => task_manager.remove_completed_tasks(),
            SelectionDisplay::Quit => break,
        };
    }
    // Clean up before graceful exit
    task_manager.save_all_tasks()?;
    Ok(())
}
