/*****************************************
 * File Name: list.rs
 * Date: 11/8/24
 * File Description: Linked List module for
 *                   Rust version of Movies
 * Author(s): Ivan Wong
 *****************************************/


use std::fmt::Display;


/**************
 * Node Struct 
 **************/
pub struct Node<T> {
    pub val: T,
    pub next: Option<Box<Node<T>>>,
}


/*********************
 * Linked List Struct
 *********************/
pub struct List<T> {
    pub head: Option<Box<Node<T>>>,
    pub length: usize,
}


/***********************
 * Node Implementations
 ***********************/
impl<T> Node<T> {
    pub fn new(val: T) -> Node<T> {
        // Init node
        Node {
            val,
            next: None,
        }
    }
}


/***********************
 * List Implementations
 ***********************/
impl<T: Display> List<T> {
    pub fn new() -> Self {
        // Init list
        List {
            head: None,
            length: 0,
        }
    }

    pub fn add_list(&mut self, val: T) {
        // Create new node, link to front of list
        let mut new_node: Box<Node<T>> = Box::new(Node::new(val));

        // No nodes in list, head ptr gets new node
        if self.length == 0 {
            new_node.next = None;
            self.head = Some(new_node);

        }
        // Head exists, add to front of list
        else {
            new_node.next = self.head.take();
            self.head = Some(new_node);

        }
        self.length += 1;
    }

    pub fn len(&self) -> usize {
        // Length getter
        self.length
    }
}
