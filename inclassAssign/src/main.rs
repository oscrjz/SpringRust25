struct Student {
    major: String,
}

fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

fn update_majors(students: &mut [Student], behavior: fn(&mut Student, String)) {
    let mut count = 1;
    for student in students.iter_mut() {
        let m = format!("Major{}", count);
        behavior(student, m);
        count += 1;
    }
}

fn print_students(students: &[Student]) {
    for (i, s) in students.iter().enumerate() {
        println!("Student {}: {}", i + 1, s.major);
    }
}

fn main() {
    let mut students = vec![
        Student { major: "".to_string() },
        Student { major: "".to_string() },
        Student { major: "".to_string() },
    ];

    println!("before update:");
    print_students(&students);

    update_majors(&mut students, assign_major);

    println!("after update:");
    print_students(&students);
}
