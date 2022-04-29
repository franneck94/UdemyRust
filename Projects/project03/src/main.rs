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

    let good_students: Vec<Student> = students
        .iter()
        .map(|s| {
            let mut s = s.split(' ');
            let name = s.next()?.to_string();
            let grade = s.next()?.parse::<u32>().ok()?;

            Some(Student { name, grade })
        })
        .filter(|s| match s {
            Some(s) => s.grade >= 70,
            None => false,
        })
        .flatten()
        .collect();

    println!("{:#?}", good_students);
}
