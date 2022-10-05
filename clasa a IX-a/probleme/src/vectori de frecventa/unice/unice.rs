/*
erinţa

Se dau n numere numere naturale cu cel mult două cifre fiecare. Să se determine acele numere care apar o singură dată.
Date de intrare

Fişierul de intrare unice.in conţine pe prima linie numărul n; urmează n numere naturale cu cel mult două cifre fiecare, dispuse pe mai multe linii şi separate prin spaţii.
Date de ieşire

Fişierul de ieşire unice.out va conţine pe prima linie, în ordine crescătoare, valorile care apar o singură dată, separate printr-un spaţiu.
Restricţii şi precizări

    1 ≤ n ≤ 100.000


Exemplu

unice.in

7
3 5 2 1
5 23 1

unice.out

2 3 23
*/
pub fn unice() {
    let mut arr: [i8; 7] = [3, 5, 2, 1, 5, 23, 1];
    let mut vec: Vec<(i8, i8)> = Vec::new();
    let mut i = 0;

    sort(&mut arr);
    vec.push((arr[0], 1));

    for j in 1..arr.len() {
        if (arr[j] == vec[i].0) {
            vec[i].1 += 1;
        } else {
            vec.push((arr[j], 1));
            i += 1;
        }
    }
    // println!("{}", arr[i]);
    println!("{:?}", vec);
    printUnic(&vec);
}

fn sort(arr: &mut [i8]) {
    let mut index = 1;

    while (index < arr.len()) {
        let mut j = index;

        while (j > 0 && arr[j-1] > arr[j]) {
            let temp = arr[j-1];
            arr[j-1] = arr[j];
            arr[j] = temp;

            j -= 1;
        }

        index += 1;
    }
}

fn printUnic(arr: &Vec<(i8, i8)>) {
    for i in 0..arr.len() {
        if (arr[i].1 == 1) {
            print!("{} ", arr[i].0);
        }
    }
}
