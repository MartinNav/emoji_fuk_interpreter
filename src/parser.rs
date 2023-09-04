
#[derive(Debug,PartialEq)]
pub enum Instruction{
    MoveR,
    MoveL,
    Add,
    Sub,
    Out,
    In,
    Nop,
    JR,
    JL,
}

#[inline(always)]
pub fn parse(source_code:String)->Vec<Instruction>{
let symbols:Vec<char> = source_code.chars().collect();

symbols.iter().map(|sym| tokenize(sym)).collect()

}
fn tokenize(symbols:&char)->Instruction{
    match symbols {
        '>' | '🤔'|'👉'=>Instruction::MoveR,
        '<'|'🤒'|'👈'=>Instruction::MoveL,
        '+'|'🥵'| '🤯'=>Instruction::Add,
        '-'|'🥶'|'🥴'=>Instruction::Sub,
        '.'|'👇'|'👎'=>Instruction::Out,
        ','|'☝'|'👍'=>Instruction::In,
        '['|'⏭' =>Instruction::JR,
        ']'|'⏮'=>Instruction::JL,
        _=>Instruction::Nop,
    }
}


#[cfg(test)]
mod parser_tests{
    use crate::parser::{parse,Instruction};
    #[test]
    fn parser_movR_test() {

        let parsed = parse(">🤔👉".to_string());
    
        let comparable = vec![Instruction::MoveR,Instruction::MoveR,Instruction::MoveR];
        assert_eq!(parsed[0], comparable[0]);
        assert_eq!(parsed[1], comparable[1]);
        assert_eq!(parsed[2], comparable[2]);

    }
    #[test]
    fn parser_movL_test(){
        let parsed=parse("<🤒👈".to_string());
        assert_eq!(parsed[0],Instruction::MoveL);
        assert_eq!(parsed[1],Instruction::MoveL);
        assert_eq!(parsed[2],Instruction::MoveL);
    }   
    #[test]
    fn parser_add_test(){
        let parsed = parse("+🥵🤯".to_string());
        assert_eq!(parsed[0], Instruction::Add);
        assert_eq!(parsed[1], Instruction::Add);
        assert_eq!(parsed[2], Instruction::Add);
    }
    #[test]
    fn parser_sub_test(){
        let parsed = parse("-🥶🥴".to_string());
        assert_eq!(parsed[0], Instruction::Sub);
        assert_eq!(parsed[1], Instruction::Sub);
        assert_eq!(parsed[2], Instruction::Sub);
    }
    #[test]
    fn parser_out_test(){
        let parsed = parse(".👇👎".to_string());
        assert_eq!(parsed[0], Instruction::Out);
        assert_eq!(parsed[1], Instruction::Out);
        assert_eq!(parsed[2], Instruction::Out);
    }
    #[test]
    fn parser_in_test(){
        let parsed = parse(",☝👍".to_string());
        assert_eq!(parsed[0], Instruction::In);
        assert_eq!(parsed[1], Instruction::In);
        assert_eq!(parsed[2], Instruction::In);
    }
    #[test]
    fn parser_invalid_test(){
        let parsed = parse("?/*&@123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());
        for p in parsed{
            assert_eq!(p, Instruction::Nop);
        }
    }
}
