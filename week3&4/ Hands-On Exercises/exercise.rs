fn filter_even_numbers(nums: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for &num in nums {
        if num % 2 == 0 {
            result.push(num);
        }
    }
    result
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let even_nums = filter_even_numbers(&nums);
    println!("{:?}", even_nums);
}
