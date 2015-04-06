use emlib::gpio;

/*
This approach makes button interrupts a lot faster, probably due to the way interrupts
are handled by the 'gpioint' module. This way we ignore everything with functions specific for
each interrupt, and clear it in the main loop instead.
However, it is not that nice, so using the libraries would be preferable.

Also, for some reason, SW4 (Port C Pin 3) can not be registered as a button when using the
library, but it does work when it's initialized like this.
We should look into the library of why this happens, there might be a bug somewhere, leading to the
pin not getting initialized as a Button.
Or maybe the pin is used by some other peripheral (EBI?), and thus
maybe it shouldn't be used for button interrupts at all?

We'll stick with this initialization for now...
*/
pub fn init() {
    let gpio_pa_base: u32          = 0x40006000;
    let gpio_pc_base: u32          = 0x40006048;
    let gpio_pc_model: *mut u32    = (gpio_pc_base + 0x04) as *mut u32;
    let gpio_pc_dout: *mut u32     = (gpio_pc_base + 0x0c) as *mut u32;
    let gpio_exitpsell: *mut u32   = (gpio_pa_base + 0x100) as *mut u32;
    let gpio_exitfall: *mut u32    = (gpio_pa_base + 0x108) as *mut u32;
    let gpio_exitrise: *mut u32    = (gpio_pa_base + 0x10c) as *mut u32;
    let gpio_ien: *mut u32         = (gpio_pa_base + 0x110) as *mut u32;

    unsafe {
        // Enable pins 0-7 on Port C, according to SW1-SW8
        *gpio_pc_model = 0x33333333;
        *gpio_pc_dout = 0xff;

        // Enable interrupts for each of the pins
        *gpio_exitpsell = 0x22222222;
        *gpio_exitfall = 0xff;
        *gpio_exitrise = 0xff;
        *gpio_ien = 0xff;
    }
}
