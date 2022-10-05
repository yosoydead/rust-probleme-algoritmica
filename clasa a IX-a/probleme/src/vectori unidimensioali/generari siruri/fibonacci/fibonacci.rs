/*
Problema Fibonacci
Cerință

Se dă un număr natural, n. Să se afișeze primii n termeni ai șirului Fibonacci.
Date de intrare

Programul citește de la tastatură un singur număr natural, n.
Date de ieșire

Programul afișează pe ecran, pe prima linie, primii n termeni ai șirului Fibonacci, despărțiți printr-un spațiu.
Restricții și precizări

    1 ≤ n ≤ 45

Exemplu

5

1 1 2 3 5

Explicația exemplului

Primii 5 termeni ai șirului Fibonacci sunt 1, 1, 2, 3, 5.
*/
pub fn fibonacci() {
    let n = 5;

    let mut a = 1;
    let mut b = 1;
    let mut c = a + b;

    for i in 0..n {
        println!("{}", a);
        a = b;
        b = c;
        c = a + b;
    }
}