#[allow(dead_code)]
pub struct Stack<T>{
    vec: Vec::<T>
}

#[allow(dead_code)]
impl<T> Stack<T>{

    pub fn new() -> Stack<T>{ 
        Stack{
            vec: Vec::new()
        }
    }

    pub fn init(first: T) -> Stack<T> {
        let mut res = Stack::new();
        res.push(first);
        res
    }

    pub fn push(&mut self, x: T){
        self.vec.push(x);
    }

    pub fn pop(&mut self) -> Option<T>{
        self.vec.pop()
    }

    pub fn val(&self) -> Option<&T>{
        self.vec.last()
    }

    pub fn val_mut(&mut self) -> Option<&mut T>{
        self.vec.last_mut()
    }

    pub fn is_empty(&self) -> bool{
        self.vec.len() == 0
    }

    pub fn size(&self) -> usize{
        self.vec.len()
    }

    pub fn change_top(&mut self, new: T) {
        *self.val_mut().unwrap() = new;
    }
} 

use std::collections::VecDeque;

#[allow(dead_code)]
pub struct Queue<T> {
    vec: VecDeque<T>
}

#[allow(dead_code)]
impl<T> Queue<T> {

    pub fn new() -> Queue<T>{ 
        Queue{
            vec: VecDeque::new()
        }
    }

    pub fn init(first: T) -> Queue<T> {
        let mut res = Queue::new();
        res.inqueue(first);
        res
    }

    pub fn inqueue(&mut self, x: T){
        self.vec.push_back(x);
    }

    pub fn dequeue(&mut self) -> Option<T>{
        self.vec.pop_front()
    }

    pub fn val(&self) -> Option<&T>{
        if self.is_empty() {
            None
        }else{
            Some(&self.vec[0])
        }
    }

    pub fn val_mut(&mut self) -> Option<&mut T>{
        if self.is_empty() {
            None
        }else{
            Some(&mut self.vec[0])
        }
    }

    pub fn is_empty(&self) -> bool{
        self.vec.len() == 0
    }

    pub fn size(&self) -> usize{
        self.vec.len()
    }

}