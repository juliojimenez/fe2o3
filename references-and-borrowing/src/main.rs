fn main() {
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}", s1, len);
    }
    {
        let s = String::from("hello");
        change(&s);
    }
    {
        let mut s = String::from("hello");
        change_mut(&mut s);
    }
    {
        // let mut s = String::from("hello");
        // let r1 = &mut s;
        // let r2 = &mut s;
        // println!("{}, {}", r1, r2);
        // error[E0499]: cannot borrow `s` as mutable more than once at a time
        //   --> src/main.rs:18:18
        //    |
        // 17 |         let r1 = &mut s;
        //    |                  ------ first mutable borrow occurs here
        // 18 |         let r2 = &mut s;
        //    |                  ^^^^^^ second mutable borrow occurs here
        // 19 |         println!("{}, {}", r1, r2);
        //    |                            -- first borrow later used here
    }
    {
        let mut s = String::from("hello");
        let _r1 = &mut s;
        {
            let _r2 = &mut s;
        }
    }
    {
        // let mut s = String::from("hello");
        // let r1 = &s;
        // let r2 = &s;
        // let r3 = &mut s;
        // println!("{}, {}, and {}", r1, r2, r3);
        // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
        //   --> src/main.rs:41:18
        //    |
        // 39 |         let r1 = &s;
        //    |                  -- immutable borrow occurs here
        // 40 |         let r2 = &s;
        // 41 |         let r3 = &mut s;
        //    |                  ^^^^^^ mutable borrow occurs here
        // 42 |         println!("{}, {}, and {}", r1, r2, r3);
        //    |                                    -- immutable borrow later used here
    }
    {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
        let r3 = &mut s;
        println!("{}", r3);
    }
    {
        let reference_to_nothing = no_dangle();
        println!("{}", reference_to_nothing);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(_some_string: &String) {
    // some_string.push_str(", world!");
    // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    //   --> src/main.rs:18:5
    //    |
    // 18 |     some_string.push_str(", world!");
    //    |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    //    |
    // help: consider changing this to be a mutable reference
    //    |
    // 17 | fn change(some_string: &mut String) {
    //    |                         +++
}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
// error[E0106]: missing lifetime specifier
//   --> src/main.rs:90:16
//    |
// 90 | fn dangle() -> &String {
//    |                ^ expected named lifetime parameter

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
