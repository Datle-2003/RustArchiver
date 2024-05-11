// use std::{collections::HashMap};

// mod lib;
// use lib::arguments::Arguments;

// fn main() {
//     // take all args from the command line
//     let args: Vec<String> = std::env::args().collect();

//     if args.len() < 3 {
//         Arguments::help();
//         std::process::exit(1);
//     }

//     let mut params = HashMap::new();

//     for arg in &args[1..] {
//         let parts: Vec<&str> = arg.split('=').collect();
//         if parts.len() != 2 {
//             Arguments::help();
//         }
//         params.insert(parts[0], parts[1]);
//     }

//     let mut arguments = Arguments::new();

//     for (key, value) in params {
//         match key {
//             "source" => arguments.set_source(value),
//             "destination" => arguments.set_destination(value),
//             "format" => arguments.set_format(value),
//             "password" => arguments.set_password(value),
//             "action" => arguments.set_action(value),
//             _ => Arguments::help(),
//         }
//     }








// }
