use std::io::Read;

use crate::parser::Instruction;


//mod parser;




pub fn run(program:Vec<Instruction>){

    let mut memory =[0u8;30_000];
    let mut location = 15_000;

    for instruction in program {
        match instruction {
            Instruction::Add=>{
                memory[location]+=1;
            },
            Instruction::Sub=>{
                memory[location]-=1;
            },
            Instruction::MoveL=>{
                location-=1;
            },
            Instruction::MoveR=>{
                location+=1;
            },
            Instruction::In=>{
                memory[location]=std::io::stdin().bytes().next().unwrap_or(Ok(0)).unwrap_or(0);
            },
            Instruction::Out=>{
                print!("{}", std::str::from_utf8(&[memory[location]]).unwrap_or(""));
            },
            Instruction::Nop=>{}
        }
    }



}



/*fn execute(instruction: parser::Instruction,cell_v:u8,cell_i:u16){

}
*/

