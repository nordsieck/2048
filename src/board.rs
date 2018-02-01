// type Board = [[u32; 4]; 4];

fn foo() -> bool { true }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(foo(), true);
    }
}
