/* Se dă un vector cu n elemente numere naturale.

Cerinţa
Să se ordoneze crescător elementele vectorului.

Date de intrare
Programul citește de la tastatură numărul n, apoi n numere întregi, reprezentând elementele vectorului. O SA FAC CU UN VECTOR HARDCODAT

Date de ieşire
Programul va afișa pe ecran cele n elemente ale vectorului, ordonate conform cerinței, separate printr-un spațiu. */
pub fn ordonare(array: &[i32], asc: bool) {
    // displayArray(&array);
    let mut cp = copy(&array);
    bubbleSort(&mut cp);
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