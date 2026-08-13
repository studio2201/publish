use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CrdtBlock {
    pub id: String,
    pub content: String,
    pub version: u64,
}

impl CrdtBlock {
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            version: 1,
        }
    }

    pub fn update(&mut self, new_content: impl Into<String>) {
        self.content = new_content.into();
        self.version = self.version.wrapping_add(1);
    }
}

pub struct MockP2pNode {
    pub blocks: HashMap<String, CrdtBlock>,
}

impl MockP2pNode {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn sync(&mut self, block: &CrdtBlock) -> Result<(), String> {
        let existing = self.blocks.get(&block.id);
        match existing {
            Some(e) if e.version >= block.version => {
                Ok(())
            }
            _ => {
                self.blocks.insert(block.id.clone(), block.clone());
                Ok(())
            }
        }
    }
}

thread_local! {
    pub static MOCK_NODE: RefCell<MockP2pNode> = RefCell::new(MockP2pNode::new());
}

pub fn sync_to_mock_network(block: &CrdtBlock) -> Result<(), String> {
    MOCK_NODE.with(|node| {
        let mut n = match node.try_borrow_mut() {
            Ok(n) => n,
            Err(_) => return Err("Failed to borrow mock node".to_string()),
        };
        n.sync(block)
    })
}
