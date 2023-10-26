pub struct BPlusTree<T: Ord> {  
    root: Option<Box<Node<T>>>,  
    order: usize,  
}  
  
#[derive(Debug)]  
struct Node<T: Ord> {  
    keys: Vec<T>,  
    children: Vec<Box<Node<T>>>,  
    is_leaf: bool,  
}  
  
impl<T: Ord> BPlusTree<T> {  
    pub fn new(order: usize) -> Self {  
        BPlusTree { root: None, order }  
    }  
  
    pub fn insert(&mut self, key: T) {  
        let mut cur_node = &mut self.root;  
        let mut idx = 0usize;  
        while cur_node.is_some() {  
            let node = cur_node.as_mut().unwrap();  
            idx = node.find_index(key);  
            if idx == node.keys.len() {  
                if node.is_leaf {  
                    node.keys.push(key);  
                    return;  
                } else {  
                    cur_node = &mut node.children[idx];  
                }  
            } else {  
                if node.keys[idx] > key {  
                    cur_node = &mut node.children[idx * 2];  
                } else {  
                    cur_node = &mut node.children[idx * 2 + 1];  
                }  
            }  
        }  
        if let Some(mut leaf) = cur_node.take() {  
            leaf.keys.push(key);  
        } else {  
            let mut new_node = Box::new(Node {  
                keys: Vec::new(),  
                children: Vec::new(),  
                is_leaf: true,  
            });  
            new_node.keys.push(key);  
            self.root = Some(new_node);  
        }  
    }  
  
    pub fn find(&self, key: T) -> Option<&T> {  
        let mut cur_node = &self.root;  
        while cur_node.is_some() {  
            let node = cur_node.as_ref().unwrap();  
            let idx = node.find_index(key);  
            if idx < node.keys.len() && node.keys[idx] == key {  
                return Some(&node.keys[idx])  
            } else if idx == node.keys.len() {  
                cur_node = &node.children[idx];  
            } else {  
                cur_node = &node.children[idx * 2 + 1];  
            }  
        }  
        None  
    }  
}
