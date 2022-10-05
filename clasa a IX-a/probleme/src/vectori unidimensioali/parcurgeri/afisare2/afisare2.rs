/*
Problema Afisare vector 2
Cerință

Se dă un șir de n numere naturale. Să se afișeze numerele din vector în ordinea: primul, ultimul, al doilea, penultimul și așa mai departe.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elemente ale vectorului, iar de pe a doua linie, n numere naturale, reprezentând elementele vectorului.
Date de ieșire

Programul afișează pe ecran elementele din vector în ordinea specificată: primul, ultimul, al doilea, penultimul și așa mai departe.
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele vectorului ≤ 2.000.000.000

Exemplu

6
2 3 1 4 6 9

2 9 3 6 1 4

Explicația exemplului

Se afișează primul element (2), după care ultimul (9), după care al doilea (3), după care penultimul (6), urmat de al treilea (1) și, în final, antepenultimul (4).
*/
pub fn afisare2() {
    let arr = [2, 3, 1, 4, 6, 9, 5];
    let mut i = 0;
    let mut j = arr.len() - 1;

    while (i <= j) {
        print!("{} ", arr[i]);

        if (i != j) {
            print!("{} ", arr[j]);
        }
        i += 1;
        j -= 1;
    }
}