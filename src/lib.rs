use std::error::Error;
use std::fs;
use std::env;

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&input.file)?;
    

    let results = if input.case_sensitive {
        search_case_sensitive(&input.query, &contents)
    } 
    else {
        search_case_insensitive(&input.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

fn search_case_sensitive<'a,'b>(query: &'a str, contents: &'b str) -> Vec<&'b str> {
    let mut result:Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

fn search_case_insensitive<'a,'b>(query: &'a str, contents: &'b str) -> Vec<&'b str> {
    let mut result:Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

pub struct Input {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, String> {
        if args.len() != 3 {
            return Err(
                "You need to enter two parameters: 1. search string, 2. filename".to_string()
            );
        }

        let query = &args[1];
        let file = &args[2];

        let case_sensitive:bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Input {
            query: query.clone(),
            file: file.clone(),
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_one_result() {
        let query = "ree";
        let contents = "\
One two:
Three Four
Five Six
AGREE";
        assert_eq!(vec!["Three Four"], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive_two_results() {
        let query = "rUsT";
        let contents = "\
Trust Me:
One two:
Three Four
Five Six
Rusty";
        assert_eq!(vec!["Trust Me:", "Rusty"], search_case_insensitive(query, contents));
    }
}
