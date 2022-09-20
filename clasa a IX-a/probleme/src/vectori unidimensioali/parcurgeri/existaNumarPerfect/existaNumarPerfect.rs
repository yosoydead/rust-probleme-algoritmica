/*
Problema Exista un numar perfect

Un număr natural se numește perfect dacă este egal cu suma divizorilor săi, excluzându-l pe el însuși. Spre exemplu, 6 are divizorii 1, 2, 3 (excluzându-l pe el însuși), iar 1 + 2 + 3 = 6.
Cerință

Se dă un șir de n numere naturale. Să se verifice dacă cel puțin un număr din șir este perfect.
Date de intrare

Programul citește de la tastatură, de pe primul rând, numărul natural n, iar de pe următorul rând, n numere naturale separate prin câte un spațiu, reprezentând valorile șirului.
Date de ieșire

Programul afișează pe ecran mesajul DA, dacă există cel puțin un număr perfect în șirul dat, respectiv NU în caz contrar.
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele șirului ≤ 2.000.000.000

Exemplu

5
3 6 1 2 3

DA

Explicația exemplului

În șirul dat apare numărul 6, care este număr perfect (deoarece 1 + 2 + 3 = 6, conform definiției menționate mai sus). Astfel, pe ecran se afișează mesajul DA.
*/
pub fn existaNumarPerfect() {
    let arr = [3, 6, 1, 2, 3];

    // println!("{}",sumaDivizori(6));
    let r = areNrPerfect(&arr);

    if (r == true) {
        println!("DA");
    } else {
        println!("NU");
    }
}

fn sumaDivizori(n: i32) -> i32 {
    if (n == 1) { return 0; }
    if (n <= 3) { return 1; }

    let mut sum = 1;
    for i in 2..n+1 {
        if (n % i == 0 && i != n) {
            sum += i;
        }
    }

    return sum;
}

fn areNrPerfect(arr: &[i32]) -> bool {
    let mut res = false;

    for i in 0..arr.len() {
        let x = sumaDivizori(arr[i]);

        if (x == arr[i]) {
            return true;
        }
    }
    return res;
}