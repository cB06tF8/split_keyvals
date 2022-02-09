extern crate split_keyvals;

fn main() {
    let my_keyval = split_keyvals::split_str_str(String::from("mode=encode"), '=');
    match my_keyval {
        Ok(x) => println!("key: {:?} = val: {:?}", x.key, x.val),        
        Err(e) => println!("invalid key=value pair"),
    }
}