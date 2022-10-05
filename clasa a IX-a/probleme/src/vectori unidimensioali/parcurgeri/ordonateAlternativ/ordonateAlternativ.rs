/*
Problema Ordonate alternativ
Cerință

Se dă un șir de n numere naturale. Să se verifice dacă elementele șirului aflate pe poziții pare sunt așezate strict crescător, iar elementele șirului aflate pe poziții impare sunt așezate strict descrescător.
Date de intrare

Programul citește de la tastatură, de pe primul rând, numărul natural n, iar de pe următorul rând, n numere naturale separate prin câte un spațiu, reprezentând valorile șirului.
Date de ieșire

Programul afișează pe ecran mesajul DA, dacă elementele șirului de pe poziții impare sunt ordonate strict descrescător, iar elementele șirului de pe poziții pare sunt ordonate strict crescător, respectiv NU în caz contrar.
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele șirului ≤ 2.000.000.000

Exemplu

5
3 5 2 5 1

NU

Explicația exemplului

Elementele de pe poziții impare sunt: 3 2 1. Acestea sunt așezate strict descrescător. Cu toate acestea, elementele de pe poziții pare, 5 5, nu sunt așezate strict crescător. Așadar, se afișează pe ecran mesajul NU.
*/
pub fn ordonateAlternativ() {
    let arr = [3, 5, 2, 5, 1];
    let mut pare = Vec::new();
    let mut impare = Vec::new();

    for i in 0..arr.len() {
        if ((i + 1) % 2 == 0) {
            pare.push(arr[i]);
        } else {
            impare.push(arr[i]);
        }
    }

    for j in 0..impare.len() - 1 {
        if (impare[j] <= impare[j + 1]) {
            println!("NU");
            return;
        }
    }

    for k in 0..pare.len() - 1 {
        if (pare[k] >= pare[k + 1]) {
            println!("NU");
            return;
        }
    }

    println!("DA");
}
