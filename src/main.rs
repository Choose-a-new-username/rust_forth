use std::{fs, env, io::Error, process::exit};

#[derive(Debug)]
enum TokenType {
    Int,
    Add,
    Sub,
    Dump
}

type Token = (String, TokenType);
type LexedWord<'a> = (String, usize, usize);

fn parse(lexed: Vec<LexedWord>) -> Vec<Token> {
    let mut parsed: Vec<Token> = Vec::new();

    for word in lexed {
        let token_type = if word.0.parse::<i32>().is_ok() {
            TokenType::Int
        } else if word.0 == "+" {
            TokenType::Add
        } else if word.0 == "-" {
            TokenType::Sub
        } else if word.0 == "." {
            TokenType::Dump
        } else {
            eprintln!("file_name/{}/{}/: Invalid token {}", word.1, word.2, word.0);
            exit(1);
        };

        parsed.push((word.0.clone(), token_type));
    }
    parsed
}

fn compile(tokens: Vec<Token>) -> String {
    let mut compiled: String = String::from(
"format ELF64 executable
entry start
segment readable executable
print:
    mov rcx, 69
    call dec2str
    mov rax, 1
    mov rsi, rdi
    mov rdi, 1
    mov rdx, 2
    syscall

dec2str:
    .stack_dec:
        xor rdx, rdx
        div rcx
        add rdx, '0'
        push rdx
        test rax, rax
        jz .purge_dec
        call .stack_dec
    .purge_dec:
        pop [rdi]
        inc rdi
        ret
start:\n");

    for token in tokens {
        let push: Option<String> = match token.1 {
            TokenType::Int => Some(format!("    push {}\n", token.0)),
            TokenType::Add => Some(format!("    pop rax\n    pop rbx\n    add rax, rbx\n    push rax\n")),
            TokenType::Dump => Some(format!("    call print\n")),
            _ => None
        };

        if let Some(s) = push {
            compiled += &s;
        }
    }
    compiled += "mov rax, 60\npop rdi\nsyscall";
    compiled
}

fn lex_line(line: &str) -> Vec<(String, usize)> {
    let mut ret: Vec<(String, usize)> = Vec::new();
    let mut chars = line.clone().to_string();
    let mut word = String::new();
    let mut i = 0;

    'findwords: loop {
        while chars.chars().collect::<Vec<char>>()[0].is_ascii_whitespace() {
            chars.remove(0);
            i += 1;
            if chars.len() == 0 {
                break 'findwords;
            }
        }
        while !chars.chars().collect::<Vec<char>>()[0].is_ascii_whitespace() {
            word.push(chars.remove(0));
            i += 1;
            if chars.len() == 0 {
                let len = word.len();
                ret.push((word, i - len));
                break 'findwords;
            }
        }
        let len = word.len();
        ret.push((word, i - len));
        word = String::new();
    }

    ret
}

fn lex(text: &str) -> Vec<LexedWord> {
    let mut lexed: Vec<LexedWord> = Vec::new();

    for (row, line) in text.lines().enumerate() {
        for (token, col) in lex_line(line) {
            lexed.push((token.clone(), row, col));
        }
    }

    lexed
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file: String = fs::read_to_string(file_path)?;

    println!("{}", compile(parse(lex(&file))));
    Ok(())
}
