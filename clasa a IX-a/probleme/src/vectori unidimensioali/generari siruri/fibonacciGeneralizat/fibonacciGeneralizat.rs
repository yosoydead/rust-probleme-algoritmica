/*
Problema Fibonacci generalizat
Cerință

Se dau numerele n, x și y. Să se afișeze primii n termeni ai șirului Fibonacci generalizat, cu primele două elemente x și y. Mai exact, șirul se va calcula exact ca și cel Fibonacci, doar că primii doi termeni sunt x și y în loc de 1.
Date de intrare

Programul citește de la tastatură numerele n, x și y, despărțite printr-un spațiu.
Date de ieșire

Programul afișează pe ecran primii n termeni ai șirului descris în cerință.
Restricții și precizări

    1 ≤ n ≤ 10
    1 ≤ toți termenii care sunt afișați ≤ 1.000.000.000

Exemplu

5 3 3

3 3 6 9 15

Explicația exemplului

Regula șirului este f1 = 3, f2 = 3, și fn = fn - 2 + fn - 1 (pentru n ≥ 3). Primii 5 termeni ai șirului cu această regulă sunt 3, 3, 6, 9, 15.
*/
pub fn fibonacciGeneralizat() {
    let n = 5;
    
    let mut a = 3;
    let mut b = 3;
    let mut c = a + b;

    for i in 0..n {
        println!("{}", a);

        a = b;
        b = c;
        c = a + b;
    }
}