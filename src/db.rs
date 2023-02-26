use std::{collections::HashSet, path::Path};

use crate::data::task_manager::Task;

const DEFAULT_DB_PATH_STR: &str = "./.rust_todo.db";

pub struct DBHelper {
    connection: sqlite::Connection,
}

impl Default for DBHelper {
    fn default() -> Self {
        Self::new(Path::new(DEFAULT_DB_PATH_STR)).unwrap()
    }
}

impl DBHelper {
    /// Creates a new instance of `DBHelper`.
    ///
    /// param `db_path` file path to sqlite database.
    ///
    /// Returns `Some(DBHelper)` or `None` if db could not be opened/created at given path.
    pub fn new(db_path: &Path) -> Option<Self> {
        match sqlite::open(db_path) {
            Ok(connection) => {
                let db = Self { connection };
                db.create_default_db().unwrap();
                Some(db)
            }
            Err(_) => None,
        }
    }

    /// Instantiates the `DBHelper` with the default table structure for handling `Task`s.
    ///
    /// Retuns an sqlite `Result`.
    fn create_default_db(&self) -> sqlite::Result<()> {
        let query = "CREATE TABLE IF NOT EXISTS tasks (details TEXT UNIQUE, complete INTEGER);";
        self.connection.execute(query)
    }

    /// Runs reset query to clear `Task` table in DB.
    ///
    /// TODO currently unused.
    fn _reset_db(&self) -> sqlite::Result<()> {
        let query = "DROP TABLE IF EXISTS tasks;";
        self.connection.execute(query)
    }

    /// Queries DB for all persisted tasks.
    ///
    /// Returns `Ok(HashSet<Task>)` of tasks or `Err` if there was an sqlite error.
    pub fn load_tasks(&self) -> sqlite::Result<HashSet<Task>> {
        let mut tasks: HashSet<Task> = HashSet::new();
        let query = "SELECT * from tasks;";
        for row in self
            .connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap())
        {
            let details = row.read::<&str, _>("details").to_string();
            let complete = matches!(row.read::<i64, _>("complete"), 1);
            let task = Task { details, complete };
            tasks.insert(task);
        }
        Ok(tasks)
    }

    /// Persists a single `Task` in the database.
    ///
    /// param `task` to persist.
    ///
    /// Returns a sqlite `Result`.
    ///
    /// TODO allow dead code because used only in testing.
    #[allow(dead_code)]
    pub fn insert_task(&self, task: &Task) -> sqlite::Result<()> {
        let complete_i64 = match task.complete {
            true => 1,
            false => 0,
        };
        let query = format!(
            "INSERT OR REPLACE INTO tasks VALUES (\"{}\", {})",
            task.details, complete_i64
        );
        self.connection.execute(query)
    }

    /// Persists a collection (`HashSet`) of tasks in the database.
    ///
    /// param `tasks` to persist.
    ///
    /// Returns a sqlite `Result`.
    pub fn insert_tasks(&self, tasks: &HashSet<Task>) -> sqlite::Result<()> {
        let mut query = String::from("BEGIN TRANSACTION;\n");
        for task in tasks {
            let complete_i64 = match task.complete {
                true => 1,
                false => 0,
            };
            let q = format!(
                "INSERT OR REPLACE INTO tasks VALUES(\"{}\", {});\n",
                task.details, complete_i64
            );
            query.push_str(q.as_str());
        }
        query.push_str("COMMIT");
        self.connection.execute(query)
    }

    /// Deletes a single task from the database.
    ///
    /// param `task` to delete.
    ///
    /// Returns sqlite `Result`.
    ///
    /// TODO currently unused.
    pub fn _delete_task(&self, task: &Task) -> sqlite::Result<()> {
        let query = format!("DELETE FROM tasks WHERE details = \"{}\"", task.details);
        self.connection.execute(query)
    }

    /// Deletes a collection (`HashSet`) of tasks from the database.
    ///
    /// param `tasks` to delete.
    ///
    /// Returns a sqlite `Result`.
    ///
    /// TODO currently unused.
    pub fn _delete_tasks(&self, tasks: &HashSet<Task>) -> sqlite::Result<()> {
        let mut query = String::from("BEGIN TRANSACTION;\n");
        for task in tasks {
            query.push_str(
                format!("DELETE FROM tasks WHERE details = \"{}\";\n", task.details).as_str(),
            );
        }
        query.push_str("COMMIT");
        self.connection.execute(query)
    }

    /// Deletes all tasks in the database which have been marked as 'complete'.
    ///
    /// Returns an sqlite `Result`.
    pub fn delete_completed_tasks(&self) -> sqlite::Result<()> {
        let query = "DELETE FROM tasks WHERE complete = 1;";
        self.connection.execute(query)
    }
}

// Currently all tests here need to be run consequtively (`cargo test -- --test-threads=1`) because the DB operations
// always happen on the same DB file. This should eventually be refactored so that the tests operate on separate files.
#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::data::task_manager::Task;

    use super::DBHelper;

    const TEST_DB_PATH_STR: &str = "./.test_db.db";

    /// Run before the start of every test to create a new DB for testing.
    ///
    /// Needed as long as there is not a test DB with a separate file per instance.
    fn setup() -> DBHelper {
        DBHelper::new(Path::new(TEST_DB_PATH_STR)).unwrap()
    }

    /// Run after the conclusion of every test to clean up the testing DB.
    ///
    /// Needed as long as there is not a test DB which implements the Drop trait for clean-up (delete).
    fn teardown() {
        std::fs::remove_file(Path::new(TEST_DB_PATH_STR)).unwrap();
    }

    #[test]
    fn test_single_insert() {
        let db = setup();
        let task = Task::new("Test task");
        db.insert_task(&task).unwrap();
        let tasks = db.load_tasks().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks.iter().next().unwrap(), &task);
        teardown();
    }

    // TODO add more tests after creating a DBHelper specifically for testing purposes.
}
