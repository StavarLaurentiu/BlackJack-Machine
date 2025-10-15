use embassy_rp::bind_interrupts;
use embassy_rp::i2c::InterruptHandler;
use embassy_rp::peripherals::{I2C0, I2C1};

// Bind the I2C interrupt handlers
bind_interrupts!(
    pub(super) struct Irqs {
        I2C0_IRQ => InterruptHandler<I2C0>;
        I2C1_IRQ => InterruptHandler<I2C1>;
    }
);
