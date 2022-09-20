/*
Problema Este sir palindrom

Un șir de numere naturale se consideră palindrom dacă primul element este egal cu ultimul, al doilea este egal cu penultimul, și așa mai departe. Spre exemplu, șirul (1, 43, 5, 43, 1) este un șir palindrom.
Cerință

Se dă un șir de n numere naturale. Să se verifice dacă șirul dat este sau nu palindrom.
Date de intrare

Programul citește de la tastatură, de pe primul rând, numărul natural n, iar de pe următorul rând, n numere naturale separate prin câte un spațiu, reprezentând valorile șirului.
Date de ieșire

Programul afișează pe ecran mesajul DA, dacă șirul este palindrom, respectiv NU în caz contrar.
Restricții și precizări

    1 ≤ n ≤ 1000
    1 ≤ elementele șirului ≤ 2.000.000.000

Exemplu

5
1 43 5 43 1

DA

Explicația exemplului

Primul element este egal cu ultimul (1), al doilea este egal cu penultimul (43), iar al treilea element coincide cu antepenultimul (5), așadar șirul este palindrom. Prin urmare, pe ecran se afișează mesajul DA.
*/
pub fn esteSirPalindrom() {
    let arr = [1, 43, 5, 43, 1];

    let mut i = 0;
    let mut j = arr.len() - 1;

    while (i < j) {
        // println!("{}, {}", arr[i], arr[j]);
        if (arr[i] != arr[j]) {
            println!("NU");
            return;
        }

        i += 1;
        j -= 1;
    }

    println!("DA");
}