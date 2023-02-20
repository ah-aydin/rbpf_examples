use rbpf::helpers;
use rbpf::assembler::assemble;

#[test]
fn test_vm_add() {
    let prog = assemble("
        mov32 r0, 1
        mov32 r1, 2
        add r0, r1
        exit 
    ").unwrap();

    let vm = rbpf::EbpfVmNoData::new(Some(&prog)).unwrap();
    assert_eq!(vm.execute_program().unwrap(), 0x3);
}

#[test]
fn test_vm_add_context() {
    let prog = assemble("
        ldxw r0, [r1]
        ldxw r1, [r1+4]
        add r0, r1
        be32 r0
        exit
    ").unwrap();
    let mem = &mut [
        0x0, 0x22, 0x0, 0x0,
        0x0, 0x0, 0x33, 0x0
    ];

    let vm = rbpf::EbpfVmRaw::new(Some(&prog)).unwrap();
    assert_eq!(vm.execute_program(mem).unwrap(), 0x00223300);
}

#[allow(unused_variables)]
pub fn fib(r1: u64, r2: u64, r3: u64, r4: u64, r5: u64) -> u64 {
    let mut i = 1;
    let mut j = 1;
    let mut k = r1;
    
    while k - 2 > 0 {
        j = i + j;
        i = j - i;
        k -= 1;
    }

    j
}

#[test]
fn test_vm_fib() {
    let prog = assemble("
        mov r2, r1
        ldxw r1, [r1]
        call 0
        stxw [r2+4], r0
        exit
    ").unwrap();

    let mem = &mut [
        0xa, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0
    ];
    
    let mut vm = rbpf::EbpfVmRaw::new(Some(&prog)).unwrap();
    vm.register_helper(0, fib).unwrap();

    assert_eq!(vm.execute_program(mem).unwrap(), 0x37);
    assert_eq!(mem[4], 0x37);
}

fn main() {
}
