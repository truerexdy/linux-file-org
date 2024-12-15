//Don't assume that this is AI generated by seeing the comments
//I am used to writing comments, it's helps me understand the code better

use std::fs;
use std::env;
fn main() {
    /*Here I'm declaring a Vec<String> named args to hold the arguements.
    std::env module is used to collece command line arguements*/
    let args: Vec<String> = env::args().collect();
    /*For my case I just need one arguement i.e. the directory path nothing else so
    I will check for that constraint*/
    
    if args.len() != 2{ //2 becaue command itself + the arguement
        println!("Invalid number of command line arguements {}",args.len());
        std::process::exit(0);
        //this is simillar to exit(0) of stdlib in C
    }

    let path: &str = &args[1];
    /*Now this will hold the path of the directory, index is 1 because
    0th index will have the command name itself*/

    let mut files: Vec<String> = Vec::new();
    //A vector of string to hold the file names

    match fs::read_dir(path){
        Ok(entries)=>{ // Entries is a Vec<DirEntry>
            for entry in entries{ //DirEntry is a struct defined in std::fs
                match entry{
                    Ok(entry)=>{
                        files.push(entry.file_name().into_string().unwrap());
                        //unwrap returns string if valid(is UTF-8), Err if invalid(not UTF-8)
                    }
                    Err(e)=>{
                        println!("Error reading the file {}", e);
                    }
                }
            }
        }
        Err(e)=>{
            println!("Error reading directory {} \n {}", path, e);
        }
    }

    println!("Here are the files in the directory {path}");
    for file in files{
        print!("{file}\n");
    }
    
}
