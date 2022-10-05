/*
Problema Sortare suma cifrelor
Iconita bookmark
Cerință
Se dă un șir de n numere naturale. Să se ordoneze elementele șirului în ordine crescătoare după suma cifrelor.

Date de intrare
Programul citește de la tastatură, de pe prima linie, numărul natural n reprezentând numărul de elemente ale șirului, iar de pe a doua linie, n valori naturale reprezentând elementele șirului.

Date de ieșire
Programul afișează pe ecran șirul ordonat crescător după suma cifrelor.

Restricții și precizări
1 ≤ n ≤ 10.000
1 ≤ elementele șirului ≤ 2.000.000.000
Exemplu
6
25 332 1111 24 476 9
1111 24 25 332 9 476
Explicația exemplului
Elementele șirului sunt 25 332 1111 24 476 9. Sumele cifrelor fiecărui număr în parte sunt, pe rând, 7 8 4 6 17 9. Astfel, ordonându-se după suma cifrelor, ordinea finală a numerelor este 1111 24 25 332 9 476.
*/
pub fn sortareSumaCifrelor() {
    let arr = [25, 332, 1111, 24, 476, 9];
    let mut cp = [25, 332, 1111, 24, 476, 9];

    let mut sums: [(usize, i32); 6] = [(0, 0); 6];

    for i in 0..arr.len() {
        let s = sumaCifre(arr[i]);
        sums[i] = (i, s);
    }

    println!("sums {:?}", sums);
    sort(&mut sums);
    println!("sums {:?}", sums);

    // println!("arr {:?}", arr);

    for i in 0..arr.len() {
        let toBeIndex = sums[i].0;
        let toBe = arr[toBeIndex];
        cp[i] = toBe;
    }
    println!("arr {:?}", cp);

}

fn sumaCifre(n: i32) -> i32 {
    let mut cp = n;
    let mut result = 0;

    while cp != 0 {
        result += cp % 10;
        cp = cp / 10;
    }

    return result;
}

fn sort(arr: &mut [(usize, i32)]) {
    let mut index = 0;

    while (index < arr.len()) {
        let mut j = index;

        while (j > 0 && arr[j-1].1 > arr[j].1) {
            let temp = arr[j-1];
            arr[j-1] = arr[j];
            arr[j] = temp;

            j -= 1;
        }

        index += 1;
    }
}