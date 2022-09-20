/*
Problema Afisare vector 3
Cerință

Se dă un șir de n numere naturale. Să se afișeze elementele șirului care respectă proprietatea că cmmdc-ul dintre ele însăși și ultimul element al șirului este diferit de 1.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elemente ale șirului, iar de pe a doua linie, n numere naturale, reprezentând elementele șirului.
Date de ieșire

Programul afișează pe ecran elementele șirului care respectă proprietatea descrisă în cerință: cmmdc-ul dintre ele însăși și ultimul element al vectorului să fie diferit de 1.
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele vectorului ≤ 2.000.000.000
    Se garantează că va exista cel puțin un element în șir care, împreună cu ultimul element al șirului, să aibă cmmmdc-ul diferit de 1.

Exemplu

6
2 3 1 4 6 9

3 6 9

Explicația exemplului

Singurele numere care respectă proprietatea cerută sunt 3 ((3; 9) = 3), 6 ((6; 9) = 3) și 9 ((9; 9) = 9).
*/
pub fn afisare3() {
    let arr = [2, 3, 1, 4, 6, 9];
    let last = arr[arr.len() - 1];

    // cmmdc(48, 18);
    for i in 0..arr.len() {
        let n = cmmdc(arr[i], last);

        if (n != 1) {
            println!("{}", arr[i]);
        }
    }
}

fn cmmdc(a: i32, b: i32) -> i32 {
    let mut cpA = a;
    let mut cpB = b;

    while (cpA != cpB) {
        if (cpA > cpB) {
            let newVal = cpA - cpB;
            cpA = newVal;
        } else {
            let newVal = cpB - cpA;
            cpB = newVal;
        }
    }

    return cpA;
}
