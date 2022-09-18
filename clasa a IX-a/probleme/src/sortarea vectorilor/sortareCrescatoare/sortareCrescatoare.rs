/*
Cerință
Se dă un șir de n numere naturale. Să se ordoneze crescător elementele acestui șir.

Date de intrare
Programul citește de la tastatură un număr natural n, urmat de cele n numere naturale ale șirului.

Date de ieșire
Programul afișează pe ecran șirul cu elementele sale ordonate crescător.

Restricții și precizări
1 ≤ n ≤ 1000
1 ≤ elementele șirului ≤ 1000
Exemplu
5
32 12 14 12 1
1 12 12 14 32
Explicația exemplului
Cele 5 numere, ordonate crescător, sunt în ordinea 1 12 12 14 32.
*/
pub fn sortareCrescatoare() {
    let mut arr: [i32; 5] = [32, 12, 14, 12, 1];
    println!("arr: {:?}", arr);
    sort(&mut arr);
    println!("arr: {:?}", arr);
}

fn sort(array: &mut [i32]) {
    let mut index = 1;

    while (index < array.len()) {
        let mut j = index;

        while (j > 0 && array[j-1] > array[j]) {
            let temp = array[j-1];
            array[j-1] = array[j];
            array[j] = temp;

            j -= 1;
        }

        index += 1;
    }
}