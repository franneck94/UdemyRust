#[derive(Debug)]
struct Student {
    name: String,
    grade: u32,
}

fn main() {
    let students = vec![
        "Jan 77",
        "Marie 65",
        "Dejan 49",
        "Pascal 100",
        "Lisa 80",
        "Malte 56",
    ];

    let good_student_threshold = 70;

    let good_students: Vec<Student> = students
        .iter()
        .map(|student_string| {
            let mut splitted_str = student_string.split(' ');
            let name = splitted_str.next()?.to_string();
            let grade = splitted_str.next()?.parse::<u32>().ok()?;

            Some(Student { name, grade })
        })
        .filter(|student| match student {
            Some(valid_student) => valid_student.grade >= good_student_threshold,
            None => false,
        })
        .flatten()
        .collect();

    println!("Good students: {:#?}", good_students);
}
