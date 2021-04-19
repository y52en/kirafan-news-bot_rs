// use std::fs;
// use std::io::Write;


// pub fn main(path: &String) -> Result<(), Box<std::error::Error>> {
//     if !is_pathExist(&path) {
//         fs::File::create(path).unwrap();
//     }
//     let content = fs::read_to_string("a.txt")?;
//         println!("{}", content);
//         Ok(())
// }

// fn is_pathExist(path: &String) -> bool {
//     Path::new(path).exists()
// }

