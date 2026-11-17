use std::{io::stdin, process::ExitCode};
use std::io::{self, Write};

fn main() -> ExitCode {
    // Load config files
    // Run command loop
    rush_loop();
    // Perform shutdown/cleanup
    ExitCode::SUCCESS //0
}

fn rush_loop() -> io::Result<()>{
    let mut line:String;
    let mut args:Vec<String> ;
    
    let mut status:u32 = 1;

    loop{
        print!(" ");
        io::stdout().flush()?;
        //line = rush_read_line();
        //args = rush_split_line(&line);
        //status = rush_execute(args);
        let mut buf = String::new();
        if stdin().read_line(&mut buf)? == 0 {break; } //EOF
        run(&buf);
    }
    Ok(())
}

fn run(buf:&str){
    println!("{buf}");
}

fn rush_read_line() -> String{
    todo!("reads a line, will be implemented");
    "hola mundo".to_string()

}

fn rush_split_line(_line: &str) -> Vec<String>{
    todo!("splits the line with args, to be implemented");
}

fn rush_execute(_args: Vec<String>) -> u32{
    todo!("executes the process");
}
