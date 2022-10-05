/* 
Cerința
Se dă un vector cu n elemente, numere naturale. Afișați în ordine crescătoare valorile prime din acest vector.

Date de intrare
Programul citește de la tastatură numărul n, iar apoi n numere naturale, reprezentând elementele vectorului.

Date de ieșire
Programul va afișa pe ecran valorile prime din vector, în ordine crescătoare, separate prin exact un spațiu.

Restricții și precizări
1 ≤ n ≤ 1000
cele n numere citite vor fi mai mici decât 1.000.000.000
*/
pub fn sortPrime(arr: &[i32]) {
    // let mut cp = copy(&arr);
    let mut cp = [13, 1, 10, 15, 3, 7, 11];
    insertionSort(&mut cp);
    println!("{:?}", cp);
    
    for i in 0..cp.len() {
        if (isPrime(cp[i]) == true) {
            print!("{} ", cp[i]);
        }
    }
}

fn copy(arrToCopy: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 0..arrToCopy.len() {
        result.push(arrToCopy[i]);
    }

    return result;
}

fn isPrime(number: i32) -> bool {
    if (number < 2) { return false; }
    if (number == 2) { return true; }

    let root = (number as f32).sqrt() as i32 + 1;
    for i in 2..root {
        if (number % i == 0) {
            return false;
        }
    }

    return true;
}

fn insertionSort(arr: &mut[i32]) {
    let mut index = 1;

    while (index < arr.len()) {
        let mut j = index;
        
        while j > 0 && arr[j-1] > arr[j] {
            let temp = arr[j-1];
            arr[j-1] = arr[j];
            arr[j] = temp;

            j -= 1;
        }

        index += 1;
    }
}