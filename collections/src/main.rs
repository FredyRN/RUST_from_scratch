fn main() {
    println!("Collections: tuples");
    println!("*-------------------*");
    let tuple_a = ('W', 5i32, true, "WORD");
    // Tuple start at <<0>> index.
    println!("First tuple element is {}", tuple_a.0);
    println!("Tuple elements, NO cicle {} {} {} {}", tuple_a.0, tuple_a.1, tuple_a.2, tuple_a.3);
    println!("Collections: structs");
    println!("*-------------------*");
    // Clasic struct
    struct Student { name: String, level: u8, remote: bool }
    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32);
    // Unit struct
    struct Unit;
    let student_1: Student = Student { name: String::from("F3deR1c0"), remote: true, level:3 };
    let student_2: Student = Student { name: String::from("Consuelo"), level: 4, remote: false };
    //
    let mark_1: Grades = Grades('A', 'A', 'A', 'B', 4.25);
    let mark_2: Grades = Grades('B', 'B', 'A', 'C', 3.45);
    //
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
              student_1.name, student_1.level, student_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
              println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
              student_2.name, student_2.level, student_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
}
