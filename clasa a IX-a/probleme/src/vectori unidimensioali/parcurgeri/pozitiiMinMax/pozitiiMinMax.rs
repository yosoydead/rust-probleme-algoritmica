/*
Problema Pozitii minim si maxim
Cerință

Se dă un șir de n numere naturale. Să se determine minimul și maximul șirului, precum și pozițiile în care se află acestea.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elemente ale șirului, iar de pe a doua linie, n numere naturale, reprezentând elementele șirului.
Date de ieșire

Programul afișează pe ecran, pe prima linie, minimul din șir, urmat de toate pozițiile în care acesta se găsește în șir, iar pe a doua linie, maximul șirului, urmat de toate pozițiile sale din șir. Valorile aflate pe aceeași linie a șirului sunt separate printr-un spațiu.
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele vectorului ≤ 2.000.000.000
    Elementele șirului sunt numerotate începând cu 1.
    Pozițiile afișate pe o linie vor fi în ordine crescătoare

Exemplu

6
2 3 1 4 1 9

1 3 5
9 6

Explicația exemplului

Cel mai mic număr din șir este 1. Acesta apare de două ori în șir, o dată pe poziția 3 și încă o dată pe poziția 5.
Cel mai mare număr din șir este 9, care apare doar o dată, pe poziția 6.
*/
pub fn pozitiiMinMax() {
    let arr = [2, 3, 1, 4, 1, 9];
    let res = findSmallestAndBiggest(&arr);

    print!("{} ", res.0);
    for i in 0..arr.len() {

        if (arr[i] == res.0) {
            print!("{} ", i+1);
        }
    }

    println!("");

    print!("{} ", res.1);
    for i in 0..arr.len() {

        if (arr[i] == res.1) {
            print!("{} ", i+1);
        }
    }
}

fn findSmallestAndBiggest(arr: &[i32]) -> (i32, i32) {
    let mut min = std::i32::MAX;
    let mut max = std::i32::MIN;

    let mut i = 0;
    let mut j = arr.len() - 1;

    while (i <= j) {
        if (arr[i] < min) {
            min = arr[i];
        }
        if (arr[i] > max) {
            max = arr[i];
        }

        if (arr[j] > max) {
            max = arr[j];
        }
        if (arr[j] < min) {
            min = arr[j];
        }

        i += 1;
        j -= 1;
    }

    return (min, max);
}