/*
Problema Generare sir
Cerință

Se definește șirul f astfel: f1 = 2, fx = fx - 1 * 2 - 1. Dându-se un număr natural n, să se găsească al n-lea element al șirului dat.
Date de intrare

Programul citește de la tastatură un singur număr natural, n.
Date de ieșire

Programul afișează pe ecran al n-lea element al șirului f definit în cerință.
Restricții și precizări

    1 ≤ n ≤ 30
    Se garantează că 1 ≤ fi ≤ 2.000.000.000, pentru 1 ≤ i ≤ n

Exemplu

6

33

Explicația exemplului

Primii termeni ai șirului sunt 2, 3, 5, 9, 17, 33, …. Astfel, al 6-lea termen al șirului este 33.
*/
pub fn generareSir() {
    let n = 6;

    let mut x = 2;

    for i in 0..n {
        println!("{}", x);
        x = (x * 2) - 1;
    }
}