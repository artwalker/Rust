    // --------------------------------------
    //          Comments and its Different Styles
    //          Program Ouputs
    // --------------------------------------

fn main() {
    // This is a single-line comment.
    // This is another single-line comment.

   /* This is a multi-line comment.
      This is another line of the comment. */
   
    // This is a printing function
    println!("--- This is a printing function ---");
    println!("Hello from rust program.");
    print!("{}", 2);
    println!();
    print!("Hi, my friend! This is a print statement.");
    println!();
    println!("Hi, my friend! This is a println statement.");
    println!("-----------------------------------\n");

   // comments inside a command
    println!("--- comments inside a command ---");
    print/*ln*/!("Hello, world!");
    println!("\n----------------------------------\n");

    // learning some basic output commands
    println!("--- learning some basic output commands ---");
    println!("The value of the constant is {}",10);
    println!("-------------------------------------------\n");

    // Learning to print strings 
    println!("--- Learning to print strings ---");
    println!("My first name is {} and my last name is {}","Ethan", "Wang");
    println!("----------------------------------\n");

    // Learning the print command 
    println!("--- Learning the print command ---");
    print!("This is a print command "); 
    print!("This is going to be printed on the same line");
    println!("-----------------------------------\n");

    // Learing to write on multiple lines
    println!("--- Learing to write on multiple lines ---");
    print!("\nThis is going to be
            Printed on multiple 
            line");
    println!("\n------------------------------------------\n");
        
    // Learning the use of escape sequences 
    println!("--- Learning the use of escape sequences ---");
    println!("\\n\nThis is going to be printed after two lines. \t This will have a tab before");
    println!("----------------------------------------------\n");

    // Learning somes uses of back slash.
    println!("--- Learning somes uses of back slash. ---");
    println!("This will print single quote \' and this double quotes \"");
    println!("This is going to print one back slash \\") ;
    print!("This is some text which will be overwritten \r this text will only appear on the screen");
    println!("\n-------------------------------------------\n");

    // Learning Positional Arguments
    println!("--- Learning Positional Arguments ---");
    println!("I doing {2} from {1} years and I {0} it","like",1000,"programming"); 
    println!("-------------------------------------\n");

    // learning Named Arguments 
    println!("--- learning Named Arguments ---");
    println!("{language} is a system progrmaming language which is cool to {activity} in.", language="Rust", activity = "code");
    println!("---------------------------------\n");

    // learning to print basci maths 
    println!("--- learning to print basic maths ---");
    println!("The summation of 25 + 10 = {}", 25 +10);
    println!("-------------------------------------\n");
}