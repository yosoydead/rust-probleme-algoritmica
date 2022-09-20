/*
Problema Inlocuire vector
Cerință

Se dă un șir de n numere naturale. Să se înlocuiască numerele impare din vector cu dublul lor, după care să se afișeze vectorul modificat.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elemente ale vectorului, iar de pe a doua linie, n numere naturale, reprezentând elementele vectorului.
Date de ieșire

Programul afișează pe ecran vectorul modificat după regulile specificate în cerință (elementele impare se înlocuiesc cu dublul lor).
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele vectorului ≤ 1.000.000.000

Exemplu

6
2 3 1 4 6 9

2 6 2 4 6 18

Explicația exemplului

Elementele impare, 3, 1 și 9, se înlocuiesc cu dublul lor: 6, 2, respectiv 18.
*/
pub fn inlocuire() {
    let mut arr = [2, 3, 1, 4, 6, 9];

    for i in 0..arr.len() {
        if (arr[i] % 2 != 0) {
            let newNr = arr[i] * 2;
            arr[i] = newNr;
        }
    }

    println!("{:?}", arr);
}