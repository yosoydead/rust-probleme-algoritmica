/*
Problema Determinare vector 4
Cerință

Se dau două șiruri de numere naturale, ambele cu câte n elemente. Să se determine dacă există un număr natural x, astfel încât înmulțind elementele primului șir cu x, obținem al doilea șir.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elemente ale șirurilor. De pe a doua linie, se citesc n numere naturale, reprezentând elementele primului șir, iar de pe a treia linie se citesc alte n numere naturale, reprezentând elementele celui de al doilea șir. Numerele de pe același rând sunt separate prin câte un spațiu.
Date de ieșire

Programul afișează pe ecran, mesajul DA, dacă există un număr natural x cu proprietatea descrisă în enunț, respectiv NU, în caz contrar.
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele celor două șiruri ≤ 2.000.000.000

Exemplu

6
2 3 1 4 6 9
4 6 2 8 12 18

DA

Explicația exemplului

Pentru x = 2, 2 * x = 4, 3 * x = 6, 1 * x = 2, …, 9 * x = 18.
*/
pub fn determinare4() {
    let arr = [2, 3, 1, 4, 6, 9];
    let arr2 = [4, 6, 2, 8, 12, 18];

    let r = d(&arr, &arr2);
    if (r == true) {
        println!("DA");
    } else {
        println!("NU");
    }
}

fn d(arr: &[i32], arr2: &[i32]) -> bool {
    let mut res = true;
    let mut div = arr2[0] / arr[0];

    for i in 1..arr.len() {
        let newDiv = arr2[i] / arr[i];

        if (newDiv != div) {
            res = false;
            return res;
        }
    }

    return res;
}