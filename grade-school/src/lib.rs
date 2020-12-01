use std::collections::HashMap;

pub struct School {
    students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            students: HashMap::default(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students
            .entry(grade)
            .or_insert(vec![])
            .push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut g = self.students.keys().copied().collect::<Vec<u32>>();
        g.sort_unstable();
        g
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if let Some(mut s) = self.students.get(&grade).cloned() {
            s.sort_unstable();
            return Some(s);
        }
        None
    }
}
