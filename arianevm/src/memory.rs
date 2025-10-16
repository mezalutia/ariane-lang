pub struct Stack<T>
where
    T: Clone,
{
    data: Vec<T>,
}

impl<T> Default for Stack<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    #[inline(always)]
    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    #[inline(always)]
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    #[inline(always)]
    pub fn set(&mut self, index: usize, val: T)  {
        if index >= self.data.len() {
            panic!("Attempted to index out of bounds");
        } else {
            self.data.insert(index, val);
        }
    }
}

enum Constant {
    Int(u64),
}

pub struct ConstantPool {
    constants: Vec<Constant>,
}
