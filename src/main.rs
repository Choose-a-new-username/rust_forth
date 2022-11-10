use std::{fs, env, io::Error, process::exit};

enum TokenType {
    Int,
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    DDrop,
    Swap,
    Rot,
    Over,
    Equal,
    Greater,
    Less,
    Dump,
    AsciiDump,
    If,
    Else,
    While,
    Do,
    End
}

type LexedWord = (String, String, usize, usize);
type Token = (TokenType, LexedWord, Option<u32>, Option<i32>);


fn parse(lexed: Vec<LexedWord>) -> Vec<Token> {
    let mut parsed: Vec<Token> = Vec::new();
    let mut lexed_iter = lexed.clone();
    let mut i: u32 = 0;
    let mut block_stack: Vec<u32> = Vec::new();
    let mut line: usize = 0;
    let mut comment: bool = false;

    loop {
        let word = if lexed_iter.len() > 0 { lexed_iter.remove(0) } else { break; };
        if word.2 != line {
            line = word.2;
            comment = false;
        }
        if comment {
            continue;
        }

        let push: Token = if word.0.parse::<i32>().is_ok() {
            (TokenType::Int, word.clone(), None, Some(word.0.parse().unwrap()))
        } else if word.0 == "true" {
            (TokenType::Int, word.clone(), None, Some(1))
        } else if word.0 == "false" {
            (TokenType::Int, word.clone(), None, Some(0))
        } else if word.0 == "+" {
            (TokenType::Add, word.clone(), None, None)
        } else if word.0 == "-" {
            (TokenType::Sub, word.clone(), None, None)
        } else if word.0 == "*" {
            (TokenType::Mul, word.clone(), None, None)
        } else if word.0 == "/" {
            (TokenType::Div, word.clone(), None, None)
        } else if word.0 == "=" {
            (TokenType::Equal, word.clone(), None, None)
        } else if word.0 == "<" {
            (TokenType::Less, word.clone(), None, None)
        } else if word.0 == ">" {
            (TokenType::Greater, word.clone(), None, None)
        } else if word.0 == "dup" {
            (TokenType::Dup, word.clone(), None, None)
        } else if word.0 == "drop" {
            (TokenType::DDrop, word.clone(), None, None)
        } else if word.0 == "swp" {
            (TokenType::Swap, word.clone(), None, None)
        } else if word.0 == "rot" {
            (TokenType::Rot, word.clone(), None, None)
        } else if word.0 == "over" {
            (TokenType::Over, word.clone(), None, None)
        } else if word.0 == "dump" {
            (TokenType::Dump, word.clone(), None, None)
        } else if word.0 == "asciidump" {
            (TokenType::AsciiDump, word.clone(), None, None)
        } else if word.0 == "if" {
            i += 1;
            block_stack.push(i);
            (TokenType::If, word.clone(), Some(i), None)
        } else if word.0 == "else" {
            (TokenType::Else, word.clone(), Some(block_stack[block_stack.len() - 1]), None)
        } else if word.0 == "while" {
            i += 1;
            block_stack.push(i);
            (TokenType::While, word.clone(), Some(i), None)
        } else if word.0 == "do" {
            block_stack.pop().unwrap_or_else(|| {
                eprintln!("{}/{}/{}/: Unexpected do statement", word.1, word.2, word.3);
                exit(1);
            });
            block_stack.push(i);
            (TokenType::Do, word.clone(), Some(i), None)
        } else if word.0 == "end" {
            (TokenType::End, word.clone(), Some(block_stack.pop().unwrap_or_else(|| {
                eprintln!("{}/{}/{}/: Unexpected end statement", word.1, word.2, word.3);
                exit(1);
            })), None)
        } else if word.0 == "rem" {
            comment = true;
            continue;
        } else {
            eprintln!("{}/{}/{}/: Invalid token {:?}", word.1, word.2, word.3, word.0);
            exit(1);
        };

        parsed.push(push);
        if lexed_iter.len() == 0 {
            if block_stack.len() > 0 {
                eprintln!("{}/{}/{}/: Expected \"end\", found {:?}", word.1, word.2, word.3, word.0);
                exit(1);
            }
            break;
        }
    }

    parsed
}

fn compile(tokens: Vec<Token>) -> String {
    let mut compiled: String = String::from(
"format ELF64 executable
entry start
segment readable executable
dump:
    enter 64, 0
    mov qword[rbp-56], rdi
    mov byte[rbp-16], 10
    mov qword[rbp-8], 1
.L:
    mov rcx, qword[rbp-56]
    mov rdx, -3689348814741910323
    mov rax, rcx
    mul rdx
    shr rdx, 3
    mov rax, rdx
    sal rax, 2
    add rax, rdx
    add rax, rax
    sub rcx, rax
    mov rdx, rcx
    mov eax, edx
    lea edx, [rax+48]
    mov eax, 32
    sub rax, qword[rbp-8]
    mov byte[rbp-48+rax], dl
    add qword[rbp-8], 1
    mov rax, qword[rbp-56]
    mov rdx, -3689348814741910323
    mul rdx
    mov rax, rdx
    shr rax, 3
    mov qword[rbp-56], rax
    cmp qword[rbp-56], 0
    jne .L
    mov eax, 33
    sub rax, qword[rbp-8]
    lea rdx, [rbp-48]
    lea rcx, [rdx+rax]
    mov rdx, qword[rbp-8]
    mov rsi, rcx
    mov rdi, 1
    mov rax, 1
    syscall
    leave
    ret
start:
");

    let mut block_stack: Vec<TokenType> = Vec::new();
    for token in tokens {
        let token_type: TokenType = token.0;
        let value = token.1;

        let push: String = match token_type {
            TokenType::Int => format!("    push {}\n", token.3.unwrap()),
            TokenType::Add => format!("    pop rbx\n    pop rax\n    add rax, rbx\n    push rax\n"),
            TokenType::Sub => format!("    pop rbx\n    pop rax\n    sub rax, rbx\n    push rax\n"),
            TokenType::Mul => format!("    pop rbx\n    pop rax\n    mul rbx\n    push rax\n"),
            TokenType::Div => format!("    pop rbx\n    pop rax\n    div rbx\n    push rax\n"),
            TokenType::Equal => format!("    mov rcx, 0\n    mov rdx, 1\n    pop rax\n    pop rbx\n    cmp rax, rbx\n    cmove rcx, rdx\n    push rcx\n"),
            TokenType::Greater => format!("    mov rcx, 0\n    mov rdx, 1\n    pop rax\n    pop rbx\n    cmp rax, rbx\n    cmovl rcx, rdx\n    push rcx\n"),
            TokenType::Less => format!("    mov rcx, 0\n    mov rdx, 1\n    pop rax\n    pop rbx\n    cmp rax, rbx\n    cmovg rcx, rdx\n    push rcx\n"),
            TokenType::Dup => format!("    pop rax\n    push rax\n    push rax\n"),
            TokenType::DDrop => format!("    pop rax\n"),
            TokenType::Swap => format!("    pop rax\n    pop rbx\n    push rax\n    push rbx\n"),
            TokenType::Rot => format!("    pop rax\n    pop rbx\n    pop rcx\n    push rax\n    push rcx\n    push rbx\n"),
            TokenType::Over => format!("    pop rax\n    pop rbx\n    push rbx\n    push rax\n    push rbx\n"),
            TokenType::Dump => format!("    pop rdi\n    call dump\n"),
            TokenType::AsciiDump => format!("    mov rax, 1\n    mov rdi, 1\n    mov rsi, rsp\n    mov rdx, 1\n    syscall\n    pop rax\n"),
            TokenType::If => {
                block_stack.push(token_type);
                format!("    pop rax\n    test rax, rax\n    jz .ENDIF_{}\n", token.2.unwrap())
            },
            TokenType::Else => {
                let a = block_stack.pop().unwrap();
                if !matches!(a, TokenType::If) {
                    eprintln!("{}/{}/{}/: Unmatched \"else\"", value.1, value.2, value.3);
                    exit(1);
                }
                block_stack.push(token_type);
                format!("    jmp .ENDELSE_{0}\n.ENDIF_{0}:\n", token.2.unwrap())
            },
            TokenType::While => {
                block_stack.push(token_type);
                format!(".WHILE_{}:\n", token.2.unwrap())
            },
            TokenType::Do => {
                if !matches!(block_stack.pop().unwrap(), TokenType::While) {
                    eprintln!("{}/{}/{}/: Unexpected do statement", value.1, value.2, value.3);
                    exit(1);
                }
                block_stack.push(token_type);
                format!("    pop rax\n    test rax, rax\n    jz .ENDWHILE_{}\n", token.2.unwrap())
            },
            TokenType::End => {
                let a = block_stack.pop().unwrap();
                match a {
                    TokenType::If => format!(".ENDIF_{}:\n", token.2.unwrap()),
                    TokenType::Else => format!(".ENDELSE_{}:\n", token.2.unwrap()),
                    TokenType::Do | TokenType::While => format!("    jmp .WHILE_{0}\n.ENDWHILE_{0}:\n", token.2.unwrap()),
                    _ => {
                        eprintln!("{}/{}/{}/: unexpected end statement", value.1, value.2, value.3);
                        exit(1);
                    }
                }
            },
        };

        compiled += &push;
    }

    compiled += "exit:\n    mov rax, 60\n    pop rdi\n    syscall";
    compiled
}

fn lex_line(line: &str) -> Vec<(String, usize)> {
    let mut ret: Vec<(String, usize)> = Vec::new();
    let mut chars = line.clone().to_string();
    let mut word = String::new();
    let mut i = 0;

    'findwords: loop {
        if chars.is_empty() {
            break;
        }

        while chars.chars().next().unwrap().is_ascii_whitespace() {
            chars.remove(0);
            i += 1;
            if chars.len() == 0 {
                break 'findwords;
            }
        }
        while !chars.chars().next().unwrap().is_ascii_whitespace() {
            word.push(chars.remove(0));
            i += 1;
            if chars.len() == 0 {
                let len = word.len();
                ret.push((word, i - len));
                break 'findwords;
            }
        }
        let len = word.len() - 1;
        ret.push((word, i - len));
        word = String::new();
    }

    ret
}

fn lex(text: &str, file_path: &str) -> Vec<LexedWord> {
    let mut lexed: Vec<LexedWord> = Vec::new();

    for (row, line) in text.lines().enumerate() {
        for (token, col) in lex_line(line) {
            lexed.push((token.clone(), file_path.to_string(), row + 1, col));
        }
    }

    lexed
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file: String = fs::read_to_string(file_path)?;

    println!("{}", compile(parse(lex(&file, file_path))));
    Ok(())
}
