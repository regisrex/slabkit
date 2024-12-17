use crate::parser::{HtmlElement, Node};
use serde_json::{json, Map, Value};
use std::{collections::HashMap, f32::consts::E};

pub struct Evaluator {
    json_template: Value,
}

impl Evaluator {
    pub fn new(json_tempalate: Value) -> Self {
        Self {
            json_template: json_tempalate,
        }
    }
    pub fn evaluate(&mut self, node: Node) -> Node {
        let mut evaluated_node = match node {
            Node::Text(text) => Node::Text(self.replace_placeholders(text)),
            Node::Element(mut elt) => {
                if elt.tag == "slk-datamap" {
                    return self.process_datamap(elt);
                }
                let mut new_attributes: HashMap<String, String> = HashMap::new();

                for attr in elt.attributes.iter_mut() {
                    let (key, value) = attr;
                    *value = self.replace_placeholders(value.clone());

                    new_attributes.insert(key.clone(), value.clone());
                }

                if elt.children.len() == 0 {
                    return Node::Element(HtmlElement {
                        tag: elt.tag,
                        attributes: new_attributes,
                        children: Vec::new(),
                    });
                }
                let mut processed_children : Vec<Box<Node>> = Vec::new();

                for child in elt.children.drain(..) {
                    processed_children
                        .push(Box::new(Evaluator::new(self.json_template.clone()).evaluate(*child)));
                }
                elt.children = processed_children;
                Node::Element(elt)
            }
        };
        evaluated_node
    }

    pub fn get_literal_from_template(&mut self, path: String) -> Result<String, String> {
        let json_content = &self.json_template;

        let mut value = &self.json_template;
        for p in path.split('.').into_iter() {
            match value.get(p) {
                Some(v) => value = v,
                None => return Err(format!("Path '{}' not found in JSON template.", path)),
            }
        }
        match value {
            Value::String(a) => Ok(a.to_string()),
            Value::Bool(a) => Ok(a.to_string()),
            Value::Number(a) => Ok(a.to_string()),
            _ => Err("Value not found".to_string()),
        }
    }
    pub fn get_array_from_template(&mut self, path: String) -> Result<&Vec<Value>, String> {
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

        for p in path.split('.').into_iter() {
            match value.get(p) {
                Some(v) => value = v,
                None => return Err(format!("Path '{}' not found in JSON template.", path)),
            }
        }

        match value {
            Value::Object(a) => Ok(a),
            _ => Err("Value not found".to_string()),
        }
    }

    pub fn replace_placeholders(&mut self, input: String) -> String {
        let mut result = input.to_string();
        let placeholder_pattern = regex::Regex::new(r"!\{([a-zA-Z0-9_.]+)\}!").unwrap();

        // Replace each placeholder with the corresponding value from the JSON
        for caps in placeholder_pattern.captures_iter(&input) {
            if let Some(placeholder) = caps.get(1) {
                let key_path = placeholder.as_str();

                if let Ok(value) = self.get_literal_from_template(key_path.to_string()) {
                    result = result.replace(&format!("!{{{}}}!", key_path), &value);
                }
            }
        }
        result
    }

    pub fn unwrap_placeholders(&mut self, text: String) -> String {
        let mut result = text.to_string();
        result = result.replace('{', "");
        result = result.replace('}', "");
        result = result.replace('!', "");
        result = result.replace('!', "");
        result = result.replace(' ', "");

        result
    }
    pub fn process_datamap(&mut self, mut elt: HtmlElement) -> Node {

        if (elt.children.len() > 1) {
            panic!("Error: slk-datamap can only have one child element");
        }
        let data_path = match elt.attributes.remove("data") {
            Some(path) => self.unwrap_placeholders(path.clone()),
            None => return Node::Element(elt),
        };

        let selector = match elt.attributes.remove("selector") {
            Some(selector) => self.unwrap_placeholders(selector.clone()),
            None => return Node::Element(elt),
        };
        // Clone the template to avoid borrowing issues
        let data_array = match self.get_array_from_template(data_path) {
            Ok(array) => array.clone(),
            Err(_) => return Node::Element(elt),
        };
 

        let mut processed_children: Vec<Box<Node>> = Vec::new();

        let first_child = elt.children.first();
        if first_child.is_none() {
            return Node::Element(elt);
        }

        for item in data_array.into_boxed_slice().iter() {
            let item_json: Value = json!({ selector.clone(): item });
            if let Some(child) = first_child {
                processed_children.push(Box::new(Evaluator::new(item_json).evaluate(*child.clone())));
            } else {
                return Node::Element(elt);
            }
        }

        Node::Element(HtmlElement {
            tag: "div".to_string(),
            attributes: elt.attributes.clone(),
            children: processed_children,
        })
    }
}
