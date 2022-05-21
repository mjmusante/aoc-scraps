enum Operand {
    Reg(char),
    Num(i64),
}

enum Opcode {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
    Out(Operand),
}

fn idx(src: &Operand) -> usize {
    if let Operand::Reg(r) = src {
        *r as usize - 'a' as usize
    } else {
        panic!("decoding number as register");
    }
}

fn val(reg: &[i64], src: &Operand) -> i64 {
    match src {
        Operand::Num(n) => *n,
        Operand::Reg(r) => reg[(*r as usize - 'a' as usize)],
    }
}

fn run(prg: &[Opcode], start: i64) -> bool {
    let mut reg = vec![start, 0, 0, 0];
    let mut count = 0;
    let mut last_out = 1;

    let mut ip = 0;
    while ip < prg.len() {
        match &prg[ip] {
            Opcode::Cpy(src, dst) => {
                let x = val(&reg, src);
                reg[idx(dst)] = x;
            }
            Opcode::Inc(tgt) => reg[idx(tgt)] += 1,
            Opcode::Dec(tgt) => reg[idx(tgt)] -= 1,
            Opcode::Jnz(tst, jmp) => {
                if val(&reg, tst) != 0 {
                    let newip = ip as i64 + val(&reg, jmp);
                    ip = newip as usize;
                    continue;
                }
            }
            Opcode::Out(tgt) => {
                let out = val(&reg, tgt);
                if out == last_out {
                    print!("{start}: failed after {count}       \r");
                    return false;
                }
                count += 1;
                last_out = out;
                if count > 10000 {
                    return true;
                }
            }
        }
        ip += 1;
    }

    false
}

fn main() {
    use Opcode::*;
    use Operand::*;
    let program = vec![
        Cpy(Reg('a'), Reg('d')),
        Cpy(Num(15), Reg('c')),
        Cpy(Num(170), Reg('b')),
        Inc(Reg('d')),
        Dec(Reg('b')),
        Jnz(Reg('b'), Num(-2)),
        Dec(Reg('c')),
        Jnz(Reg('c'), Num(-5)),
        Cpy(Reg('d'), Reg('a')),
        Jnz(Num(0), Num(0)),
        Cpy(Reg('a'), Reg('b')),
        Cpy(Num(0), Reg('a')),
        Cpy(Num(2), Reg('c')),
        Jnz(Reg('b'), Num(2)),
        Jnz(Num(1), Num(6)),
        Dec(Reg('b')),
        Dec(Reg('c')),
        Jnz(Reg('c'), Num(-4)),
        Inc(Reg('a')),
        Jnz(Num(1), Num(-7)),
        Cpy(Num(2), Reg('b')),
        Jnz(Reg('c'), Num(2)),
        Jnz(Num(1), Num(4)),
        Dec(Reg('b')),
        Dec(Reg('c')),
        Jnz(Num(1), Num(-4)),
        Jnz(Num(0), Num(0)),
        Out(Reg('b')),
        Jnz(Reg('a'), Num(-19)),
        Jnz(Num(1), Num(-21)),
    ];

    let mut start = 0;
    while !run(&program, start) {
        start += 1;
    }
    println!("\ntry {start}");
}
