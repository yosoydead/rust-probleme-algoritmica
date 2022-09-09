/*
Cerința
Se dă un vector cu n elemente, numere naturale distincte. Ordonați crescător elementele situate înaintea valorii maxime din vector și descrescător elementele situate după această valoare.

Date de intrare
Programul citește de la tastatură numerele n, iar apoi n numere naturale, reprezentând elementele vectorului.

Date de ieșire
Programul va afișa pe ecran elementele vectorului, separate prin exact un spațiu, după efectuarea operațiilor cerute.

Restricții și precizări
1 ≤ n ≤ 1000
cele n numere citite vor fi mai mici decât 1.000.000.000

Exemplu
Intrare

7
13 1 10 15 3 7 11
Ieșire

1 10 13 15 11 7 3
*/

pub fn sortMax() {
    let mut arr = [13, 1, 10, 15, 3, 7, 11];

    let mut currentIndex = 0;
    let bigIndex = findBiggestAtIndex(&arr);

    while (currentIndex < arr.len()) {
        let mut i = 0;
        if (currentIndex < bigIndex) {
            i = findSmallestAtIndex(&arr[currentIndex..bigIndex]);
        } else {
            i = findBiggestAtIndex(&arr[currentIndex..]);
        }
        
        let temp = arr[currentIndex];
        arr[currentIndex] = arr[currentIndex+i];
        arr[currentIndex+i] = temp;
        currentIndex += 1;
    }
    
    println!("{:?}", arr);
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

fn findBiggestAtIndex(arr: &[i32]) -> usize {
    let mut current = std::i32::MIN;
    let mut index  = 0;
    for i in 0..arr.len() {
        if (current < arr[i]) {
            index = i;
            current = arr[i];
        }
    }

    return index;
}
