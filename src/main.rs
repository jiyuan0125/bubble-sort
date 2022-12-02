/// Sort a given mutable slice using bubble sort algorithem.
/// # Examples
/// ```
/// let mut arr = [5, 4, 3, 2, 1];
/// bubble_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5]);
/// ```
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut arr = [5, 4, 3, 2, 1];
    println!("original i32 array: {:?}", arr);
    bubble_sort(&mut arr);
    println!("sorted i32 array: {:?}", arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);

    let mut arr = ["a01", "01a", "1a0", "0a1", "10a", "a10"];
    println!("original str array: {:?}", arr);
    bubble_sort(&mut arr);
    println!("sorted str array: {:?}", arr);
    assert_eq!(arr, ["01a", "0a1", "10a", "1a0", "a01", "a10"]);
}
