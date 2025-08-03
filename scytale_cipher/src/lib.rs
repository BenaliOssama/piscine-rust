pub fn scytale_cipher(message: &str, size: usize) -> String {
    let chars: Vec<char> = message.chars().collect();
    let length = chars.len();

    let columns = (length + size - 1) / size; 

    let mut grid = vec![vec![' '; columns]; size];

    let mut index = 0;
    for col in 0..columns {
        for row in 0..size {
            if index < length {
                grid[row][col] = chars[index];
                index += 1;
            }
        }
    }

    let mut result = String::new();
    for row in 0..size {
        for col in 0..columns {
            result.push(grid[row][col]);
        }
    }

    return result;
}

