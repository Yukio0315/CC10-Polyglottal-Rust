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
