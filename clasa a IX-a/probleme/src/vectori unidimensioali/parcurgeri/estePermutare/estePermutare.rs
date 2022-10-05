/*
Problema Este permutare

O permutare a unei mulțimi reprezintă o altă așezare a elementelor acesteia.
Cerință

Se dă un șir de n numere naturale. Să se verifice dacă acest șir este sau nu o permutare a mulțimii {1, 2, 3, …, n}.
Date de intrare

Programul citește de la tastatură, de pe primul rând, numărul natural n, iar de pe următorul rând, n numere naturale separate prin câte un spațiu, reprezentând valorile șirului.
Date de ieșire

Programul afișează pe ecran mesajul DA, dacă șirul reprezintă o permutare a mulțimii formate din primele n numere naturale nenule, respectiv NU în caz contrar.
Restricții și precizări

    1 ≤ n ≤ 1000
    1 ≤ elementele șirului ≤ 2.000.000.000

Exemplu

5
5 4 2 1 3

DA

Explicația exemplului

Cele n = 5 elemente formează o permutare a mulțimii {1, 2, 3, 4, 5}, așadar pe ecran se afișează mesajul DA.
*/
pub fn estePermutare() {
    let arr = [5,4,2,1,3];
    let mut sir: [i32; 5] = [-1; 5];
    
    for i in 0..5 {
        sir[i] = i as i32 +1;
    }

    for i in 0..arr.len() {
        let r = findNum(&sir, arr[i]);

        if (r == false) {
            println!("NU");
            return;
        }
    }

    println!("DA");
}

fn findNum(arr: &[i32], n: i32) -> bool {
    let mut l = 0;
    let mut r = arr.len();

    while (l < r) {
        let m = (((l+r) as f32) / 2.0).floor() as usize;

        if (arr[m] < n) {
            l += 1;
        } else if (arr[m] > n) {
            r -= 1;
        } else {
            return true;
        }
    }

    return false;
}
