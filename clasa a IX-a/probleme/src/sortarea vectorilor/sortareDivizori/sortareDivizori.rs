/*
Cerința
Se dau n numere naturale nenule. Ordonați descrescător cele n numere după numărul lor de divizori.

Date de intrare
Fișierul de intrare sortare_divizori.in conține pe prima linie numărul n, iar pe a doua linie n numere naturale nenule separate prin câte un spațiu.

Date de ieșire
Fișierul de ieșire sortare_divizori.out va conține cele n numere aflate pe a doua linie a fișierului de intrare ordonate descrescător după numărul de divizori.

Restricții și precizări
1 ≤ n ≤ 1000
numerele de pe a doua linie a fișierului de intrare vor fi mai mici decât 1.000.000.000
dacă există mai multe numere care au același număr de divizori, acestea vor fi ordonate crescător

Exemplu
sortare_divizori.in

5
12 20 4 100 13
sortare_divizori.out

100 12 20 4 13
Explicație
12 are 6 divizori, 20 are 6 divizori, 4 are 3 divizori, 100 are 9 divizori, 13 are 2 divizori, 12 și 20 au același număr de divizori. Așadar ordinea va fi 100 12 20 4 13.

MENTIUNE:
    - 12 si 20 au acelasi numar de divizori
    - pentru ca 20 se afla inaintea lui 12, la cautare el o sa retina prima chestie gasita, adica 20\
    - solutia e corecta in teorie
*/
pub fn sortareDivizori() {
    let mut arr = [12, 20, 4, 100, 13];
    println!("{:?}", arr);

    let mut indexing = 0;

    while (indexing < arr.len()) {
        let result = gasesteIndexCuDivizori(&arr[indexing..]);
        
        let temp = arr[indexing];
        arr[indexing] = arr[indexing + result.0];
        arr[indexing + result.0] = temp;

        indexing += 1;
        println!("{:?}", arr);
    }
    // let x = gasesteIndexCuDivizori(&arr);
    // println!("{:?}", x);
    // for i in 0..arr.len() {
    //     println!("{}", nrDivizori(arr[i]));
    // }
    println!("{:?}", arr);
}

fn nrDivizori(number: i32) -> u8 {
    let mut result: u8 = 0;

    for i in 1..number+1 {
        if (number % i == 0) {
            result += 1;
        }
    }

    return result;
}

fn gasesteIndexCuDivizori(arr: &[i32]) -> (usize, u8) {
    let mut index = 0;
    let mut lastBigDiv = 0;

    for i in 0..arr.len() {
        let divs = nrDivizori(arr[i]);

        if (divs > lastBigDiv) {
            lastBigDiv = divs;
            index = i;
        }
    }

    return (index, lastBigDiv);
}
