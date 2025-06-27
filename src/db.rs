use serde::{Deserialize, Serialize};
use chrono::DateTime;
use chrono::Local;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDescription {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    /// Next task ID
    next_id: u32,

    /// A list of tasks or habits
    pub tasks: Vec<TaskDescription>,

    /// Map completion by date
    pub completion: std::collections::HashMap<String, Vec<u32>>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            next_id: 1, // Start with ID 1
            tasks: Vec::new(),
            completion: std::collections::HashMap::new(),
        }
    }

    pub fn add_task(&mut self, name: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(TaskDescription { id, name: name.to_string() });
        id
    }

    pub fn is_task_done(&self, id: u32, date: DateTime<Local>) -> bool {
        let date_str = date.format("%Y-%m-%d").to_string();
        if let Some(completed_tasks) = self.completion.get(&date_str) {
            completed_tasks.contains(&id)
        } else {
            false
        }
    }

    pub fn get_task(&self, id: u32) -> Option<&TaskDescription> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn mark_task_done(&mut self, id: u32, date: DateTime<Local>) -> Result<(), String> {
        if self.get_task(id).is_some() {
            self.add_completion(id, date);
            Ok(())
        } else {
            Err(format!("Task with ID {} not found.", id))
        }
    }

    fn add_completion(&mut self, id: u32, date: DateTime<Local>) {
        let date_str = date.format("%Y-%m-%d").to_string();
        let completed_tasks = self.completion.entry(date_str).or_default();
        if !completed_tasks.contains(&id) {
            completed_tasks.push(id);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_task() {
        let mut data = super::Data::new();
        data.next_id = 1; // Reset ID for testing
        let id = data.add_task("Test Task");
        assert_eq!(id, 1);
        assert_eq!(data.tasks.len(), 1);
        assert_eq!(data.tasks[0].name, "Test Task");
    }
    #[test]
    fn test_mark_task_done() {
        let mut data = super::Data::new();
        let id = data.add_task("Test Task");
        let date = chrono::Local::now();
        assert!(data.mark_task_done(id, date).is_ok());
        assert!(data.is_task_done(id, date));
    }
    #[test]
    fn test_is_task_done() {
        let mut data = super::Data::new();
        let id = data.add_task("Test Task");
        let date = chrono::Local::now();
        assert!(!data.is_task_done(id, date));
        data.mark_task_done(id, date).unwrap();
        assert!(data.is_task_done(id, date));
    }
    #[test]
    fn test_get_task() {
        let mut data = super::Data::new();
        let id = data.add_task("Test Task");
        let task = data.get_task(id);
        assert!(task.is_some());
        assert_eq!(task.unwrap().name, "Test Task");
    }
    #[test]
    fn test_get_unexisting_task() {
        let data = super::Data::new();
        let task = data.get_task(999); // Non-existing ID
        assert!(task.is_none());
    }
    #[test]
    fn test_add_completion() {
        let mut data = super::Data::new();
        let id = data.add_task("Test Task");
        let date = chrono::Local::now();
        data.add_completion(id, date);
        assert!(data.is_task_done(id, date));
        // Check if the completion was added correctly
        let date_str = date.format("%Y-%m-%d").to_string();
        assert!(data.completion.contains_key(&date_str));
        assert!(data.completion[&date_str].contains(&id));
    }
    #[test]
    fn test_add_completion_multiple_times() {
        let mut data = super::Data::new();
        let id = data.add_task("Test Task");
        let date = chrono::Local::now();
        data.add_completion(id, date);
        data.add_completion(id, date); // Add again
        // Ensure it doesn't duplicate the ID in the completion list
        let date_str = date.format("%Y-%m-%d").to_string();
        assert_eq!(data.completion[&date_str].len(), 1);
        assert!(data.completion[&date_str].contains(&id));
    }
    #[test]
    fn test_mark_task_done_non_existing() {
        let mut data = super::Data::new();
        let date = chrono::Local::now();
        let result = data.mark_task_done(999, date); // Non-existing ID
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Task with ID 999 not found.");
    }
    #[test]
    fn test_add_task_with_multiple_calls() {
        let mut data = super::Data::new();
        let id1 = data.add_task("First Task");
        let id2 = data.add_task("Second Task");
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(data.tasks.len(), 2);
        assert_eq!(data.tasks[0].name, "First Task");
        assert_eq!(data.tasks[1].name, "Second Task");
    }
}

pub struct FileDatabase{
    filename: String,
    initialized: bool,
    data: Data,
}

#[allow(dead_code)]
impl FileDatabase {
    pub fn new(filename: &str) -> Self {
        FileDatabase {
            filename: filename.to_string(),
            initialized: false,
            data: Data::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), std::io::Error> {
        if !self.initialized {
            if std::fs::metadata(&self.filename).is_err() {
                self.write()?;
            } else {
                self.read()?;
            }
        }
        Ok(())
    }

    fn read(&mut self) -> Result<(), std::io::Error> {
        let data = std::fs::read_to_string(&self.filename)?;
        let data: Data = serde_json::from_str(&data)?;
        self.data = data;
        self.initialized = true;
        Ok(())
    }

    fn write(&mut self) -> Result<(), std::io::Error> {
        let data = serde_json::to_string(&self.data)?;
        std::fs::write(&self.filename, data)?;
        self.initialized = true;
        Ok(())
    }

    pub fn get_data(&self) -> Result<&Data, std::io::Error> {
        if !self.initialized {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database not initialized. Call read() first.",
            ));
        }
        Ok(&self.data)
    }

    pub fn get_mut_data(&mut self) -> Result<&mut Data, std::io::Error> {
        if !self.initialized {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database not initialized. Call read() first.",
            ));
        }
        Ok(&mut self.data)
    }
}

impl Drop for FileDatabase {
    fn drop(&mut self) {
        if let Err(e) = self.write() {
            eprintln!("Error writing to database on drop: {}", e);
        }
    }
}
