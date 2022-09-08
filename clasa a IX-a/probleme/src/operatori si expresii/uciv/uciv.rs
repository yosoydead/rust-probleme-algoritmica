/*
Calculați ultima cifră a sumei a două numere naturale.
ex: 25 + 78 = 103 => 3
*/
pub fn uciv(a: i32, b:i32) {
    let sum = a + b;
    let digit = sum % 10;

    println!("Digit: {}", digit);
}