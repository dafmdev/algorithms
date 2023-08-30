fn find_minor<T: PartialOrd>(lst: &Vec<T>) -> usize {
    let mut minor = &lst[0];
    let mut lower_index = 0;

    for i in 1..lst.len() {
        if lst[i] < *minor {
            minor = &lst[i];
            lower_index = i
        }
    }

    lower_index
}

pub fn selection_sort<T: PartialOrd>(mut lst: Vec<T>) -> Vec<T> {
    let mut new_lst = Vec::new();

    for _ in 0..lst.len() {
        let minor = find_minor(&lst);
        new_lst.insert(new_lst.len(), lst.remove(minor));
    }

    new_lst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_unsigned_list() {
        let lst: Vec<u32> = vec![5, 3, 6, 2, 10];

        assert_eq!(vec![2, 3, 5, 6, 10], selection_sort(lst))
    }

    #[test]
    fn sort_signed_list() {
        let lst: Vec<i32> = vec![1, -1, 0, 100, 25, -30];

        assert_eq!(vec![-30, -1, 0, 1, 25, 100], selection_sort(lst))
    }
}
