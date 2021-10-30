use crate::shared_functions::is_piped;

/// Just like you know it. Takes the args part of `ShellCommand` and prints them.
pub fn echo(args: &[String]) -> String {
    let mut output = String::new();
    is_piped(&args, "echo");
    for arg in args {
        output.push_str(format!("{} ", arg).as_str());
    }
    output
}
