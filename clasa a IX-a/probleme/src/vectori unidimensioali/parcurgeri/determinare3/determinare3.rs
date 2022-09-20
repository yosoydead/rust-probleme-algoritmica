/*
Problema Determinare vector 3
Cerință

Se dă un șir de n numere naturale. Să se determine perechile ordonate de elemente ale șirului cu proprietatea că atât ele, cât și indicii lor sunt prime între ele: (Ai; Aj) = 1, (i; j) = 1, pentru 1 ≤ i < j ≤ n.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elemente ale șirului, iar de pe a doua linie, n numere naturale, reprezentând elementele șirului.
Date de ieșire

Programul afișează pe ecran, pe câte o linie, perechile de indici i j, cu 1 ≤ i < j ≤ n, cu proprietatea că atât i, j, cât și Ai, Aj sunt prime între ele. Indicii de pe aceeași linie sunt separați printr-un spațiu.
Restricții și precizări

    1 ≤ n ≤ 100
    1 ≤ elementele vectorului ≤ 2.000.000.000
    Elementele șirului sunt numerotate începând cu 1.

Exemplu

6
2 3 1 4 6 9

1 2
1 3
1 6
2 3
3 4
3 5

Explicația exemplului

Pentru prima pereche: (1; 2) = 1, dar de asemenea (A1; A2) = (2; 3) = 1. La fel se poate verifica și pentru celelalte perechi.
*/
pub fn determinare3() {
    let arr = [2, 3, 1, 4, 6, 9];

    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            let indexes = cmmdc((i + 1) as i32, (j + 1) as i32);
            let nrs = cmmdc(arr[i], arr[j]);

            if (indexes == 1 && nrs == 1) {
                println!("{} {}", i+1, j+1);
            }
        }
    }
}

fn cmmdc(a: i32, b: i32) -> i32 {
    let mut i = a;
    let mut j = b;

    while (i != j) {
        if (i > j) {
            let newVal = i - j;
            i = newVal;
        } else {
            let newVal = j - i;
            j = newVal;
        }
    }

    return j;
}