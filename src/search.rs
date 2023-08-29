use std::cmp::{PartialEq, PartialOrd};

pub mod search {
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
}
