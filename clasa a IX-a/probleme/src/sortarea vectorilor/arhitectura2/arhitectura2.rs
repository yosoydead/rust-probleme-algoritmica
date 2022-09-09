/*
Cerința
După ce arhitectul Gigel a reușit să rezolve sarcina primită de la primărie ( #Arhitectura ), el și-a dat seama că aspectul zonei nu va fi unul după preferințele sale. Astfel, s-a stabilit că în oraș nu există clădiri cu înălțimea mai mare decât hmax. Acum primăria îi cere afișarea clădirilor în ordine descrescătoare, precum și verificarea, pentru fiecare clădire din lista ordonată, dacă înălțimea ei este egală cu media aritmetică a înălțimilor celor două clădiri vecine.

Gigel vă cere ajutorul ca să-și termine proiectul care a devenit mult prea greu .

Date de intrare
Fișierul de intrare arhitectura2.in conține pe prima linie numărul n, iar pe următoarele linii n numere naturale separate prin spații. Fiecare linie conține maxim 100.000 de valori.

Date de ieșire
Fișierul de ieșire arhitectura2.out va conține doua linii cu n valori fiecare. Prima linie va conține înălțimile în ordine descrescătoare , iar a doua linie va conține n valori 1 sau 0, după cum înălțimea clădirii curente este sau nu egală cu media aritmetică a înălțimilor celor două clădiri vecine.

Restricții și precizări
1 ≤ n ≤ 2000000
hmax ≤ 100000
Pentru 40% din teste n ≤ 50000
Pentru 80% din teste n ≤ 500000
Considerăm că înaintea primei clădiri și după ultima clădire se află câte o pseudoclădire de înălțime 0 – care va interveni în verificarea cerută.

Exemplu
arhitectura2.in

10
5 10 10 9 5 2 5 8 5 2
arhitectura2.out

10 10 9 8 5 5 5 5 2 2 
0  0  1 0 0 1 1 0 0 0
Explicatie:
Șirul devine 10 10 9 8 5 5 5 5 2 2
Doar 9 si cei doi de 5 din mijloc respectă condiția.
*/
pub fn arhitectura2() {
    let mut arr = [5, 10, 10, 9, 5, 2, 5, 8, 5, 2];

    let mut currentIndex = 0;

    while (currentIndex < arr.len()) {
        let biggestIndex = biggest(&arr[currentIndex..]);

        let temp = arr[currentIndex];
        arr[currentIndex] = arr[currentIndex+biggestIndex];
        arr[currentIndex+biggestIndex] = temp;

        currentIndex += 1;
    }

    println!("{:?}", arr);
    calculeazaVecini(&arr);
}

fn biggest(arr: &[i32]) -> usize{
    let mut resultIndex: usize = 0;
    let mut currBiggest: i32 = std::i32::MIN;

    for i in 0..arr.len() {
        if (arr[i] > currBiggest) {
            currBiggest = arr[i];
            resultIndex = i;
        }
    }

    return resultIndex;
}

fn calculeazaVecini(arr: &[i32]) {
    for i in 0..arr.len() {
        if (i == 0 || i == arr.len() - 1) {
            print!("{} ", 0);
        } else {
            let x = (arr[i - 1] + arr[i+1]) / 2;
            let output = if arr[i] == x { 1 } else { 0};
            print!("{} ", output);
        }

    }
}