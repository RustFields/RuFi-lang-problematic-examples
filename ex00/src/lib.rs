#[cfg(test)]
mod tests {

    #[test]
    fn test_multiple_borrows() {
        let mut vector = vec![1, 2, 3];
        let first = &vector[0]; // Immutable borrow

        vector.push(4); // Error: Cannot borrow `vector` as mutable because it is also borrowed as immutable

        println!("The first element is: {}", first);
    }

    #[test]
    fn test_with_blocks() {
        let mut vector = vec![1, 2, 3];
        //We need to make sure the first borrow is dropped before the second one starts
        {
            let first = &vector[0]; // Immutable borrow
            println!("The first element is: {}", first)
        }

        vector.push(4);
        println!("The vector is: {:?}", vector);
    }
}