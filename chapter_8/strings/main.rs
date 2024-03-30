fn main() {
    
    // create instance of string which is empty
    let mut s = String::new();

    // initialize string with contents
    let data = "initial contents";  // start with string literal

    let s2 = data.to_string();

    // the method also works on a literal directly
    let s3 = "initial contents".to_string();

    // another way to construct string from literal directly
    let s4 = String::from("initial contents");

    // various utf-8 encoded forms of hello
    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("שָׁלוֹם");
    println!("{hello}");
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");
    println!("{hello}");

    println!("");  // add space

    // append string slice
    let mut s_mut = String::from("foo");
    s_mut.push_str("bar");
    println!("{s_mut}");

    // second string retains ownership
    let mut s_mut_1 = String::from("foo");
    let s5 = "bar";
    s_mut_1.push_str(s5);
    println!("s5 is {s5}");

    // push single letter onto string
    let mut s_mut_2 = String::from("lo");
    s_mut_2.push('l');
    println!("{s_mut_2}");

    // concatenate strings using + operator
    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    let s8 = s6 + &s7;  // note s1 has been moved here and can no longer be used
    println!("{s8}");

    // multiple concatenations
    let s9 = String::from("tic");
    let s10 = String::from("tac");
    let s11 = String::from("toe");

    let s12 = s9 + "-" + &s10 + "-" + &s11;
    println!("{s12}");

    let s14 = String::from("tic");

    // alternative way to combine
    let s13 = format!("{s14}-{s10}-{s11}");  // uses references so parameters retain ownership
    println!("{s13}");

    // this code causes ane error
    //let s14 = String::from("hello");
    //let h = s14[0];

    // get length of word
    let hello = String::from("Здравствуйте");
    println!("{}", hello.len());  // length is 24 since each character is 2 bytes

    // get length of word
    let text_word = "नमस्ते";
    println!("{}", text_word.len());

    // get slice of string
    let hello = "Здравствуйте";
    let s15 = &hello[0..4];  // get first 4 bytes
    println!("{}", s15);  // prints 2 characters each spanning 2 bytes

    // this code causes error since slices only part of character's bytes
    //let s15 = &hello[0..1]

    println!("");  // add space

    // iterate over chars of utf-8 word
    for c in "Зд".chars() {
        println!("{c}");
    }

    println!("");  // add space

    // iterate over bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
