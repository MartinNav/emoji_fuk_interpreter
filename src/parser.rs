

pub enum Instruction{
    MoveR,
    MoveL,
    Add,
    Sub,
    Out,
    In,
    Nop,
}

#[inline(always)]
pub fn parse(source_code:String)->Vec<Instruction>{
let symbols:Vec<char> = source_code.chars().collect();

symbols.iter().map(|sym| tokenize(sym)).collect()

}
#[inline(always)]
fn tokenize(symbols:&char)->Instruction{
    match symbols {
        '>' | '🤔'|'👉'=>Instruction::MoveR,
        '<'|'🤒'|'👈'=>Instruction::MoveL,
        '+'|'🥵'| '🤯'=>Instruction::Add,
        '-'|'🥶'|'🥴'=>Instruction::Sub,
        '.'|'👇'|'👎'=>Instruction::Out,
        ','|'☝'|'👍'=>Instruction::In,
        _=>Instruction::Nop,
    }
}
