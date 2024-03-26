fn main() {
    // Example 1
    // {
    //     let x = 1337; // 0xAA
    //     println!("{x}");

    //     {
    //         let x = -10; // 0xBB
    //         println!("{x}");
    //     }

    //     println!("{x}");
    // }
    // Example 2
    // {
    //     let mut x = 1337; // 0xAA
    //     println!("{x}");

    //     x = 0; // 0xAA
    //     println!("{x}");

    //     x = 10;
    //     println!("{x}");
    // }
    // // Example 3
    {
        let x = 1337; // 0xAA
        println!("{x}");

        let x = 0; // 0xBB
        println!("{x}");

        // x = 10;
    }
}
