pub struct Stack<T>{
    vec: Vec::<T>
}

impl<T> Stack<T>{

    pub fn new() -> Stack<T>{ 
        Stack{
            vec: Vec::new()
        }
    }

    pub fn push(&mut self, x: T){
        self.vec.push(x);
    }

    pub fn pop(&mut self) -> T{
        self.vec.pop().unwrap()
    }

    pub fn val(&self) -> &T{
        self.vec.last().unwrap_or_else(||{
            panic!("The stack is empty.");
        })
    }

    pub fn is_empty(&self) -> bool{
        return self.vec.len() == 0;
    }

    pub fn size(&self) -> usize{
        self.vec.len()
    }
} 