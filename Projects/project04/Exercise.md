# Student Grading Exercise

## Objective

Create a program that parses a list of student names and grades, identifies good students based on a threshold grade, and prints their information.  
This exercise aims to reinforce knowledge of Rust's iterators, error handling, and data structuring.

## Instructions

### Define Student Struct

Create a Student struct with fields for the students name and grade.

### Parse Student Data

In the main function, initialize a vector of students containing strings in the format "Name Grade", where Name is the student's name and Grade is the student's grade.

### Set Good Student Threshold

Define a threshold grade (good_student_threshold) to determine what constitutes a "good" student.  
This value should be customizable by a threshold variable.

### Parse Student Information

Use iterators and closures to parse each string in the student's vector into a Student struct.  
Split each string by whitespace and extract the name and grade components.  
Convert the grade to a u32 using parse. Handle potential errors gracefully.

### Filter Good Students

Filter the parsed students to identify those whose grades meet or exceed the good_student_threshold.  
Use the filter method with a closure to perform the filtering.

### Flatten and Collect

Flatten the filtered iterator of Option<Student> to remove the Option wrapper and collect the valid students into a vector of Student structs.

### Print Results

Print the information of the good students, including their names and grades.

#### Example Input

```rust
    let students = vec![
        "Jan 77",
        "Marie 65",
        "Dejan 49",
        "Pascal 100",
        "Lisa 80",
        "Malte 56",
    ];

    let good_student_threshold = 70;
```

#### Example Output

```rust
Good students: [
    Student { name: "Jan", grade: 77 },
    Student { name: "Pascal", grade: 100 },
    Student { name: "Lisa", grade: 80 },
]
```

Happy coding!
