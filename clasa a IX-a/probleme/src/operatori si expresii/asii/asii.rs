/*
Se dau 2 numere naturale. Calculați suma, diferenţa,
    produsul şi câtul lor, în această ordine.
*/
pub fn asii(a: i32, b: i32) {
    let sum = a + b;
    let dif = a - b;
    let prod = a * b;
    let div = a / b;
    println!("Suma: {}. Dif: {}. Prod: {}. Div: {}.", sum, dif, prod, div);
}