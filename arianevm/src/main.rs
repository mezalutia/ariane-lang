use arianevm;
use arianevm::VirtualMachine;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let bytecode = std::fs::read(&args[1]).unwrap();

    let mut vm = VirtualMachine::new(bytecode.into());
    vm.run();
}