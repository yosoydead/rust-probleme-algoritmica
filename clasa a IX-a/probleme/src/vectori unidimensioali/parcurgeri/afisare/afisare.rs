/*
roblema Afisare vector
Cerință

Se dă un șir de n numere naturale. Să se afișeze numerele din vector care se află pe poziții impare.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elemente ale vectorului, iar de pe a doua linie, n numere naturale, reprezentând elementele vectorului.
Date de ieșire

Programul afișează pe ecran elementele din vector care se află pe poziții impare. Elementele sunt separate prin câte un caracter spațiu.
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele vectorului ≤ 2.000.000.000
    Elementele vectorului sunt numerotate începând cu 1.

Exemplu

6
2 3 1 4 6 9

2 1 6

Explicația exemplului

Elementele de pe pozițiile impare sunt: 2 (poziția 1), 1 (poziția 3) și 6 (poziția 5).
*/
pub fn afisare() {
    let arr = [2, 3, 1, 4, 6, 9];

    for i in 0..arr.len() {
        if ((i + 1) % 2 != 0) {
            println!("{}", arr[i]);
        }
    }
}