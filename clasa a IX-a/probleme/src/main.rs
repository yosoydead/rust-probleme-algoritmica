// #[path="operatori si expresii/sum00/sum.rs"] mod sum;
// #[path="operatori si expresii/urare/urare.rs"] mod urare;
// #[path="operatori si expresii/scadere/scadere.rs"] mod scadere;
// #[path="operatori si expresii/asii/asii.rs"] mod asii;
// #[path="operatori si expresii/uciv/uciv.rs"] mod uciv;
// #[path="operatori si expresii/globuri/globuri.rs"] mod globuri;
// #[path="operatori si expresii/copii2/copii2.rs"] mod copii2;
// #[path="operatori si expresii/parc2/parc2.rs"] mod parc2;
// #[path="operatori si expresii/camioane/camioane.rs"] mod camioane;
#[path="sortarea vectorilor/ordonare/ordonare.rs"] mod ordonare;


fn main() {
    println!("Hello, world!");
    // sum::sum();
    // urare::urare();
    // scadere::scadere(50, 20);
    // asii::asii(10, 20);
    // uciv::uciv(25, 78);
    // globuri::globuri(7);
    // copii2::copii2(7, 5, 3);
    // parc2::parc2(3);
    // camioane::camioane(3, 5, 4, 2, 5);

    // let array: [u32; 20] = [438, 712, 229, 301, 61, 409, 948, 508, 901, 789, 388, 495, 848, 180, 376, 790, 700, 939, 436, 929];
    let array: [i32; 50] = [373, 171, 372, -671, 669, 455, 11, 12, -512, 297, 136, -195, 928, 75, -206, -177, 657, 734, 588, 872, -686, -324, 934, -659, -859, -399, 679, -194, 32, 175, -972, 81, 898, -799, -3, 859, 343, 182, -114, -855, 95, -70, 220, 801, -567, 812, 563, -116, -274, -511];
    ordonare::ordonare(&array, true);
}
