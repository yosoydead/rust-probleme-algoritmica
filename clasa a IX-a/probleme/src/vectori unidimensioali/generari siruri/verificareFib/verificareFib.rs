/*
Problema Verificare Fibonacci
Cerință

Se dau n numere naturale. Pentru fiecare, să se determine dacă este sau nu termen al șirului Fibonacci.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul n, iar de pe următoarea linie, cele n numere naturale.
Date de ieșire

Programul afișează pe ecran n valori de 0 sau de 1, cu următoarea semnificație: dacă al i-lea număr afișat este 0, înseamnă că al i-lea număr nu este termen al șirului Fibonacci, iar dacă este 1, înseamnă că este.
Restricții și precizări

    1 ≤ n ≤ 100.000
    1 ≤ cele n numere ≤ 2.000.000.000

Exemplu

5
1 2 3 4 5

1 1 1 0 1

Explicația exemplului

Primele numere din șirul Fibonacci sunt 1, 1, 2, 3, 5, 8, 13, …. Astfel, dintre cele 5 numere date, doar 1, 2, 3 și 5 sunt termeni ai șirului Fibonacci.
*/
pub fn verificareFib() {
    const n: usize = 5;
    let arr: [u32; n] = [1, 2, 3, 4, 5];
    let mut fibs: [u32; n] = [0; n];

    let mut a = 1;
    let mut b = 1;
    let mut c = a + b;

    for i in 0..n {
        fibs[i] = b;

        a = b;
        b = c;
        c = a + b;
    }
    
    println!("{:?}", arr);
    println!("{:?}", fibs);

    for i in 0..arr.len() {
        let result = search(&fibs, arr[i]);

        if result == true {
            print!("1 ", )
        } else {
            print!("0 ", )
        }
    }
}

fn search(arr: &[u32], n: u32) -> bool {
    let mut left: usize = 0;
    let mut right: usize = arr.len() -1;

    while (left <= right) {
        let mid = (left + right) / 2;
        if (arr[mid] < n) {
            left = mid + 1;
        }
        if (arr[mid] > n) {
            right = mid - 1;
        }
        if (arr[mid] == n){
            return true;
        }
    }
    return false;
}