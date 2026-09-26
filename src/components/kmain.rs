// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[macro_export]
macro_rules! kmain {
    (
        align($align:literal);
        fn kmain($arg:ident: $arg_ty:ty) -> $ret_ty:ty $body:block
    ) => {

        #[panic_handler]
        fn panic(_info: &::core::panic::PanicInfo) -> ! {
            $crate::components::syscall::Syscall::exit(1).expect("Kyaaa panic error");
            loop {}
        }

        #[cfg(target_arch = "x86_64")]
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        pub extern "C" fn _start() -> ! {
            ::core::arch::naked_asm!(
                "xor rbp, rbp",
                "mov rdi, rsp",
                concat!("and rsp, -", stringify!($align)),
                "call {main}",
                "mov rdi, rax",
                "mov rax, 231",
                "syscall",
                "ud2",
                main = sym kmain,
            );
        }

        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        pub extern "C" fn _start() -> ! {
            ::core::arch::naked_asm!(
                "mv a0, sp",
                concat!("andi sp, sp, -", stringify!($align)),
                "call {main}",
                "li a7, 93",
                "ecall",
                "unimp",
                main = sym kmain,
            );
        }

        #[cfg(target_arch = "aarch64")]
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        pub extern "C" fn _start() -> ! {
            ::core::arch::naked_asm!(
                "mov x0, sp",
                "mov x1, sp",
                concat!("and x1, x1, -", stringify!($align)),
                "mov sp, x1",
                "bl {main}",
                "mov x8, 93",
                "svc #0",
                "brk #0",
                main = sym kmain,
            );
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn kmain($arg: $arg_ty) -> $ret_ty {
            $body
        }
    };
}
