/*
Problema Cautare binara
Cerință

Se dă un șir a cu n numere naturale, ordonate crescător, și alte m numere. Pentru fiecare dintre cele m numere, să se afle dacă se află sau nu în șirul a.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul n, de pe a doua linie cele n numere naturale ale șirului a, de pe a treia linie, numărul m, iar de pe a patra linie cele m numere.
Date de ieșire

Programul afișează pe ecran m valori de 0 sau de 1, separate prin câte un spațiu. Dacă al i-lea număr dintre cele m se află în șirul a, atunci a i-a valoare este 1, respectiv 0 în caz contrar.
Restricții și precizări

    1 ≤ n, m ≤ 100.000
    1 ≤ numerele șirului a, și cele m numere ≤ 2.000.000.000

Exemplu

6
2 4 7 8 8 9
5
1 2 8 10 9

0 1 1 0 1

Explicația exemplului

Numărul 1 nu se află în șirul a.
Numărul 2 se află în șirul a, pe poziția 1.
Numărul 8 se află în șirul a, pe pozițiile 4 și 5.
Numărul 10 nu se află în șirul a.
Numărul 9 se află în șirul a, pe poziția 6.
*/
pub fn cautareBinara() {
    let arr = [2, 4, 7, 8, 8, 9];
    let nums = [1, 2, 8, 10, 9];

    for i in 0..nums.len() {
        let r = findNum(&arr, nums[i]);
        let b = if (r == true) { "1" } else { "0" };

        print!("{} ", b);
    }
}

fn findNum(arr: &[i32], n: i32) -> bool {
    let mut l = 0;
    let mut r = arr.len();

    while (l < r) {
        let m = ((l + r) as f32 / 2.0).floor() as usize;

        if (arr[m] < n) {
            l += 1;
        } else if (arr[m] > n) {
            r -= 1;
        } else {
            return true;
        }
    }

    return false;
}