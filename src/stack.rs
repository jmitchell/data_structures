extern crate quickcheck;

use std::{clone, fmt};

/// A growable stack type.
pub struct Stack<T> {
    data : Vec<T>
}

impl<T> Stack<T> {
    /// Constructs a new, empty `Stack<T>`.
    pub fn new() -> Stack<T> {
        Stack { data: vec![] }
    }

    /// Pushes an element onto the stack.
    pub fn push(&mut self, value: T) {
        self.data.push(value)
    }

    /// Pops an element off of the stack.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

/// Macro constructs a stack based on comma-separated
/// arguments. Rightmost argument is the "top" of the stack.
macro_rules! stack {
    ( $( $x:expr ),* ) => {{
        let mut _stack = Stack::new();
        $( _stack.push($x); )*
        _stack
    }};
}

impl<T> PartialEq for Stack<T> where T: PartialEq {
    fn eq(&self, other: &Stack<T>) -> bool {
        self.data == other.data
    }
}

impl<T> fmt::Debug for Stack<T> where T: fmt::Debug {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.debug_struct("Stack")
            .field("data", &self.data)
            .finish()
    }
}

impl<T> clone::Clone for Stack<T> where T: clone::Clone {
    fn clone(&self) -> Stack<T> {
        Stack { data: self.data.clone() }
    }
}

impl<T> quickcheck::Arbitrary for Stack<T> where T: quickcheck::Arbitrary {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Stack<T> {
        let mut size: u16 = quickcheck::Arbitrary::arbitrary(g);
        let mut stack = stack![];
        while size > 0 {
            stack.push(quickcheck::Arbitrary::arbitrary(g));
            size -= 1;
        }
        stack
    }

    fn shrink(&self) -> Box<Iterator<Item=Stack<T>> + 'static> {
        // TODO: make a proper shrinker for stacks
        quickcheck::empty_shrinker()
    }
}

/// Produces the reversed form of a stack. Top element of the input
/// becomes the bottom of output.
pub fn reverse<T: Clone>(ss: &Stack<T>) -> Stack<T> {
    let mut copy = ss.clone();
    let mut rev = Stack::new();
    loop {
        match copy.pop() {
            None => return rev,
            Some(elem) => rev.push(elem.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;
    use super::reverse;

    #[test]
    fn pop_empty_stack_is_none() {
        let mut ss: Stack<isize> = stack![];

        ss.pop() == None;
    }

    #[test]
    fn rightmost_value_of_stack_macro_is_on_top() {
        let mut ss = stack![1,2,3];

        ss.pop().unwrap() == 3;
    }

    #[quickcheck]
    fn push_then_pop_is_identity(ss: Stack<isize>, x: isize) -> bool {
        let mut copy = ss.clone();
        copy.push(x);

        copy.pop().unwrap() == x
    }

    #[quickcheck]
    fn reverse_of_reverse_is_identity(ss: Stack<isize>) -> bool {
        ss == reverse(&reverse(&ss))
    }
}
