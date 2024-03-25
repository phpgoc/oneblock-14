pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn bubble_sort_withcompare<T, F>(arr: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> bool,
{
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if compare(&arr[j], &arr[j + 1]) {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4]);
    }

    #[test]
    fn test_bubble_sort_withcompare() {
        let mut arr = [1,2,3,4];
        bubble_sort_withcompare(&mut arr, |a, b| a < b);
        assert_eq!(arr, [4,3,2,1]);
        print!("Bubble sort with compare: {:?}", arr)
    }

}
