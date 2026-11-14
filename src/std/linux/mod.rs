pub fn check_err(ret: usize) -> Result<usize, i32> {
    let ret_isize = ret as isize;
    if ret_isize > -4096 && ret_isize < 0 {
        Err(-(ret_isize as i32))
    } else {
        Ok(ret)
    }
}

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod prelude;

/*#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;*/
