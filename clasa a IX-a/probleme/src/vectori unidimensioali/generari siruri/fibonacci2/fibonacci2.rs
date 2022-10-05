/*
Problema Fibonacci 2
Cerință

Se dă un număr natural, n. Să se afișeze pe ecran termenii Fibonacci mai mici sau egali decât n.
Date de intrare

Programul citește de la tastatură un singur număr natural, n.
Date de ieșire

Programul afișează pe ecran, pe prima linie, termenii șirului Fibonacci mai mici decât n. Termenii sunt afișați în ordine crescătoare.
Restricții și precizări

    1 ≤ n ≤ 2.000.000.000

Exemplu

15

1 1 2 3 5 8 13

Explicația exemplului

Termenii șirului Fibonacci, mai mici decât 15, sunt 1, 1, 2, 3, 5, 8, 13.
*/
pub fn fibonacci2() {
    let n = 15;

    let mut a = 1;
    let mut b = 1;
    let mut c = a + b;

    while (a <= n) {
        println!("{}", a);

        a = b;
        b = c;
        c = a + b;
    }
}