use std::collections::VecDeque;
use std::cell::RefCell;

static ITEMS: [Item; 6] = [
    Item { name: "Mixed Fruit", val: 215 },
    Item { name: "French Fries", val: 275 },
    Item { name: "Side Salad", val: 335 },
    Item { name: "Hot Wings", val: 355 },
    Item { name: "Mozzarella Sticks", val: 420 },
    Item { name: "Sampler Plate", val: 580 },
];


type NodeIdx = usize;


trait Container<T>: Extend<T> {
    // a generic trait for a FIFO Queue or LIFO Stack
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


struct Tree {
    nodes: RefCell<Vec<Node>>
}


impl Tree {
    fn new() -> Self {
        let root_node = Node { item: None, total: 0, parent: None };
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

    fn visit(&self, idx: NodeIdx, target: Dollars) -> Option<Vec<&Item>> {
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

    fn search<C>(&mut self, target: Dollars, mut container: C) -> Option<Vec<&Item>> 
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

    fn bfs(&mut self, target: Dollars) -> Option<Vec<&Item>> {
        self.search(target, Queue::new())
    }

    fn dfs(&mut self, target: Dollars) -> Option<Vec<&Item>> {
        self.search(target, Stack::new())
    }
}


#[derive(Debug)]
struct Item {
    name: &'static str,
    val: Dollars,
}


struct Node {
    item: Option<&'static Item>,
    total: Dollars,
    parent: Option<NodeIdx>,
}


type Dollars = i32;
fn dollars(val: f32) -> Dollars {
    (val * 100.0).round() as Dollars
}

fn main() {
    let mut tree = Tree::new();
    // let target = 3*275 + 2*335 + 4*580 + 3*420 ;
    let target = dollars(42.35);
    // let target = 15.05;
    // let target = 1505;
    let val = tree.bfs(target);
    println!("{:?}", val);
}
