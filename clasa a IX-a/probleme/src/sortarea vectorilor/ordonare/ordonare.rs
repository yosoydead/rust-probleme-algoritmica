/* Se dă un vector cu n elemente numere naturale.

Cerinţa
Să se ordoneze crescător elementele vectorului.

Date de intrare
Programul citește de la tastatură numărul n, apoi n numere întregi, reprezentând elementele vectorului. O SA FAC CU UN VECTOR HARDCODAT

Date de ieşire
Programul va afișa pe ecran cele n elemente ale vectorului, ordonate conform cerinței, separate printr-un spațiu. */
pub fn ordonare(array: &[i32], asc: bool) {
    displayArray(&array);
    let mut cp = copy(&array);
    // bubbleSort(&mut cp);
    selectionSort(&mut cp);
    displayArray(&cp);
}

fn copy(arrToCopy: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 0..arrToCopy.len() {
        result.push(arrToCopy[i]);
    }

    return result;
}

fn displayArray(arr: &[i32]) {
    for i in 0..arr.len() {
        print!("{}, ", arr[i]);
    }
    println!("");
}

fn bubbleSort(arr: &mut[i32]) {
    for i in 0..arr.len() {
        // arr[i] = arr[i] + 1;
        for j in 0..arr.len() -1 {
            // println!("compar {} cu {}", arr[j], arr[j+1]);
            if (arr[j] > arr[j+1]) {
                let temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
}

fn selectionSort(arr: &mut[i32]) {
    let mut currentIndex = 0;

    fn findSmallest(slice: &[i32]) -> (i32, usize) {
        // println!("slice {:?}", slice);
        let mut smallestNum = i32::MAX;
        let mut indexToReturn: usize = 0;

        for i in 0..slice.len() {
            if (slice[i] < smallestNum) {
                smallestNum = slice[i];
                indexToReturn = i;
            }
        }

        return (smallestNum, indexToReturn);
    }
    // println!("{:?}", bla);
    // let x = arr[bla.1];
    while (currentIndex < arr.len()) {
        let foundElem = findSmallest(&arr[currentIndex ..]);
        // println!("{:?}", foundElem);
        let temp = arr[currentIndex];
        arr[currentIndex] = arr[foundElem.1 + currentIndex];
        arr[foundElem.1 + currentIndex] = temp;

        // println!("temp {}, arr currIndex {}, arrFoundIndex {}", temp, arr[currentIndex], arr[foundElem.1]);

        currentIndex += 1;
    }
}