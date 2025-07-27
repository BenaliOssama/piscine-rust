pub fn bubble_sort(arr: &mut [i32]) {
    for j in 0..arr.len() {
        for i in j..arr.len() {
            if arr[i] < arr[j] {
                arr.swap(i,j);
            }
        }
    }
}
