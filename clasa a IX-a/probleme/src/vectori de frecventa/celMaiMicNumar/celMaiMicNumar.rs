/*
Problema Cel mai mic numar
Cerință

Se dă un număr natural n. Să se afișeze cel mai mic număr posibil care se poate obține prin a rearanja cifrele lui n.
Date de intrare

Programul citește de la tastatură numărul natural n.
Date de ieșire

Programul afișează pe ecran cel mai mic număr care se poate obține prin rearanjarea cifrelor lui n.
Restricții și precizări

    0 ≤ n ≤ 1018
    Cifrele de 0 nu se pot pune la începutul numărului

Exemple

9412

1249

123

123

Explicația exemplelor

Pentru primul exemplu: din cifrele 9, 4, 1, 2, cel mai mic număr care se poate forma este 1249.

Pentru al doilea exemplu: numărul 123 este deja cel mai mic număr care se poate forma din cifrele 1, 2, 3.
*/
pub fn celMaiMicNumar() {
    let n: u32 = 9412;
    
    let mut vec: Vec<u8> = Vec::new();
    adaugaCifre(n, &mut vec);

    // for i in 0..size {
    //     vec[i] = i as u32;
    // }
    sort(&mut vec, true);
    println!("n = {}. cel mai mic nr {:?}", n, createNum(&vec));
}

pub fn celMaiMareNumar() {
    let n = 2149;

    let mut vec: Vec<u8> = Vec::new(); 
    adaugaCifre(n, &mut vec);
    sort(&mut vec, false);
    println!("n = {} .cel mai mare nr {:?}", n, createNum(&vec));

}

fn adaugaCifre(n: u32, arr: &mut Vec<u8>) {
    let mut i: usize = 0;
    let mut cp = n;

    while (cp != 0) {
        arr.push((cp % 10) as u8);
        cp = cp / 10;
        i += 1;
    }
}

fn sort(arr: &mut Vec<u8>, asc: bool) {
    let mut index = 1;

    while (index < arr.len()) {
        let mut j = index;

        if asc == true {
            while (j > 0 && arr[j-1] > arr[j]) {
                let temp = arr[j-1];
                arr[j-1] = arr[j];
                arr[j] = temp;
    
                j -= 1;
            }
        } else {
            while (j > 0 && arr[j-1] < arr[j]) {
                let temp = arr[j-1];
                arr[j-1] = arr[j];
                arr[j] = temp;
    
                j -= 1;
            }
        }

        index += 1;
    }
}

fn createNum(arr: &Vec<u8>) -> u32 {
    let mut result: u32 = 0;
    for i in 0..arr.len() {
        result = (result * 10) + (arr[i] as u32);
    }
    return result;
}