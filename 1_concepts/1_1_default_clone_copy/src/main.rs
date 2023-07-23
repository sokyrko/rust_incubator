fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_copy() {
        let a = 1;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_clone() {
        let a = String::from("Hello");
        let b = a.clone();
        assert_eq!(a, b); // change here
    }
}
