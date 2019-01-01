use scan_fmt::*;
use std::io::Read;
use std::fs::File;
use left_pad::leftpad;

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
    optimized();
    //regular();
}

fn regular() {

    let (ip_reg, instrs) = parse();

    print_instrs(ip_reg, &instrs);
    let mut regs: Regs = [0; N];
    regs[0] = 16777215;
              
    let mut ip = 0;
    let mut executions = 0;
    //print!("{:?}", regs);

    let mut max_reg_2 = 0;
    while ip < instrs.len() {
        let instr = &instrs[ip];
        regs[ip_reg] = ip;
        //if ip == 28 {
            //if max_reg_2 < regs[2] {
                //println!("{}", regs[2]);
                //max_reg_2 = regs[2];
                //println!("{:?} => {}", regs, executions);
            //}
        //}

        print!("ip={} ", leftpad(format!("{}", ip), 2));
        print_regs(&regs);
        //println!("   \t{}", instr.0);
        println!("   \t{} {} {} {}\t", instr.0, instr.1, instr.2, instr.3);
        
        regs = exec(regs, instr);
        
        //print_regs(&regs);
        //println!("");
        
        ip = regs[ip_reg];
        ip += 1;
        executions += 1;
        //if executions >= 100000000 {
            //break;
        //}
    }
}



fn print_instrs(ip_reg: usize, instrs: &Vec<Instruction>) {
    println!("#ip {}", ip_reg);
    for instr in instrs.iter() {
        println!("{} {} {} {} ", instr.0, instr.1, instr.2, instr.3);
    }
    println!("");

}
fn print_regs(regs: &[usize; N]) {
    print!("[{} ", leftpad(format!("{}", regs[0]), 15));
    for i in 1..regs.len() {
        print!(", {}", leftpad(format!("{}", regs[i]), 15));
    }
    print!(" ]");
}


fn optimized() {
    let mut a: usize = 0;
    let mut c: usize = 0;
    let mut d: usize = 0;
    let mut f: usize = 0;

    use std::collections::HashSet;
    let mut unique: HashSet<usize> = HashSet::new();


    'outer: loop {

        f = c | 65536;
        c = 5234604;

        'inner: loop {
            c = c + (f & 255);
            c = c & 16777215;
            c = c * 65899;
            c = c & 16777215;

            if f >= 256 {
                f = f / 256;
            } else {
                break 'inner;
            }
        }

        if !unique.contains(&c) {
            println!("{}", c);
            unique.insert(c);
        }

    }



}
