/*
Problema Secventa de numere egale
Cerință

Se dă un șir de n numere naturale. Să se determine indicii de început și de sfârșit ai celei mai lungi secvențe din șir în care toate elementele sunt egale. Dacă există mai multe astfel de secvențe, se cere secvența cea mai din stânga.
Date de intrare

Programul citește de la tastatură, de pe prima linie, numerele naturale n, iar de pe a doua linie, cele n numere naturale.
Date de ieșire

Programul afișează pe ecran două numere naturale despărțite printr-un spațiu, reprezentând indicii de început și de sfârșit ai secvenței de lungime maximă.
Restricții și precizări

    1 ≤ n ≤ 1000
    1 ≤ numerele șirului ≤ 2.000.000.000
    Elementele vectorului sunt indexate de la 1

Exemplu

10
3 3 4 9 9 9 1 10 10 10

4 6

Explicația exemplului

Secvențele din șir de lungime maximă în care toate elementele sunt egale sunt 3 3 4 9 9 9 1 10 10 10. Secvența de lungime maximă 9 9 9, care începe pe poziția 4 și se termină pe poziția 6 (cu lungimea 3) este cea mai din stânga, astfel că răspunsul este 4 6.
*/
pub fn secventaDeNumereEgale() {
    let arr: [i32; 10] = [3, 3, 4, 9, 9, 9, 1, 10, 10, 10];
    let mut sec: [(i32, usize, usize); 10] = [(-1, 0, 0); 10];
    println!("sec {:?}", sec);

    let mut index = 0;
    sec[index] = (arr[0], 0, 1);

    for i in 1..arr.len() {
        if (sec[index].0 == arr[i]) {
            sec[index].2 = i;
        } else {
            index += 1;
            sec[index] = (arr[i], i, i+1);
        }
    }

    println!("sec {:?}", sec);

    let mut gap = 0;
    let mut resultIndex = 0;
    for i in 0..sec.len() {
        if (sec[i].0 == -1) {
            break;
        }

        if (sec[i].2 - sec[i].1 > gap) {
            gap = sec[i].2 - sec[i].1;
            resultIndex = i;
        }
    }

    println!("sec {:?}", sec[resultIndex]);
}