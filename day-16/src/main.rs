#[allow(non_camel_case_types)]
use scan_fmt::*;
use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

type Val = usize;
type Regs = [Val; 4];
type OpCode = Val;
type A = Val;
type B = Val;
type C = Val;

fn addr(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = r[a] + r[b]                      ; r }
fn addi(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = r[a] + b                         ; r }
fn mulr(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = r[a] * r[b]                      ; r }
fn muli(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = r[a] * b                         ; r }
fn banr(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = r[a] & r[b]                      ; r }
fn bani(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = r[a] & b                         ; r }
fn borr(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = r[a] | r[b]                      ; r }
fn bori(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = r[a] | b                         ; r }
fn setr(mut r: Regs, (a,_,c): (A,B,C)) -> Regs { r[c] = r[a]                             ; r }
fn seti(mut r: Regs, (a,_,c): (A,B,C)) -> Regs { r[c] = a                                ; r }
fn gtir(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = if a > r[b] { 1 } else { 0 }     ; r }
fn gtri(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = if r[a] > b { 1 } else { 0 }     ; r }
fn gtrr(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = if r[a] > r[b] { 1 } else { 0 }  ; r }
fn eqir(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = if a == r[b] { 1 } else { 0 }    ; r }
fn eqri(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = if r[a] == b { 1 } else { 0 }    ; r }
fn eqrr(mut r: Regs, (a,b,c): (A,B,C)) -> Regs { r[c] = if r[a] == r[b] { 1 } else { 0 } ; r }

fn parse_op(line: &str) -> (Val,Val,Val,Val) {
    let y = scan_fmt!(line, "{} {} {} {}", OpCode, A, B, C);
    (y.0.unwrap(), y.1.unwrap(), y.2.unwrap(), y.3.unwrap())
}

fn parse_sample(input: &mut Lines<'_>) -> (Regs,(Val,Val,Val,Val),Regs) {
    let x = scan_fmt!(input.next().unwrap(), "Before: [{}, {}, {}, {}]", OpCode, A, B, C);
    let y = scan_fmt!(input.next().unwrap(), "{} {} {} {}", OpCode, A, B, C);
    let z = scan_fmt!(input.next().unwrap(), "After:  [{}, {}, {}, {}]", OpCode, A, B, C);
    ([x.0.unwrap(), x.1.unwrap(), x.2.unwrap(), x.3.unwrap()],
     (y.0.unwrap(), y.1.unwrap(), y.2.unwrap(), y.3.unwrap()),
     [z.0.unwrap(), z.1.unwrap(), z.2.unwrap(), z.3.unwrap()])
}

use std::str::Lines;
fn main() {
    let mut file = File::open("input-1").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let mut input = buf.lines();
    
    let ops: Vec<fn(Regs, (Val,Val,Val)) -> Regs> = vec![
        addr, addi,
        mulr, muli,
        banr, bani,
        borr, bori,
        setr, seti,
        gtir, gtri, gtrr,
        eqir, eqri, eqrr,
    ];

    let mut unknown: HashMap<OpCode, HashSet<OpCode>> = HashMap::new();
    loop {
        let (pre, (sample_opcode,a,b,c), post) = parse_sample(&mut input);
        for (opcode, op) in ops.iter().enumerate() {
            if op(pre, (a,b,c)) == post {
                unknown
                    .entry(sample_opcode)
                    .and_modify(|matches| { matches.insert(opcode); })
                    .or_insert_with(|| HashSet::from_iter(vec![opcode]));
            }
        }
        if input.next().is_none() {
            break;
        }
    }

    let mut translation: HashMap<OpCode, OpCode> = HashMap::new();
    loop {
        let mut singles: Vec<(OpCode, OpCode)> = Vec::new();
        for (sample_opcode, matches) in unknown.iter() {
            if matches.len() == 1 {
                singles.push((*sample_opcode, *matches.iter().next().unwrap()));
            }
        }
        for (sample_opcode, _) in singles.iter() {
            unknown.remove(&sample_opcode);
        }
        for (_, opcode) in singles.iter() {
            for (_, matches) in unknown.iter_mut() {
                matches.remove(&opcode);
            }
        }
        translation.extend(singles);
        if unknown.len() == 0 {
            break;
        }
    }
    println!("{:?}", translation);

    let mut file = File::open("input-2").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let input = buf.lines();

    let mut reg: Regs = [0; 4];
    for line in input {
        let (opcode, a, b, c) = parse_op(line);
        reg = ops[*translation.get(&opcode).unwrap()](reg, (a,b,c));
    }
    println!("{:?}", reg);
}
