use std::{cell::RefCell, rc::{Rc, Weak}};

// Node represents a node in a linked list.
#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<NextNode>, // allows mutable access to the next node while the Node itself is shared (via Rc).
}

// NextNode repesents the next pointer in the node.
#[derive(Debug)]
enum NextNode {
    None, // No next node (end of the structure).
    Strong(Rc<Node>), // Strong reference to the next node, which increases the 
                      // reference count and keeps the node alive.
    Weak(Weak<Node>), // Weak reference to the next node, which does not increase the reference
                      // count. This prevents reference cycles.
}

// Implement the Drop trait for Node to print a message when the node is dropped.
impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping node with value {}", self.value);
    }
}

fn main() {
    // Node 2 -> Node 1 -> None
    let tail = Rc::new(Node {
        value: 1,
        next: RefCell::new(NextNode::None), // The tail node has no next node.
    });
    let head = Rc::new(Node {
        value: 2,
        next: RefCell::new(NextNode::Strong(tail.clone())), // The head node has a strong reference
                                                            // to the tail node.
    });

    
    // The next pointer of tail is updated to create a weak reference to head.
    // Rc::downgrade converts a strong reference (Rc) into a weak reference (Weak), 
    //  which does not increase the reference count of head.
    // This forms a cycle: head -> tail -> head, but it's a safe cycle because Weak references are used.
    *tail.next.borrow_mut() = NextNode::Weak(Rc::downgrade(&head));

    println!("head: {head:?}");
}
