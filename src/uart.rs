use core::fmt::{Error, Write};

pub enum UartBase {
    UART0 = 0x02500000,
    UART1 = 0x02500400,
    UART2 = 0x02500800,
    UART3 = 0x02500C00,
    UART4 = 0x02501000,
    UART5 = 0x02501400,
}

pub struct Uart(usize);

impl Write for Uart {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        for c in out.bytes() {
            self.put(c);
        }
        Ok(())
    }
}

impl Default for Uart {
    fn default() -> Self {
        Self::new(UartBase::UART0)
    }
}

impl Uart {
    pub fn new(uart: UartBase) -> Self {
        Self(uart as usize)
    }

    pub fn get(&self) -> Option<u8> {
        unsafe {
            let ptr = self.0 as *const u8;
            if 1 == ptr.add(0x0014).read_volatile() & (0x1 << 0) {
                Some(ptr.read_volatile())
            } else {
                None
            }
        }
    }

    pub fn put(&mut self, c: u8) {
        unsafe {
            let ptr = self.0 as *mut u8;
            while 0 == ptr.add(0x0014).read_volatile() & (0x1 << 5) {}
            ptr.write_volatile(c);
            // core::arch::asm!("sb {}, ({})\n", in(reg) c,in(reg) UART0);
        }
    }
}
