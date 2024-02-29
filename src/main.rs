use std::io::{self, Write, Read, Stdin};

#[derive(Debug)]
enum OpCode {
    IncPtr,          // >
    DecPtr,          // <
    IncVal,          // +
    DecVal,          // -
    Write,           // .
    Read,            // ,
    JmpFront(usize), // [
    JmpBack(usize)   // ]
}

impl TryFrom<char> for OpCode {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
	use OpCode::*;

	match c {
	    '>' => Ok(IncPtr),
	    '<' => Ok(DecPtr),
	    '+' => Ok(IncVal),
	    '-' => Ok(DecVal),
	    '.' => Ok(Write),
	    ',' => Ok(Read),
	    '[' => Ok(JmpFront(0)),
	    ']' => Ok(JmpBack(0)),
	    _ => Err(())
	}
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1)
        .ok_or("Takes a bf file as first argument")?;
    
    let contents = std::fs::read_to_string(filename)?;
    let code = parse_bf(&contents)?;

    exec(&code)?;
    
    Ok(())
}
	
fn parse_bf(input: &str) -> Result<Vec<OpCode>, &str> {
    let code : Vec<OpCode> = input.chars()
	.filter_map(| c | c.try_into().ok())
	.collect();

    let mut stack = vec![];
    let mut fixes = vec![];
    
    for (addr, opcode) in code.iter().enumerate() {
	match opcode {
	    OpCode::JmpFront(_) => { stack.push(addr) },
	    OpCode::JmpBack(_) => {
		let to = stack.pop().ok_or("Bad loop")?;
		fixes.push((to, OpCode::JmpFront(addr)));
		fixes.push((addr, OpCode::JmpBack(to)));
	    },
	    _ => {}
	}
    }

    let mut code = code;
    for (addr, opcode) in fixes {
	code[addr] = opcode;
    }
    
    Ok(code)
}

fn exec(code: &[OpCode]) -> Result<(), io::Error> {
    use OpCode::*;
    let mut mem = vec![0_u8 ; 30_000];
    let mut ptr : usize = 0;
    let mut pc : usize = 0;
    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();
    
    while pc < code.len() {
	match code[pc] {
	    IncPtr => ptr = (ptr + 1) % mem.len(),
	    DecPtr => ptr = if ptr == 0 { mem.len()-1 } else { ptr-1 },
	    IncVal => mem[ptr] = mem[ptr].wrapping_add(1),
	    DecVal => mem[ptr] = mem[ptr].wrapping_sub(1),
	    Write => stdout.write_all(&mem[ptr..ptr+1])?,
	    Read => mem[ptr] = read_value(&mut stdin)?,
	    JmpFront(to) if mem[ptr] == 0 => pc = to,
	    JmpBack(to) if mem[ptr] != 0 => pc = to,
	    _ => {}
	}
	pc += 1;
    }
    Ok(())
}

fn read_value(stdin: &mut Stdin) -> Result<u8, io::Error> {
    let mut input: [u8; 1] = [0; 1];
    stdin.read_exact(&mut input)?;
    Ok(input[0])
}
