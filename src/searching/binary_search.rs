pub fn binary_search<T: PartialEq + PartialOrd>(lst: Vec<T>, item: T) -> Option<usize> {
    let mut low = 0;
    let mut high = lst.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let estimated_item = &lst[mid];

        if *estimated_item == item {
            return Some(mid);
        } else if *estimated_item > item {
            high = mid - 1
        } else {
            low = mid + 1
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_number_near_end_of_list() {
        let num_slice = vec![
            2, 4, 5, 12, 15, 30, 32, 33, 34, 40, 45, 51, 55, 57, 60, 66, 70, 71, 90, 99, 100,
        ];

        assert_eq!(binary_search(num_slice, 70), Some(16));
    }

    #[test]
    fn finds_number_near_start_of_list() {
        let num_slice = vec![
            2, 4, 5, 12, 15, 30, 32, 33, 34, 40, 45, 51, 55, 57, 60, 66, 70, 71, 90, 99, 100,
        ];

        assert_eq!(binary_search(num_slice, 5), Some(2));
    }

    #[test]
    fn finds_char() {
        let char_slice = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        assert_eq!(binary_search(char_slice, 'l'), Some(11));
    }

    #[test]
    fn returns_none_for_chars() {
        let char_slice = vec!['a', 'b', 'c'];

        assert_eq!(binary_search(char_slice, 'l'), None);
    }
}
