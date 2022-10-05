/*
Problema Sortare numar divizori
Iconita bookmark
Cerință
Se dă un șir de n numere naturale. Să se ordoneze elementele șirului în ordine descrescătoare după numărul de divizori. Dacă două numere au același număr de divizori, acestea se ordonează descrescător.

Date de intrare
Programul citește la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elemente ale șirului, iar de pe a doua linie, n valori naturale reprezentând elementele șirului.

Date de ieșire
Programul afișează pe ecran șirul ordonat descrescător după numărul divizorilor.

Restricții și precizări
1 ≤ n ≤ 5000
1 ≤ elementele șirului ≤ 2.000.000
Exemplu
6
2 3 1 4 6 9
6 9 4 3 2 1
Explicația exemplului
Elementele șirului sunt 2 3 1 4 6 9. 2 are 2 divizori, 3 are tot 2 divizori, 1 are 1 singur divizor, 4 are 3 divizori, 6 are 4 divizori, iar 9 are 3 divizori. Elementele se ordonează în ordine descrescătoare după numărul de divizori, astfel, se formează șirul final 6 9 4 3 2 1.
*/
pub fn sortareNumarDivizori() {
    let arr = [2, 3, 1, 4, 6, 9];
    let mut cp = [2, 3, 1, 4, 6, 9];
    let mut divs: [(usize, i32); 6] = [(0, 0); 6];
    
    for i in 0..arr.len() {
        let d = nrDivizori(arr[i]);
        divs[i] = (i, d);
    }

    sort(&mut divs);
    for i in 0..arr.len() {
        let toBeIndex = divs[i].0;
        cp[i] = arr[toBeIndex];
    }

    println!("{:?}", cp);

}

fn nrDivizori(n: i32) -> i32 {
    let mut result = 2;
    for i in 2..n {
        if (n % i == 0) {
            result += 1;
        }
    }

    return result;
}

fn sort(arr: &mut [(usize, i32)]) {
    let mut index = 1;
    while (index < arr.len()) {
        let mut j = index;

        while (j > 0 && arr[j-1].1 < arr[j].1) {
            let temp = arr[j-1];
            arr[j-1] = arr[j];
            arr[j] = temp;

            j -= 1;
        }

        index += 1;
    }
}