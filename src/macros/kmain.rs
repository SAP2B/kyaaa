#[macro_export]
macro_rules! kmain {
    (
        align($align:literal);
        fn kmain($arg:ident: $arg_ty:ty) -> $ret_ty:ty $body:block
    ) => {
        #[cfg(not(test))]
        #[panic_handler]
        fn panic(_info: &::core::panic::PanicInfo) -> ! {
            #[cfg(target_os = "none")]
            {
                $crate::std::syscall::exit(1)
            }
            #[cfg(not(target_os = "none"))]
            {
                loop {}
            }
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

        #[unsafe(no_mangle)]
        pub extern "C" fn kmain($arg: $arg_ty) -> $ret_ty {
            $body
        }
    };
}
