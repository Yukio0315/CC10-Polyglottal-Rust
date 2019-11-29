pub fn sort(array: &mut [i32]) {
    let mut tmp = (0, 0);
    let mut i = 0;
    while i < array.len() {
        tmp = (i, array[i]);
        let mut j = i + 1;
        while j < array.len() {
            let (min_order, value) = tmp;
            if value > array[j] {
                tmp = (j, array[j]);
            }
            j += 1;
        }
        let (min_order, value) = tmp;
        array.swap(i, min_order);
        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sort_i32_array() {
        let mut x = [10, 9, 3, 2, 5, 0, 1];

        sort(&mut x);

        assert_eq!(x, [0, 1, 2, 3, 5, 9, 10]);
    }
}
