
fn main() {
    let mut arr_i32 = [9, 6, 18, 12, 3, 2, 4, 7, 1];
    sort(&mut arr_i32);
    println!("Sorted: {:?}", arr_i32);

    let mut arr_str = ["this", "is", "substrate", "lesson","and","rust"];
    sort(&mut arr_str);
    println!("Sorted: {:?}", arr_str);
}
fn sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

