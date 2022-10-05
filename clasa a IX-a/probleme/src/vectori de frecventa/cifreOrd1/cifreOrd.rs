/*
Cerinţa

Se dau mai multe numere naturale, fiecare cu cel mult 9 cifre. Să se afişeze, în ordine descrescătoare, toate cifrele care apar în numerele date.
Date de intrare

Fişierul de intrare cifreord1.in conţine cel mult 10.000 numere naturale, dispuse pe mai multe linii.
Date de ieşire

Fişierul de ieşire cifreord1.out va conţine cifrele determinate, ordonate descrescător, câte 20 pe o linie, valorile de pe fiecare linie fiind separate prin spaţii. Ultima linie a fişierului poate conţine mai puţin de 20 de cifre.

Exemplu

cifreord1.in

301941 81912 83392
776996 431446

cifreord1.out

9 9 9 9 9 8 8 7 7 6 6 6 4 4 4 4 3 3 3 3 
2 2 1 1 1 1 1 0 
*/
pub fn cifreOrd() {
    let arr = [301941, 81912, 83392, 776996, 431446];
    let mut frec: [i8; 10] = [0; 10];

    println!("{:?}", frec);
    // adaugaCifre(1234567890, &mut frec);
    for i in 0..arr.len() {
        adaugaCifre(arr[i], &mut frec);
    }

    for i in 0..frec.len() {
        for j in 0..frec[i] {
            print!("{} ", 9 - i);
        }
    }

    // println!("{:?}", frec);
}

fn adaugaCifre(n: i32,  arr: &mut [i8]) {
    let mut cp = n;

    while (cp != 0) {
        let c = cp % 10;
        let index = 9 - (c % 10) as usize;
        // println!("cifra {}. index normal {}. index inversat {}", c, c, 9 - (c % 10));
        arr[index] += 1;
        cp = cp / 10;
    }
}