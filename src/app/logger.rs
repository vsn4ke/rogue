use crate::utils::clamp;

#[derive(Default)]
pub struct Logger {
    entries: Vec<String>,
}

impl Logger {
    pub fn get_last_entries(&self, number: i32) -> Vec<String> {
        let number = clamp(0, self.entries.len() as i32, number);

        let mut entries = Vec::new();

        for i in 0..=number {
            let index = self.entries.len() as i32 - i - 1;

            if index >= 0 {
                entries.push(self.entries[index as usize].clone());
            }
        }

        entries
    }

    pub fn add_entries(&mut self, entry: impl Into<String>) {
        self.entries.push(entry.into());
    }
}
