use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct NameRoster {
   user_name: String,
   age: i32,
}

static USER_MAP: Lazy<Mutex<HashMap<String, NameRoster>>> =
   Lazy::new(|| Mutex::new(HashMap::new()));

impl NameRoster {
   fn new(name: String, age: i32) -> Self {
      let roster = NameRoster {
         user_name: name.clone(),
         age: age,
      };
      USER_MAP.lock().unwrap().insert(name, roster.clone());
      roster
   }

   fn for_name(name: &String) -> Option<NameRoster> {
      let user_map = USER_MAP.lock().unwrap();
      if user_map.contains_key(name) {
         return Some(user_map[name].clone());
      }
      None
   }

   fn get_user_name() -> String {
      let mut user_name = String::new();
      stdin()
         .read_line(&mut user_name)
         .expect("Unable to read user_name\n");
      user_name.trim().to_string()
   }

   fn get_user_age() -> Option<i32> {
      let mut user_age_str = String::new();
      let read_age = stdin().read_line(&mut user_age_str);
      match read_age {
         Ok(_) => {
            let age_val = user_age_str.trim().parse::<i32>();
            return match age_val {
               Ok(v) => Some(v),
               _ => None,
            };
         }
         _ => return None,
      }
   }
}

fn add_to_roster(name: String, age: i32) {
   NameRoster::new(name, age);
}

fn age_for_name(name: String) -> i32 {
   let roster = NameRoster::for_name(&name);
   match roster {
      Some(roster) => roster.age,
      None => -1,
   }
}

#[cxx::bridge]
mod testcpp {
   extern "Rust" {
      type NameRoster;
      fn add_to_roster(name: String, age: i32);
      fn age_for_name(name: String) -> i32;
   }
   unsafe extern "C++" {
      include!("rustcppinterop/include/test.h");
      type Test;
      fn new_test() -> UniquePtr<Test>;
   }
}

const NAMES_TO_COLLECT: i32 = 5;
fn main() {
   let _test = testcpp::new_test();

   let mut i: i32 = 0;
   loop {
      if i >= NAMES_TO_COLLECT {
         break;
      }
      print!("Please enter name #{}: ", i);
      stdout().flush().expect("Unable to flush stdout.");
      let user_name = NameRoster::get_user_name();
      print!("Please enter age #{}: ", i);
      stdout().flush().expect("Unable to flush stdout.");
      let user_age = NameRoster::get_user_age();
      match user_age {
         Some(age) => {
            NameRoster::new(user_name, age);
            i += 1;
            ()
         }
         None => {
            println!("Invalid numeric age provided, try again.");
         }
      }
   }

   print!("Name to find in list: ");
   stdout().flush().expect("Unable to flush stdout.");
   let name_to_find = NameRoster::get_user_name();
   let roster = NameRoster::for_name(&name_to_find);
   match roster {
      Some(roster) => println!("Name {} found, age {}!", name_to_find, roster.age),
      None => println!("Name {} not found in roster.", name_to_find),
   }
}
