use crate::parser::{HtmlElement, Node};
use clap::Error;
use serde_json::{Map, Value};
use std::{collections::HashMap, f32::consts::E, hash::Hash, result};

#[derive(Debug)]
pub enum JsonValue {
    Text(String),
    Number(usize),
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
}

pub struct Evaluator {
    node: Node,
    json_template: Value,
}
impl Evaluator {
    pub fn new(node: Node, json_tempalate: Value) -> Self {
        Self {
            node: node,
            json_template: json_tempalate,
        }
    }

    pub fn evaluate(&self) -> Node {
        let parent_node = &self.node;
        match parent_node {
            Node::Text(text) => Node::Text(text.to_string()),
            Node::Element(elt) => Node::Text("method not implemented yet".to_string()),
        }
    }

    pub fn get_array_from_template(&mut self, path: String) -> Result<&Vec<Value>, String> {
        let json_content = &self.json_template;

        let mut value = &self.json_template;
        for p in path.split('.').into_iter() {
            match value.get(p) {
                Some(v) => value = v,
                None => return Err(format!("Path '{}' not found in JSON template.", path)),
            }
        }
        match value {
            Value::Array(a) => Ok(a),
            _ => Err("Value not found".to_string()),
        }
    }

    pub fn get_object_from_template(
        &mut self,
        path: String,
    ) -> Result<&Map<String, Value>, String> {
        let json_content = &self.json_template;

        let mut value = &self.json_template;
        for p in path.split(".").into_iter() {
            value = &value[p.to_string()]
        }
        match value {
            Value::Object(a) => Ok(a),
            _ => Err("Value not found".to_string()),
        }
    }
}

#[cfg(test)]
mod evaluator_tests {
    use crate::{evaluator, parser::Node};
    use serde::de::value;
    use serde_json::{json, Value};

    use super::Evaluator;

    #[test]
    pub fn test_placeholder_replacer() {
        let json_data = json!({
            "section": {
                "title": "Welcome!",
                "people": [
                    { "name": "John Doe" },
                    { "name": "Jane Smith" }
                ]
            }
        });


        println!("=============\n{:?}\n==============",json_data["section"]["people"].get(0));

        let mut evaluator = Evaluator::new(Node::Text("regis".to_string()), json_data);

        let array_result = evaluator.get_array_from_template("section.people".to_string());
        assert!(matches!(array_result, Ok(_))); 
        let object_result =  evaluator.get_object_from_template("section".to_string());
        println!("========{:?}========", object_result.clone().unwrap());
        assert!(matches!(object_result, Ok(_)));

    }
}
