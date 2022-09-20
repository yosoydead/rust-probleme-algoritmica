/*
Problema Determinare vector
Cerință

Se dă un șir de n numere naturale. Să se afișeze diferența în valoare absolută dintre suma numerelor aflate la stânga celui mai mic număr din șir și suma numerelor aflate la dreapta acestuia.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elemente ale șirului, iar de pe a doua linie, n numere naturale, reprezentând elementele șirului.
Date de ieșire

Programul afișează pe ecran diferența în valoare absolută dintre suma numerelor aflate la stânga celui mai mic număr din șir și suma numerelor aflate la dreapta acestuia.
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele vectorului ≤ 200.000
    Garantăm că cel mai mic element nu se va afla la unul dintre capetele vectorului.

Exemplu

6
2 3 1 4 6 9

14

Explicația exemplului

Cel mai mic număr din șir este 1. În stânga sa se află numerele 2 și 3, care adunate dau 2 + 3 = 5, iar în dreapta sa se află numerele 4, 6 și 9, care adunate dau 4 + 6 + 9 = 19. Diferența în valoare absolută a celor două sume este 14.
*/
pub fn determinare() {
    let arr = [2, 3, 1, 4, 6, 9];
    let index = smallest(&arr);

    let mut s1 = 0;
    let mut s2 = 0;

    let mut i = 0;
    while (i < index) {
        s1 += arr[i];
        i += 1;
    }

    let mut j = index + 1;
    while (j < arr.len()) {
        s2 += arr[j];
        j += 1;
    }

    println!("{}", (s1-s2).abs());
}

fn smallest(arr: &[i32]) -> usize {
    let mut result = 0;
    let mut biggest = std::i32::MAX;

    for i in 0..arr.len() {
        if (arr[i] < biggest) {
            biggest = arr[i];
            result = i;
        }
    }

    return result;
}