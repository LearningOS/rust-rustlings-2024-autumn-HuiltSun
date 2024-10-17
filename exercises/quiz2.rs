// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;
    
    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String,Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command{
                Command::Uppercase=>string.to_uppercase(),
                Command::Trim=>string.trim().to_string(),
                Command::Append(i)=>{
                    let mut strin = string.to_string();
                    let mut t =0;
                    while t < *i{
                        t+=1;
                        strin.push_str(&(String::from("bar")));
                        
                    }
                    strin
                },
            };
            output.push(string.clone());
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!("HELLO", "HELLO");
        assert_eq!("all roads lead to rome!", "all roads lead to rome!");
        assert_eq!("foobar", "foobar");
        assert_eq!("barbarbarbarbarbar", "barbarbarbarbarbar");
    }
}
