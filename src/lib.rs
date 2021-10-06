use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let program = Program::new(&contents);
    let output = program.run(config.input);
    println!("{}", output);
    Ok(())
}

pub struct Config<'a> {
    filename: &'a str,
    input: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments.")
        }

        let filename = &args[1];
        let input = &args[2];

        Ok(Self { filename, input})
    }
}

struct Program<'a> {
   replacements: Vec<(&'a str, &'a str)>
}

impl<'a> Program<'a> {
    fn new(text: &'a str) -> Self {
        let mut replacements = vec![];
        for line in text.lines() {
            if line.chars().next() == Some('#') {
                continue
            }
            let mut split =  line.split('=');
            let find = split.next().expect("Invalid left side in program");
            let replace = split.next().expect("Invalid right side in program");
            replacements.push((find, replace));
        }
        Self {replacements}
    }

    fn run(&self, input: &str) -> String {
        
        let mut input = input.to_string();
        loop {
            // Ugly workaround for python for/else...
            let mut replaced = false;
            for (find, replace) in &self.replacements {
                if let Some(_) = input.find(find) {
                    input = input.replacen(find, replace, 1);
                    replaced = true;
                    break
                }
            }
            if !replaced {
                break
            }
        }

        input
    }
}

#[cfg(test)]
mod tests {
    use crate::Program;

    // Levels from A=B game
    #[test]
    fn atob_1_1() {
        let program = Program::new("a=b");
        let input = "abc";
        let output = "bbc";
        assert_eq!(program.run(input), output)
    }

    #[test]
    fn atob_1_2() {
        let program = Program::new("\
a=A
b=B
c=C");
        let input = "abc";
        let output = "ABC";
        assert_eq!(program.run(input), output)
    }

    #[test]
    fn atob_1_3() {
        let program = Program::new("\
aa=a
bb=b
cc=c");
        let input = "aabccca";
        let output = "abca";
        assert_eq!(program.run(input), output)
    }

    #[test]
    fn atob_1_4() {
        let program = Program::new("\
aaa=aa
aa=");
        let input = "aaabacaa";
        let output = "bac";
        assert_eq!(program.run(input), output)
    }

    #[test]
    fn atob_1_5() {
        let program = Program::new("\
ab=
ba=
aa=a
bb=b");
        let input = "aabbaa";
        let output = "a";
        assert_eq!(program.run(input), output)
    }

    #[test]
    fn atob_1_6() {
        let program = Program::new("\
ba=ab
cb=bc
ca=ac");
        let input = "caba";
        let output = "aabc";
        assert_eq!(program.run(input), output)
    }

    // Custom tests
    #[test]
    fn infinite_loop() {
        let program = Program::new("\
a=b
b=a");
        let input = "a";
        let output = "c";
        assert_eq!(program.run(input), output)
    }
}
