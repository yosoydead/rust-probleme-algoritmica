/*
Problema Secvente de numere egale
Cerință

Se dă un șir de n numere naturale, împreună cu o valoare x. Să se determine numărul de secvențe din șir în care toate elementele sunt egale cu x.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numerele naturale n și x, de pe a doua linie, cele n numere naturale.
Date de ieșire

Programul afișează pe ecran un singur număr natural, reprezentând numărul de secvențe din șir cu toate elementele egale cu x.
Restricții și precizări

    1 ≤ n ≤ 1000
    1 ≤ numerele șirului, x ≤ 2.000.000.000
    Cel puțin un element al șirului va fi egal cu x

Exemplu

10 4
1 4 4 4 5 7 4 9 4 2

3

Explicația exemplului

Secvențele din șir în care toate elementele sunt egale cu x = 4 sunt 1 4 4 4 5 7 4 9 4 2. În total sunt 3 secvențe.
*/
pub fn secventeDeNumereEgale() {
    let arr = [1, 4, 4, 4, 5, 7, 4, 9, 4, 2, 4, 2];
    let toFind = 4;

    let mut sec = 0;
    let mut inSec = false;

    for i in 0..arr.len() {
        if (arr[i] == toFind && inSec == false) {
            sec += 1;
            inSec = true;
        }
        if (arr[i] != toFind) {
            inSec = false;
        }
    }

    println!("secvente de {} sunt {}", toFind, sec);
}