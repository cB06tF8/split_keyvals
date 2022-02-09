use std::env;

#[derive(Debug)]
pub struct KeyVal<T1, T2> {
    pub key: T1,
    pub val: T2,
}

// splits a string to string mapping into a KeyVal struct
pub fn split_str_str(keyval_pair: String, split_char: char) -> Result<KeyVal<String, String>, &'static str> {
    let retval = &keyval_pair.chars().position(|c| c== split_char);
    match retval {
        Some(x) => {
            let k = &keyval_pair[0..*x];
            let v = &keyval_pair[*x + 1..];
            let key_val:KeyVal<String, String> = KeyVal{key: k.to_string(), val: v.to_string()};
            Ok(key_val)
        },
        None => { 
            return Err("None")
        }
    }
}

// Splits a string to u32 number mapping into KeyVal struct
pub fn split_str_u32(keyval_pair: String, split_char: char) -> Result<KeyVal<String, u32>, &'static str> {
    let retval = &keyval_pair.chars().position(|c| c== split_char);
    match retval {
        Some(x) => {
            let k = &keyval_pair[0..*x];
            let v = &keyval_pair[*x + 1..];
            let vnum:u32 = v.parse().unwrap();
            let key_val:KeyVal<String, u32> = KeyVal{key: k.to_string(), val: vnum};
            Ok(key_val)
        },
        None => { 
            return Err("None")
        }
    }
}
