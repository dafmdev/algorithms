mod search;

pub use search::search::binary_search;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_binary_search() {
        let vector: Vec<usize> = vec![2, 4, 6, 8];

        let result = binary_search(vector, 8);
        assert_eq!(result, Some(3));
    }
}
