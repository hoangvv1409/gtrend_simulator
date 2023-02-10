use std::error::Error;
use std::fs;

pub fn read_file(filename: String) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let data: Vec<Vec<i32>> = serde_json::from_str(&contents)?;

    Ok(data)
}

pub fn run(data: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();

    for fetch_num in 0..data.len() {
        let item = &data[fetch_num];
        let minute_offset = 60 * fetch_num;

        for index in 0..item.len() {
            let minute = minute_offset + index;
            let latest_minute = result.len();
            let new_value = item[index];

            if new_value < 0 {
                panic!("Value must be an integer");
            }

            if minute >= latest_minute {
                result.push(new_value);
                continue;
            }

            let first_value = result[minute];
            let is_diff = first_value != new_value;

            if is_diff && first_value == 0 {
                eprintln!(
                    "error, need correct | first: 0 vs new: {} - at minute: {}",
                    new_value, minute
                );
                result[minute] = new_value;
            }
        }
    }
    result
}
