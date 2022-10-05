/*
Cerința

Andrei lucrează într-un magazin de șosete, iar fiecare șosetă are asociat un numar întreg pentru a o identifica mai ușor din ce model face parte. Dându-se un vector neordonat de astfel de coduri, aflați câte perechi de șosete se pot forma.
Date de intrare

Programul citește de la tastatură numărul n, iar apoi n numere naturale, separate prin spații.
Date de ieșire

Programul va afișa pe ecran numărul de perechi care se pot forma.
Restricții și precizări

    1 ≤ n ≤ 100
    Codurile șosetelor sunt cuprinse în intervalul [1, 100]


Exemplu

Intrare

10
1 3 2 1 2 2 1 2 1 2

Ieșire

4

Explicație

Se formează 2 perechi cu șosetele care au codul 1 și încă 2 cu cele care au codul 2.
*/
pub fn perechiSosete() {
    let arr = [1, 3, 2, 1, 2, 2, 1, 2, 1, 2];
    let mut frec: [i8; 11] = [0; 11];

    for i in 0..arr.len() {
        let j = arr[i];
        frec[j] += 1;
    }

    let mut perechi = 0;
    for i in 0..frec.len() {
        // println!("{}", frec[i] / 2);
        if (frec[i] != 0) {
            perechi += (frec[i] / 2);
        }
    }

    println!("{:?}", frec);
    println!("perechi {}", perechi);
}