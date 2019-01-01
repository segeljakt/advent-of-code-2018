use scan_fmt::*;
use std::io::Read;
use std::fs::File;

const N: usize = 6;

type Val = usize;
type Regs = [Val; N];
type Op = String;
type A = Val;
type B = Val;
type C = Val;

type InstructionPointer = usize;
struct Instruction(Op, A, B, C);

fn parse() -> (InstructionPointer, Vec<Instruction>) {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let mut input = buf.lines();
    let ip_reg = scan_fmt!(input.next().unwrap(), "#ip {}", usize).unwrap();
    let mut instrs = Vec::new();
    for line in input {
        let i = scan_fmt!(line, "{} {} {} {}", String, A, B, C);
        instrs.push(Instruction(i.0.unwrap(), i.1.unwrap(), i.2.unwrap(), i.3.unwrap()));
    }
    (ip_reg, instrs)
}

fn exec(mut r: Regs, Instruction(op, a, b, c): &Instruction) -> Regs {
    let (a, b, c) = (*a, *b, *c);
    match op.as_str() {
        "addr" => r[c] = r[a] + r[b],
        "addi" => r[c] = r[a] + b,
        "mulr" => r[c] = r[a] * r[b],
        "muli" => r[c] = r[a] * b,
        "banr" => r[c] = r[a] & r[b],
        "bani" => r[c] = r[a] & b,
        "borr" => r[c] = r[a] | r[b],
        "bori" => r[c] = r[a] | b,
        "setr" => r[c] = r[a],
        "seti" => r[c] = a,
        "gtir" => r[c] = if a > r[b] { 1 } else { 0 },
        "gtri" => r[c] = if r[a] > b { 1 } else { 0 },
        "gtrr" => r[c] = if r[a] > r[b] { 1 } else { 0 },
        "eqir" => r[c] = if a == r[b] { 1 } else { 0 },
        "eqri" => r[c] = if r[a] == b { 1 } else { 0 },
        "eqrr" => r[c] = if r[a] == r[b] { 1 } else { 0 },
        _ => unreachable!()
    };
    r
}

fn main() {

    let (ip_reg, instrs) = parse();
    let mut regs: Regs = [0; N];
    regs[0] = 1;
    let mut ip = 0;

    println!("#ip {}", ip_reg);
    for instr in instrs.iter() {
        println!("{} {} {} {} ", instr.0, instr.1, instr.2, instr.3);
    }
    println!("");

    //ip=3;
    //regs = [0,      0, 10551354,  3, 1, 10551354];

    //regs[5] = 10551354;
    while ip < instrs.len() {
        //if ip == 4 {
            //regs[5] = 10551355;
        //}
        let instr = &instrs[ip];
        regs[ip_reg] = ip;
        //print!("ip={}\t", ip);
        //for (x, reg) in regs.iter().enumerate() {
            //print!("r[{}] = {}\t", x, reg);
        //}
        //println!("");
        //print!("{:?} ", regs);
        //print!("{} {} {} {} ", instr.0, instr.1, instr.2, instr.3);
        regs = exec(regs, instr);
        ip = regs[ip_reg];
        //println!("{:?}", regs);
        ip += 1;
        //if regs[0] != 0 && regs[0] != 1 {
            //println!("{}", regs[0]);
        //}
    }
    println!("{:?}", regs);
}

