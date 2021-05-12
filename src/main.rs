fn notString (any_string:String) -> String {
    //
    if (any_string.len() < 3) || (&any_string [..3] != "not") {
            format!("not {}",any_string)
    }else {
        any_string.to_string()
    }
}

fn main() {
    let mut new_string= "ahmed";
    println!("{}", notString (new_string.to_string()));
}
