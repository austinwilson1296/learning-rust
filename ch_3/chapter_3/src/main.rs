fn return_name(name_list: [&str; 3], index: usize) -> &str {
    let name = name_list[index];
    name
}

fn main() {
    let names = ["Austin", "Logan", "Nicole"];
    let find_name = return_name(names, 0);
    println!("The name from the list is {find_name}");

    
    for element in names {
        println!("the value is: {element}");
    }
}
// Variables immutable by default unless specificed with mut
// const keyword not allowed to change, does not accept mut in declaration
// Can only be set to constant expression not result of value
// ex.. const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
