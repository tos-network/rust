//@ revisions: tbf

//@[tbf] compile-flags: --target=tbf-tos-tos
//@[tbf] needs-llvm-components: tbf

#![crate_type = "lib"]
#![feature(tbf_target_feature)]
#![no_std]

#[no_mangle]
#[target_feature(enable = "alu32")]
// CHECK: define i8 @foo(i8 returned %arg) unnamed_addr #0 {
pub unsafe fn foo(arg: u8) -> u8 {
    arg
}
