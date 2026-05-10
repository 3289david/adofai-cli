use crate::parser::AdofaiLevel;

#[derive(Debug)]
pub struct UndoTree {
    states: Vec<UndoNode>,
    current: usize,
}

#[derive(Debug)]
struct UndoNode {
    snapshot: String,
    children: Vec<usize>,
    parent: Option<usize>,
    description: String,
}

impl UndoTree {
    pub fn new(initial: &AdofaiLevel) -> Self {
        let snapshot = serde_json::to_string(initial).unwrap_or_default();
        Self {
            states: vec![UndoNode {
                snapshot,
                children: vec![],
                parent: None,
                description: "Initial state".into(),
            }],
            current: 0,
        }
    }

    pub fn push(&mut self, level: &AdofaiLevel, description: &str) {
        let snapshot = serde_json::to_string(level).unwrap_or_default();
        let new_idx = self.states.len();

        self.states.push(UndoNode {
            snapshot,
            children: vec![],
            parent: Some(self.current),
            description: description.to_string(),
        });

        self.states[self.current].children.push(new_idx);
        self.current = new_idx;
    }

    pub fn undo(&mut self) -> Option<AdofaiLevel> {
        if let Some(parent) = self.states[self.current].parent {
            self.current = parent;
            self.restore_current()
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<AdofaiLevel> {
        let children = &self.states[self.current].children;
        if let Some(&last_child) = children.last() {
            self.current = last_child;
            self.restore_current()
        } else {
            None
        }
    }

    fn restore_current(&self) -> Option<AdofaiLevel> {
        serde_json::from_str(&self.states[self.current].snapshot).ok()
    }

    pub fn history_depth(&self) -> usize {
        let mut depth = 0;
        let mut idx = self.current;
        while let Some(parent) = self.states[idx].parent {
            depth += 1;
            idx = parent;
        }
        depth
    }

    pub fn current_description(&self) -> &str {
        &self.states[self.current].description
    }

    pub fn branch_count(&self) -> usize {
        self.states[self.current].children.len()
    }
}
