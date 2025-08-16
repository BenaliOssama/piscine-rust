pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut result = Vec::new();

    for i in 0..arr.len() {
        let mut prod = 1;
        for j in 0..arr.len() {
            if i != j {
                prod *= arr[j];
            }
        }
        result.push(prod);
    }

    result
}
