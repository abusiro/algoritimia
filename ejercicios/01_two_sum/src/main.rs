// solution brute force
pub fn two_sum_brute(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
            }
            return vec![nums[i] as i32, nums[j] as i32];
        }
    }
    vec![]
}
// it only works if nums is sorted
pub fn two_sum_two_pointers(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut left = 0;   
    let mut right = nums.len() - 1;
    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return vec![nums[left] as i32, nums[right] as i32];
        } else if sum < target {
            left += 1
        } else if sum > target {
            right -= 1
        }
    }

    vec![]
}

fn main() {
    // Ejemplos de prueba
    let test_cases = vec![
        (vec![3, 7, 11, 15], 9, vec![0, 1]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
        (vec![1, 5, 3, 7, 2], 10, vec![2, 3]),
    ];

    for (nums, target, expected) in test_cases.clone() {
        let result = two_sum_brute(nums.clone(), target);
        println!("nums: {:?}, target: {}", nums, target);
        println!("  Esperado: {:?}", expected);
        println!("  Resultado: {:?}", result);
        println!("  ✓ Correcto: {}\n", result == expected);
    }

    for (nums, target, expected) in test_cases.clone() {
        let result = two_sum_two_pointers(nums.clone(), target);
        println!("nums: {:?}, target: {}", nums, target);
        println!("  Esperado: {:?}", expected);
        println!("  Resultado: {:?}", result);
        println!("  ✓ Correcto: {}\n", result == expected);
    }
}