
// EJERCICIO: TWO SUM
// PROBLEMA (LeetCode #1 - Fácil)
//
// Dado un array de enteros nums y un entero target, devuelve los índices de los dos números que suman target.
//
// Suposiciones:
//
//     Existe exactamente una solución
//
//     No puedes usar el mismo elemento dos veces
//
//     Puedes devolver la respuesta en cualquier orden
//
// EJEMPLOS:
// Ejemplo 1:
// rust
//
// Input: nums = [2, 7, 11, 15], target = 9
// Output: [0, 1]
// Explicación: nums[0] + nums[1] = 2 + 7 = 9
//
// Ejemplo 2:
// rust
//
// Input: nums = [3, 2, 4], target = 6
// Output: [1, 2]
// Explicación: nums[1] + nums[2] = 2 + 4 = 6
//
// Ejemplo 3:
// rust
//
// Input: nums = [3, 3], target = 6
// Output: [0, 1]
//
pub fn two_sum_brute(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
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

    for (nums, target, expected) in test_cases {
        let result = two_sum_brute(nums.clone(), target);
        println!("nums: {:?}, target: {}", nums, target);
        println!("  Esperado: {:?}", expected);
        println!("  Resultado: {:?}", result);
        println!("  ✓ Correcto: {}\n", result == expected);
    }
}