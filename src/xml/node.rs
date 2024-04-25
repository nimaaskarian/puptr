use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub (super) struct XmlNode {
    pub(super) name: String,
    pub(super) children: Vec<Self>,
    pub(super) attributes: HashMap<String, String>,
}
impl Default for XmlNode {
    fn default() -> Self {
        XmlNode {
            name: String::new(),
            children: vec![],
            attributes: HashMap::new(),
        }
    }
}

const TAB:&str = "   ";
impl XmlNode {
    pub fn new<S>(name: S) -> Self where
    S: ToString {
        XmlNode {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn new_with_children<S>(name: S, children: Vec<Self>)  -> Self where
    S:ToString {
        XmlNode {
            children,
            ..Self::new(name)
        }
    }

    pub fn new_with_attributes<S>(name: S, attributes: HashMap<String, String>) -> Self where
    S: ToString {
        XmlNode {
            attributes,
            ..Self::new(name)
        }
    }

    pub fn push(&mut self,item: Self) {
        self.children.push(item)
    }

    pub fn to_string(&self, depth: usize) -> String {
        let inner: String = self.children.iter()
            .map(|child| child.to_string(depth+1))
            .collect();
        let tabs = TAB.repeat(depth);
        let name = &self.name;
        format!("{tabs}<{name}>\n{inner}{tabs}</{name}>\n")
    }
}
