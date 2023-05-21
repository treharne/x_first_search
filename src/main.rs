use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

static ITEMS: [Item; 6] = [
    Item { name: "Mixed Fruit", val: 2.15 },
    Item { name: "French Fries", val: 2.75 },
    Item { name: "Side Salad", val: 3.35 },
    Item { name: "Hot Wings", val: 3.55 },
    Item { name: "Mozzarella Sticks", val: 4.20 },
    Item { name: "Sampler Plate", val: 5.80 },
];


struct Tree {
    nodes: RefCell<Vec<Node>>
}

impl Tree {
    fn new() -> Self {
        let root_node = Node { item: None, total: 0.0, parent: None };
        Tree { nodes: RefCell::new(vec![root_node]) }
    }

    fn get_children(&self, parent_idx: NodeIdx) -> Vec<NodeIdx> {

        let next_idx = {
            let nodes = self.nodes.borrow();
            nodes.len()
        };

        let children: Vec<Node> = {
            let nodes = self.nodes.borrow();
            let parent_node = &nodes[parent_idx];
            ITEMS
                .iter()
                .map(|item| Node { 
                    item: Some(item), 
                    total: parent_node.total + item.val, 
                    parent: Some(parent_idx) 
                }).collect()
        };
        let child_indexes = next_idx..(next_idx + &children.len());
        let mut nodes = self.nodes.borrow_mut();
        nodes.extend(children);
        child_indexes.into_iter().collect()
    }

    fn visit(&self, idx: NodeIdx, target: f32) -> Option<Vec<&Item>> {
        let nodes = self.nodes.borrow();
        let mut node = &nodes[idx];
        
        if node.total != target { return None };
        
        let mut items = Vec::new();
        while let Some(item) = node.item {
            items.push(item);
            node = &nodes[node.parent?];
        };

        Some(items)
    }

    fn search<C>(&mut self, target: f32, mut container: C) -> Option<Vec<&Item>> 
    where C: Container<NodeIdx> {
        container.put(0);
        while let Some(node_idx) = container.get() {
            if let Some(result) = self.visit(node_idx, target) {
                return Some(result);
            } 
            let children = self.get_children(node_idx);
            container.extend(children);
        };
        None
    }
}

#[derive(Debug)]
struct Item {
    name: &'static str,
    val: f32,
}

type NodeIdx = usize;

struct Node {
    item: Option<&'static Item>,
    total: f32,
    parent: Option<NodeIdx>,
}

trait Container<T>: Extend<T> {
    // a generic type for a FIFO Queue or LIFO Stack
    // so that we can make the search function 
    // do either BFS or DFS
    fn put(&mut self, val: T) -> ();
    fn get(&mut self) -> Option<T>;
}

type Stack<T> = Vec<T>;
type Queue<T> = VecDeque<T>;

impl <T>Container<T> for Stack<T> {
    fn put(&mut self, val: T) -> () {
        self.push(val);
    }
    fn get(&mut self) -> Option<T> {
        self.pop()
    }
}

impl <T>Container<T> for Queue<T> {
    fn put(&mut self, val: T) -> () {
        self.push_back(val);
    }
    fn get(&mut self) -> Option<T> {
        self.pop_front()
    }
}


fn main() {

    let mut tree = Tree::new();
    let val = tree.search(15.05, Queue::new());
    println!("{:?}", val);

}
