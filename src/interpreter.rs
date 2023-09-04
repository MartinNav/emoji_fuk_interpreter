use std::io::Read;

use crate::parser::Instruction;



pub fn run(program:&Vec<Instruction>){

    let mut memory =[0u8;30_000];
    let mut location = 15_000;
    let mut inst_ptr =0;
    loop {
        let instruction = &program[inst_ptr];
        match instruction{
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
            Instruction::JR=>{
                
            },
                Instruction::JL=>{
                let mut ip = inst_ptr;
                if memory[location]!=0{
                loop {
                    ip-=1;
                    if program[ip]==Instruction::JR&&memory[location]>0 {
                        inst_ptr=ip;
                        break;
                    }
                }
                }
            },
            Instruction::In=>{
                let mut input_str:String=String::new();
                std::io::stdin().read_line(&mut input_str).unwrap();
                memory[location]= input_str.chars().take(1).last().unwrap_or(' ') as u8;
            },
            Instruction::Out=>{
                println!("{}", std::str::from_utf8(&[memory[location]]).unwrap_or(""));
            },
            Instruction::Nop=>{}
        }
        inst_ptr+=1;
        if inst_ptr==program.len() {
            return;
        }
    }

}


