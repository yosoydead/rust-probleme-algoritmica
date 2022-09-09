/*
Cerinţa
Se citește de la tastură un număr natural n, apoi n numere naturale. Să se afişeze cel mai mic număr care poate fi scris folosind prima cifră a numerelor citite.

Date de intrare
Programul citește de la tastatură numărul n, iar apoi cele n numere naturale, separate prin spaţii.

Date de ieşire
Programul afișează pe ecran numărul MIN, cel mai mic număr care poate fi scris folosind prima cifră a numerelor citite.

Restricţii şi precizări
0 < n < 1000
cele n numere citite vor fi nenule și mai mici decât 1.000.000.000

Exemplu
Intrare

5
100 312 276 985 5021
Ieșire

12359
*/
pub fn cifInit() {
    let arr = [100, 312, 276, 985, 5021];
    let mut cifre: [i32; 5] = [0; 5];

    for i in 0..arr.len() {
        // println!("{}", primaCifra(arr[i]));
        let c = primaCifra(arr[i]);
        cifre[i] = c;
    }

    let mut currentIndex = 0;

    while (currentIndex < cifre.len()) {
        let x = findSmallestAtIndex(&cifre[currentIndex..]);

        let temp = cifre[currentIndex];
        cifre[currentIndex] = cifre[currentIndex + x];
        cifre[currentIndex + x] = temp;

        currentIndex += 1;
    }

    println!("numar format {:?}", formNumberFromArray(&cifre));
}

fn primaCifra(mut number: i32) -> i32 {
    let mut result = 0;

    while (number != 0) {
        let c = number % 10;
        result = c;
        number = number / 10;
    }
    
    return result;
}

fn findSmallestAtIndex(arr: &[i32]) -> usize {
    let mut current = std::i32::MAX;
    let mut index  = 0;
    for i in 0..arr.len() {
        if (current > arr[i]) {
            index = i;
            current = arr[i];
        }
    }

    return index;
}

fn formNumberFromArray(arr: &[i32]) -> i32 {
    let mut result = 0;

    for i in 0..arr.len() {
        result = (result * 10) + arr[i];
    }

    return result;
}