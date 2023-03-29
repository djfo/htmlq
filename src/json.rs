use html5ever::serialize::Serializer;
use serde_json::{Map, Value};
use std::collections::LinkedList;

pub struct Json {
    stack: LinkedList<Node>,
}

impl Json {
    pub fn new() -> Json {
        let mut stack = LinkedList::new();
        let root = Node { tag_name: "".to_string(), attributes: Map::new(), children: LinkedList::new() };
        stack.push_front(root);
        Json {
            stack
        }
    }

    pub fn print(&self) {
        let hd = self.stack.front().expect("empty stack");
        print!("»{}«", hd.tag_name)
        // TODO: convert to JSON value
    }
}

struct Node {
    tag_name: String,
    attributes: Map<String, Value>,
    children: LinkedList<Node>,
}

impl Serializer for Json {
    fn start_elem<'a, AttrIter>(
        &mut self,
        name: html5ever::QualName,
        attrs: AttrIter,
    ) -> std::io::Result<()>
    where
        AttrIter: Iterator<Item = html5ever::serialize::AttrRef<'a>>,
    {
        let tag_name = name.local.to_lowercase();

        let mut m = Map::new();
        for (qual_name, value) in attrs {
            let k = qual_name.local.to_lowercase();
            m.insert(k, Value::String(value.to_string()));
        }

        let node = Node { tag_name, children: LinkedList::new(), attributes: m };
        self.stack.push_front(node);

        Ok(())
    }

    fn end_elem(&mut self, name: html5ever::QualName) -> std::io::Result<()> {
        let child = self.stack.pop_front();
        let parent = self.stack.pop_front();
        match (child, parent) {
            (Some(child), Some(mut parent)) => {
                parent.children.push_back(child);
                self.stack.push_front(parent);
                Ok(())
            }
            _ => panic!("inconsistent state")
        }
    }

    fn write_text(&mut self, text: &str) -> std::io::Result<()> {
        Ok(())
    }

    fn write_comment(&mut self, text: &str) -> std::io::Result<()> {
        Ok(())
    }

    fn write_doctype(&mut self, name: &str) -> std::io::Result<()> {
        Ok(())
    }

    fn write_processing_instruction(&mut self, target: &str, data: &str) -> std::io::Result<()> {
        Ok(())
    }
}
