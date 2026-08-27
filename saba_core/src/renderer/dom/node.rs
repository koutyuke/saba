use core::{
    cell::RefCell,
    fmt::{Display, Formatter},
    str::FromStr,
};

use alloc::{
    format,
    rc::{Rc, Weak},
    string::String,
    vec::Vec,
};

use crate::renderer::html::attribute::Attribute;

#[derive(Debug, Clone)]
pub struct Node {
    // ノードの種類
    pub kind: NodeKind,

    // DOM ウィンドウの弱参照
    window: Weak<RefCell<Window>>,

    // 親ノードへの弱参照
    parent: Weak<RefCell<Node>>,

    // ノードの一番はじめの子ノード
    first_child: Option<Rc<RefCell<Node>>>,

    // ノードの一番最後の子ノード
    last_child: Weak<RefCell<Node>>,

    // 前の兄弟ノードへの弱参照
    previous_sibling: Weak<RefCell<Node>>,

    // 次の兄弟ノードへの共有参照
    next_sibling: Option<Rc<RefCell<Node>>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            window: Weak::new(),
            parent: Weak::new(),
            first_child: None,
            last_child: Weak::new(),
            previous_sibling: Weak::new(),
            next_sibling: None,
        }
    }

    pub fn set_window(&mut self, window: Weak<RefCell<Window>>) {
        self.window = window;
    }

    pub fn set_parent(&mut self, parent: Weak<RefCell<Node>>) {
        self.parent = parent
    }

    pub fn parent(&self) -> Weak<RefCell<Node>> {
        self.parent.clone()
    }

    pub fn set_first_child(&mut self, first_child: Option<Rc<RefCell<Node>>>) {
        self.first_child = first_child;
    }

    pub fn first_child(&self) -> Option<Rc<RefCell<Node>>> {
        self.first_child.clone()
    }

    pub fn set_last_child(&mut self, last_child: Weak<RefCell<Node>>) {
        self.last_child = last_child;
    }

    pub fn last_child(&self) -> Weak<RefCell<Node>> {
        self.last_child.clone()
    }

    pub fn set_previous_sibling(&mut self, previous_sibling: Weak<RefCell<Node>>) {
        self.previous_sibling = previous_sibling;
    }

    pub fn previous_sibling(&self) -> Weak<RefCell<Node>> {
        self.previous_sibling.clone()
    }

    pub fn set_next_sibling(&mut self, next_sibling: Option<Rc<RefCell<Node>>>) {
        self.next_sibling = next_sibling;
    }

    pub fn next_sibling(&self) -> Option<Rc<RefCell<Node>>> {
        self.next_sibling.clone()
    }

    pub fn kind(&self) -> NodeKind {
        self.kind.clone()
    }

    pub fn get_element(&self) -> Option<Element> {
        match self.kind {
            NodeKind::Document | NodeKind::Text(_) => None,
            NodeKind::Element(ref e) => Some(e.clone()),
        }
    }

    pub fn element_kind(&self) -> Option<ElementKind> {
        match self.kind {
            NodeKind::Document | NodeKind::Text(_) => None,
            NodeKind::Element(ref e) => Some(e.kind.clone()),
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub enum NodeKind {
    /// https://dom.spec.whatwg.org/#interface-document
    Document,

    /// https://dom.spec.whatwg.org/#interface-element
    Element(Element),

    /// https://dom.spec.whatwg.org/#interface-text
    Text(String),
}

impl PartialEq for NodeKind {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            NodeKind::Document => matches!(other, NodeKind::Document),
            NodeKind::Element(e1) => match &other {
                NodeKind::Element(e2) => e1.kind == e2.kind(),
                _ => false,
            },
            NodeKind::Text(_) => matches!(other, NodeKind::Text(_)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    document: Rc<RefCell<Node>>,
}

impl Window {
    pub fn new() -> Self {
        let window = Self {
            document: Rc::new(RefCell::new(Node::new(NodeKind::Document))),
        };

        window
            .document
            .borrow_mut()
            .set_window(Rc::downgrade(&Rc::new(RefCell::new(window.clone()))));

        window
    }

    pub fn document(&self) -> Rc<RefCell<Node>> {
        self.document.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    kind: ElementKind,
    attributes: Vec<Attribute>,
}

impl Element {
    pub fn new(element_name: &str, attributes: Vec<Attribute>) -> Self {
        Self {
            kind: ElementKind::from_str(element_name).expect("failed to converty string to ElementKind"),
            attributes,
        }
    }

    pub fn kind(&self) -> ElementKind {
        self.kind
    }

    pub fn attributes(&self) -> Vec<Attribute> {
        self.attributes.clone()
    }

    pub fn get_attribute(&self, name: &str) -> Option<String> {
        for attr in &self.attributes {
            if attr.name() == name {
                return Some(attr.value());
            }
        }
        None
    }

    pub fn is_block_element(&self) -> bool {
        match self.kind {
            ElementKind::Body | ElementKind::H1 | ElementKind::H2 | ElementKind::P => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    /// https://html.spec.whatwg.org/multipage/semantics.html#the-html-element
    Html,

    /// https://html.spec.whatwg.org/multipage/semantics.html#the-head-element
    Head,

    /// https://html.spec.whatwg.org/multipage/semantics.html#the-style-element
    Style,

    /// https://html.spec.whatwg.org/multipage/semantics.html#the-script-element
    Script,

    /// https://html.spec.whatwg.org/multipage/semantics.html#the-body-element
    Body,

    /// https://html.spec.whatwg.org/multipage/grouping-content.html#the-p-element
    P,

    /// https://html.spec.whatwg.org/multipage/grouping-content.html#the-h1-element
    H1,

    /// https://html.spec.whatwg.org/multipage/grouping-content.html#the-h2-element
    H2,

    /// https://html.spec.whatwg.org/multipage/semantics.html#the-a-element
    A,
}

impl FromStr for ElementKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "html" => Ok(Self::Html),
            "head" => Ok(Self::Head),
            "style" => Ok(Self::Style),
            "script" => Ok(Self::Script),
            "body" => Ok(Self::Body),
            "p" => Ok(Self::P),
            "h1" => Ok(Self::H1),
            "h2" => Ok(Self::H2),
            "a" => Ok(Self::A),
            _ => Err(format!("unimplemented element kind: {:?}", s)),
        }
    }
}

impl Display for ElementKind {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let s = match self {
            ElementKind::Html => "html",
            ElementKind::Head => "head",
            ElementKind::Style => "style",
            ElementKind::Script => "script",
            ElementKind::Body => "body",
            ElementKind::H1 => "h1",
            ElementKind::H2 => "h2",
            ElementKind::P => "p",
            ElementKind::A => "a",
        };

        write!(f, "{}", s)
    }
}
