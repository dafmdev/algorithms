pub fn recursive_quick_sort<T: PartialOrd + Clone + Copy>(lst: Vec<T>) -> Vec<T> {
    if lst.len() < 2 {
        return lst;
    } else {
        let pivot = lst[0].clone();
        let minors: Vec<T> = lst[1..].iter().copied().filter(|x| x <= &pivot).collect();
        let greater: Vec<T> = lst[1..].iter().copied().filter(|x| x > &pivot).collect();

        return [recursive_quick_sort(minors), vec![pivot], recursive_quick_sort(greater)].concat();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let sorted = recursive_quick_sort(res);

        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], sorted);
    }
}