use crate::error::AppError;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn get_bool_input(prompt: &str) -> Result<bool, AppError> {
    use_input(
        Some(format!("{} y/n", prompt).as_str()),
        |line| match line.as_str() {
            "y" => Ok(true),
            "n" => Ok(false),
            _ => Err(AppError::InputError),
        },
    )
}

pub fn get_input(prompt: &str) -> Result<String, AppError> {
    use_input(Some(prompt), |line| Ok(line))
}

pub fn use_input<F, R>(prompt: Option<&str>, mut f: F) -> Result<R, AppError>
where
    F: FnMut(String) -> Result<R, AppError>,
{
    let mut rl = Editor::<()>::new();
    if let Some(text) = prompt {
        println!("{}", text);
    }
    let readline = rl.readline(">> ");
    match readline {
        Ok(line) => f(line),
        Err(ReadlineError::Interrupted) => {
            println!("CTRL-C");
            Err(AppError::InteruptionError)
        }
        Err(ReadlineError::Eof) => {
            println!("CTRL-D");
            Err(AppError::InteruptionError)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            Err(AppError::InteruptionError)
        }
    }
}
