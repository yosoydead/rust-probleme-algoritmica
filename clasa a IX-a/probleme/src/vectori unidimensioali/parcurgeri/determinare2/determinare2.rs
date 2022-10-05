/*
Problema Determinare vector 2
Cerință

Se dă un șir de n numere naturale. Să se determine numărul de elemente ale șirului care sunt strict mai mici decât media aritmetică a tuturor elementelor șirului.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elemente ale șirului, iar de pe a doua linie, n numere naturale, reprezentând elementele șirului.
Date de ieșire

Programul afișează pe ecran, un singur număr, reprezentând numărul elementelor mai mici decât media aritmetică a elementelor șirului.
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele vectorului ≤ 2.000.000.000

Exemplu

6
2 3 1 4 6 9

4

Explicația exemplului

Media aritmetică a elementelor șirului este egală cu (2 + 3 + 1 + 4 + 6 + 9) / 6 = 25 / 6 = 4.1(6). Sunt 4 elemente mai mici decât 4.1(6) în șir: 2, 3, 1, 4.
*/
pub fn determinare2() {
    let arr = [2, 3, 1, 4, 6, 9];
    let medie: f32 = sum(&arr) / arr.len() as f32;

    let mut res = 0;

    for i in 0..arr.len() {
        if ((arr[i] as f32) < medie) {
            res += 1;
        }
    }

    println!("am gasit {} nr mai mici decat media {}", res, medie);
}

fn sum(arr: &[i32]) -> f32 {
    let mut result = 0.0;

    for i in 0..arr.len() {
        result += arr[i] as f32;
    }

    return result;
}