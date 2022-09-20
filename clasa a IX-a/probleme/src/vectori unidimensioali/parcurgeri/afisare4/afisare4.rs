/*
Problema Afisare vector 4
Cerință

Se dă un șir de n numere naturale. Să se afișeze elementele șirului care se află între primul și ultimul număr impar al șirului, inclusiv.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elemente ale șirului, iar de pe a doua linie, n numere naturale, reprezentând elementele șirului.
Date de ieșire

Programul afișează pe ecran elementele șirului care se află între primul și ultimul element impar al șirului, inclusiv.
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele vectorului ≤ 2.000.000.000
    Se garantează că există cel puțin un element impar în șir.

Exemplu

6
2 3 1 4 6 9

3 1 4 6 9

Explicația exemplului

Primul număr impar din șir este 3, iar ultimul este 9. Astfel, se afișează toate numerele care se află între 3 și 9 în șirul dat: 3, 1, 4, 6, 9.
*/
pub fn afisare4() {
    let arr = [2, 3, 1, 4, 6, 9];
    // findLastOdd(&arr);
    let mut i = findFirstOdd(&arr);
    let j = findLastOdd(&arr);

    while i <= j {
        println!("{}", arr[i]);
        i += 1;
    }
}

fn findFirstOdd(arr: &[i32]) -> usize {
    let mut result: usize = 0;

    for i in 0..arr.len() {
        if (arr[i] % 2 != 0) {
            result = i;
            return result;
        }
    }

    return result;
}

fn findLastOdd(arr: &[i32]) -> usize {
    let mut result = 0;
    let j = arr.len();

    for i in (0..j).rev() {
        if (arr[i] % 2 != 0) {
            result = i;
            return result;
        }
    }

    return result;
}