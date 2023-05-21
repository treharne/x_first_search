use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Item {
    name: String,
    val: f32,
}

struct Node {
    item: Option<&'static Item>,
    total: f32,
    // parent: Option<Box<&'a Node>>,
    parent: Option<Weak<RefCell<Node>>>,
}

trait Container {
    // a generic type for a FIFO Queue or LIFO Stack
    // so that we can make the search function 
    // do either BFS or DFS
    fn put(&mut self, node: NodeRef) -> ();
    fn get(&mut self) -> Option<NodeRef>;
}

type Stack = Vec<NodeRef>;
type Queue = VecDeque<NodeRef>;

impl Container for Stack {
    fn put(&mut self, node: NodeRef) -> () {
        self.push(node);
    }
    fn get(&mut self) -> Option<NodeRef> {
        self.pop()
    }
}

impl Container for Queue {
    fn put(&mut self, node: NodeRef) -> () {
        self.push_back(node);
    }
    fn get(&mut self) -> Option<NodeRef> {
        self.pop_front()
    }
}

type NodeVisitor<T> = dyn Fn(NodeRef) -> Option<T>;
type NodeChildGetter = dyn Fn(NodeRef) -> Vec<Node>;

fn search<T>(
            start_node: NodeRef, 
            visit: &NodeVisitor<T>, 
            get_children: &NodeChildGetter, 
            mut container: impl Container
        ) -> Option<T> {
    container.put(start_node);
    while let Some(node) = container.get() {

        if let Some(result) = visit(node) {
            return Some(result);
        }

        for child in get_children(node) {
            container.put(node_ref(child));
        }
    }
    None
}

type NodeRef = Weak<RefCell<Node>>;

fn node_ref(node: Node) -> NodeRef {
    let rc = Rc::new(RefCell::new(node));
    Rc::downgrade(&rc)
}

fn main() {

    let menu = vec![
        Item { name: "Mixed Fruit".to_string(), val: 2.15 },
        Item { name: "French Fries".to_string(), val: 2.75 },
        Item { name: "Side Salad".to_string(), val: 3.35 },
        Item { name: "Hot Wings".to_string(), val: 3.55 },
        Item { name: "Mozzarella Sticks".to_string(), val: 4.20 },
        Item { name: "Sampler Plate".to_string(), val: 5.80 },
    ];

    let get_children = |parent: NodeRef| -> Vec<Node> { 
        menu.iter()
        .map(|item| Node { 
            item: Some(item), 
            total: parent.borrow().total + item.val, 
            parent: Some(parent.clone()) 
        })
        .collect()
    };

    let target = 15.05;
    let visit = move |node: NodeRef| -> Option<Vec<&Item>> {
        if node.borrow().total != target {
            return None
        };

        let mut items = Vec::new();
        // let mut node = &node;
        // let 
        let mut node = node.clone();
        while let Some(parent) = node.clone().borrow().parent {
            if let Some(item) = parent.item {
                items.push(item);
            }
            // node = &parent.clone();
            node = parent.clone();
        };
        return Some(items);
    };

    let root = Node { item: None, total: 0.0, parent: None };

    search(
        node_ref(root), 
        &visit, 
        &get_children, 
        Stack::new()
    );


    
}
