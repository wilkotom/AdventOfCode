use std::{sync::mpsc::{Receiver, Sender, self}, thread};
use hashbrown::HashMap;


#[derive(Debug)]
pub struct IntCodeMachine {
    program: HashMap<i64,i64>,
    instruction_pointer: i64,
    running: bool,
    pub input: Receiver<i64>,
    out_tx: Sender<i64>,
    relative_base: i64
}

impl IntCodeMachine {
    fn execute(&mut self) {
        while self.running {

            let raw_instruction = self.program[&self.instruction_pointer];
            let instruction = raw_instruction % 100;
            let mode_1 = (raw_instruction/ 100) % 10;
            let mode_2 = (raw_instruction / 1000) % 10;
            let mode_3 = (raw_instruction / 10000) % 10;
            let first = self.resolve_param(1, mode_1);
            let second = self.resolve_param(2, mode_2);
            

            match instruction {
                1 => {
                    let destination = self.program[&(self.instruction_pointer+3)] + if mode_3 == 2 {self.relative_base} else {0};
                    self.program.insert(destination, first + second);
                    self.instruction_pointer += 4;
                },
                2 => {
                    let destination = self.program[&(self.instruction_pointer+3)] + if mode_3 == 2 {self.relative_base} else {0};
                    self.program.insert(destination,first * second);
                    self.instruction_pointer += 4;

                },
                3 => {
                    let destination = self.program[&(self.instruction_pointer+1)] + if mode_1 == 2 {self.relative_base} else {0};

                    if let Ok(n) = self.input.recv() {
                        self.program.insert(destination, n);
                        self.instruction_pointer += 2;
                    } else {
                        self.running = false;
                    }
                },
                4 => {
                    self.out_tx.send(first).unwrap();
                    self.instruction_pointer += 2;
                },
                5 => {
                    if first != 0 {
                        self.instruction_pointer = second;
                    } else {
                        self.instruction_pointer += 3;
                    }
                },
                6 => {
                    if first == 0 {
                        self.instruction_pointer = second;
                    } else {
                        self.instruction_pointer += 3;
                    }
                },
                7 => {
                    let destination = self.program[&(self.instruction_pointer+3)] + if mode_3 == 2 {self.relative_base} else {0};
                    self.program.insert(destination, if first < second { 1} else {0});
                    self.instruction_pointer += 4;
                    
                }
                8 => {
                    let destination = self.program[&(self.instruction_pointer+3)] + if mode_3 == 2 {self.relative_base} else {0};
                    self.program.insert(destination, if first == second { 1} else {0});
                    self.instruction_pointer += 4;
                },
                9 => {
                    self.relative_base += first;
                    self.instruction_pointer +=2;
                }
                99 => {self.running = false}
                _ => panic!("Didn't understand Instruction {}", instruction)

            }
        }

    }

    fn resolve_param(&self, offset: i64, mode: i64) -> i64 {
        match mode {
            0 => *self.program.get(self.program.get(&(self.instruction_pointer + offset)).unwrap_or(&0)).unwrap_or(&0),
            1 => *self.program.get(&(self.instruction_pointer + offset)).unwrap_or(&0),
            2 => *self.program.get(&(self.program.get(&(self.instruction_pointer + offset)).unwrap_or(&0) + self.relative_base)).unwrap_or(&0),
            _ => unimplemented!()
        }
    }
    
    fn start(mut self) {
        thread::spawn(move || self.execute());
    }


}

pub fn create_vm(program_vec: &[i64]) -> IntCodeMachine {
    let (_, in_rx): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (out_tx, _): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let mut program = HashMap::new();
    for (n,i) in program_vec.iter().enumerate() {
        program.insert(n as i64, *i);
    }
    IntCodeMachine{program, 
        instruction_pointer: 0, 
        running: true, 
        input: in_rx, 
        out_tx,
        relative_base: 0}
}

pub fn run_program (program: &[i64], result_register: i64) -> i64 {
    let mut machine = create_vm(program);
    while machine.running {
        machine.execute();
    }
    machine.program[&result_register]

}

pub fn start_machine(program_vec: &[i64]) -> (Sender<i64>, Receiver<i64>) {
    let (in_tx, in_rx): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (out_tx, out_rx): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let mut program = HashMap::new();
    for (n,i) in program_vec.iter().enumerate() {
        program.insert(n as i64, *i);
    }
    let machine = IntCodeMachine{program, 
        instruction_pointer: 0, 
        running: true, 
        input: in_rx, 
        out_tx,
        relative_base: 0};
    machine.start();
    (in_tx, out_rx)
}

#[test]
fn day2_example_1() {
    assert_eq!(run_program(&[1,0,0,0,99], 0), 2);
}
#[test]
fn day2_example_2() {
    assert_eq!(run_program(&[2,3,0,3,99], 3), 6);
}

#[test]
fn day2_example_3() {
    assert_eq!(run_program(&[2,4,4,5,99,0], 5), 9801);
}

#[test]
fn day2_example_4() {
    assert_eq!(run_program(&[1,1,1,4,99,5,6,0,99], 0), 30);
}


#[test]
fn day5_parameter_example_1() {
    assert_eq!(run_program(&[1002,4,3,4,33], 4), 99);
}


#[test]
fn day5_parameter_example_2() {
    assert_eq!(run_program(&[1101,100,-1,4,0], 4), 99);
}

#[test]
fn day5_channels() {
    let program = &[3,0,4,0,99];

    let (in_tx, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    in_tx.send(3).unwrap();
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 3);
}

#[test]
fn day5_test_8_equal_to_8_position() {
    let program = &[3,9,8,9,10,9,4,9,99,-1,8];
    let (in_tx, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    in_tx.send(8).unwrap();
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 1);
}

#[test]
fn day5_test_3_equal_to_8_position() {
    let program = &[3,9,8,9,10,9,4,9,99,-1,8];
    let (in_tx, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    in_tx.send(3).unwrap();
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 0);
}


#[test]
fn day5_less_than_8_position() {
    let program = &[3,9,8,9,10,9,4,9,99,-1,8];
    let (in_tx, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    in_tx.send(0).unwrap();
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 0);
}

#[test]
fn day5_8_equal_to_8_immediate() {
    let program = &[3,3,1108,-1,8,3,4,3,99];
    let (in_tx, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    in_tx.send(8).unwrap();
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 1);
}

#[test]
fn day5_3_equal_to_8_immediate() {
    let program = &[3,3,1108,-1,8,3,4,3,99];
    let (in_tx, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    in_tx.send(3).unwrap();
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 0);
}

#[test]
fn day5_less_than_8_immediate() {
    let program = &[3,3,1107,-1,8,3,4,3,99];
    let (in_tx, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    in_tx.send(0).unwrap();
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 1);
}


#[test]
fn day5_less_than_8_expect_999() {
    let program = &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    let (in_tx, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    in_tx.send(7).unwrap();
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 999);
}

#[test]
fn day5_equal_8_expect_1000() {
    let program = &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    let (in_tx, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    in_tx.send(8).unwrap();
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 1000);
}

#[test]
fn day5_greater_than_8_expect_1001() {
    let program = &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    let (in_tx, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    in_tx.send(9).unwrap();
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 1001);
}

#[test]
fn day9_example_1(){
    let program = &[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    let (_, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    let mut result = Vec::new();
    let mut output = in_rx.recv();
    while let Ok(n) = output {
        result.push(n);
        output = in_rx.recv();
    }
    assert_eq!(result, program);
}

#[test]
fn day9_example_2(){
    let program = &[1102,34915192,34915192,7,4,7,99,0];
    let (_, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 1_219_070_632_396_864);
}

#[test]
fn day9_example_3(){
    let program = &[104,1125899906842624,99];
    let (_, in_rx): (Sender<i64>, Receiver<i64>) = start_machine(program);
    let output = in_rx.recv().unwrap();
    assert_eq!(output, 1125899906842624);
}

