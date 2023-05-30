#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[rtic::app(device = e310x)]
mod app {
    use common::sprintln;
    use riscv_rt as _;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        common::init_uart();
        sprintln!("Hello, RTIC!");

        sprintln!("init");
        (Shared {}, Local {})
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        //interrupts are enabled again; the `SoftLow` handler runs at this point
       sprintln!("idle");
       loop {
           continue;
        }
    }
    
}
