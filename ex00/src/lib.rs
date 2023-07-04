trait Executor {
    // A mutable reference to self is needed since execute may mutate self.
    fn execute<A>(&mut self, expr: impl Fn() -> A) -> A;
}

struct MyStruct { counter: i32 }

impl Executor for MyStruct {
    fn execute<A>(&mut self, expr: impl Fn() -> A) -> A {
        self.counter += 1;
        expr()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Executor, MyStruct};

    #[test]
    fn test_nested_borrow() {
        let mut my_struct = MyStruct { counter: 0 };
        // This call is legal since the closure doesn't capture my_struct.
        let res1 = my_struct.execute(|| 1);
        assert_eq!(res1, 1);
        // This call is not legal since the closure captures my_struct, causing a borrowing conflict.
        let res2 = my_struct.execute(|| my_struct.execute(|| 1));
        assert_eq!(res2, 1);
    }
}