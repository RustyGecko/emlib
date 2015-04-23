use core::prelude::*;

use collections::vec::Vec;
use collections::string::String;

use {cmu, gpio, usart};
use gpio::Port;

use self::Location::*;

/// Possible locations for the different Usart's. Not all locations are available for every Usart.
#[derive(Copy, Clone, Debug)]
pub enum Location {
    Loc0,
    Loc1,
    Loc2,
    Loc3,
    Loc4,
    Loc5,
}

/// Possible different Usart's to configure.
#[derive(Copy, Clone, Debug)]
pub enum Config {
    Usart0(Location),
    Usart1(Location),
    Usart2(Location),
}

#[allow(dead_code)]
pub struct Usart {
    config: Config,
    baudrate: u32,
    usart: &'static mut usart::Usart,
    line_break: &'static str,
}

/// Initializes and returns a new Usart1 instance with Location 1 and a baudrate of 9600.
impl Default for Usart {
    fn default() -> Usart {
        Usart {
            config: Config::Usart1(Loc1),
            baudrate: 9600,
            usart: usart::Usart::usart1(),
            line_break: "\0\r",
        }
    }
}

impl Usart {
    /// Returns a new Usart instance. It is configured to return Strings when it receives
    /// either a `\0` or a `\r` character.
    pub fn new(config: Config, baudrate: u32) -> Usart {
        Usart {
            config: config,
            baudrate: baudrate,
            usart: match config {
                Config::Usart0(_) => usart::Usart::usart0(),
                Config::Usart1(_) => usart::Usart::usart1(),
                Config::Usart2(_) => usart::Usart::usart2(),
            },
            line_break: "\0\r",
        }
    }

    /// Set the `line_break` characters used by the `read_line` function.
    pub fn set_line_break(&mut self, chars: &'static str) {
        self.line_break = chars;
    }

    /// Called in order to initialize the Usart according to it's config
    pub fn init_async(&mut self) {
        // Enable clock for HF peripherals
        cmu::clock_select_set(cmu::Clock::HF, cmu::Select::HFRCO);

        // Configure usart
        let (usart_clock, usart) = match self.config {
            Config::Usart0(_) => (cmu::Clock::USART0, usart::Usart::usart0()),
            Config::Usart1(_) => (cmu::Clock::USART1, usart::Usart::usart1()),
            Config::Usart2(_) => (cmu::Clock::USART2, usart::Usart::usart2()),
        };
        self.usart = usart;
        cmu::clock_enable(usart_clock, true);
        self.usart.init_async(&usart::InitAsync {
            enable: usart::Enable::Disable,
            baudrate: self.baudrate,
            .. Default::default()
        });

        // Configure TX, RX, CLK, CS pins and Usart location
        self.init_location_and_gpio();

        // Enable Usart
        self.usart.enable(usart::Enable::Enable);
    }

    pub fn newline(&self) {
        self.putc('\0' as u8);
        self.putc('\n' as u8);
        self.putc('\r' as u8);
    }

    /// Performs a blocking send of one `char`.
    pub fn putc(&self, data: u8) {
        self.usart.tx(data);
    }

    /// Blocks until one `char` is received.
    pub fn getc(&self) -> u8 {
        self.usart.rx()
    }

    /// Performs a blocking send of a `&str`
    pub fn write_line(&self, string: &str) {
        for c in string.chars() {
            self.putc(c as u8);
        }
    }

    /// Blocks and returns a `String` that is terminated by either one of the
    /// characters in `line_break`.
    ///
    /// #Panics
    /// Panics if it fails to parse the received characters to a UTF8 String.
    pub fn read_line(&self) -> String {
        let mut bytes = Vec::new();

        loop {
            match self.getc() as char {
                c if self.line_break.contains(c) => { break; }
                c => { bytes.push(c as u8); }
            }
        }

        String::from_utf8(bytes).unwrap_or_else(|e| {
            panic!("{}", e)
        })
    }

    /// Returns `true` if a `char` is available to read.
    pub fn readable(&self) -> bool {
        (self.usart.IF & usart::STATUS_RXDATAV) != 0
    }

    /// Returns `true` if there is space available to write a `char`.
    pub fn writeable(&self) -> bool {
        (self.usart.IF & usart::STATUS_TXBL) != 0
    }

    fn init_location_and_gpio(&mut self) {
        let (location, tx, rx, clk, cs) = match self.config {
            Config::Usart0(loc) => {
                match loc {
                    Loc0 => {
                        (usart::ROUTE_LOCATION_LOC0, Some((Port::E, 10)), Some((Port::E, 11)),
                            Some((Port::E, 12)), Some((Port::E, 13)))
                    },
                    Loc1 => {
                        (usart::ROUTE_LOCATION_LOC1, Some((Port::E, 7)), Some((Port::E, 6)),
                            Some((Port::E, 5)), Some((Port::E, 4)))
                    },
                    Loc2 => {
                        (usart::ROUTE_LOCATION_LOC2, Some((Port::C, 11)), Some((Port::C, 10)),
                            Some((Port::C, 9)), Some((Port::C, 8)))
                    },
                    Loc3 => {
                        (usart::ROUTE_LOCATION_LOC3, Some((Port::E, 13)), Some((Port::E, 12)),
                            None, None)
                    },
                    Loc4 => {
                        (usart::ROUTE_LOCATION_LOC4, Some((Port::B, 7)), Some((Port::B, 8)),
                            Some((Port::B, 13)), Some((Port::B, 14)))
                    },
                    Loc5 => {
                        (usart::ROUTE_LOCATION_LOC5, Some((Port::C, 0)), Some((Port::C, 1)),
                            Some((Port::B, 13)), Some((Port::B, 14)))
                    },
                }
            },
            Config::Usart1(loc) => {
                match loc {
                    Loc0 => {
                        (usart::ROUTE_LOCATION_LOC0, Some((Port::C, 0)), Some((Port::C, 1)),
                            Some((Port::B, 7)), Some((Port::B, 8)))
                    },
                    Loc1 => {
                        (usart::ROUTE_LOCATION_LOC1, Some((Port::D, 0)), Some((Port::D, 1)),
                            Some((Port::D, 2)), Some((Port::D, 3)))
                    },
                    Loc2 => {
                        (usart::ROUTE_LOCATION_LOC2, Some((Port::D, 7)), Some((Port::D, 6)),
                            Some((Port::F, 0)), Some((Port::F, 1)))
                    },
                    _ => panic!("Invalid location for Usart1: {:?}", loc)
                }
            },
            Config::Usart2(loc) => {
                match loc {
                    Loc0 => {
                        (usart::ROUTE_LOCATION_LOC0, Some((Port::C, 2)), Some((Port::C, 3)),
                            Some((Port::C, 4)), Some((Port::C, 5)))
                    },
                    Loc1 => {
                        (usart::ROUTE_LOCATION_LOC1, Some((Port::B, 3)), Some((Port::B, 4)),
                            Some((Port::B, 5)), Some((Port::B, 6)))
                    },
                    _ => panic!("Invalid location for Usart2: {:?}", loc)
                }
            },
        };

        self.init_gpio(tx, rx, clk, cs);
        self.usart.ROUTE = (self.usart.ROUTE & !usart::ROUTE_LOCATION_MASK) | location;
    }

    fn init_gpio(&mut self, tx: Option<(Port, u32)>, rx: Option<(Port, u32)>,
        clk: Option<(Port, u32)>, cs: Option<(Port, u32)>)
    {
        // Enable clock for GPIO module (required for pin configuration)
        cmu::clock_enable(cmu::Clock::GPIO, true);

        // Configure GPIO pins for TX, RX, CLK and CS
        let tx_route = match tx {
            Some((port, pin)) => {
                gpio::pin_mode_set(port, pin, gpio::Mode::PushPull, 1);
                usart::ROUTE_TXPEN
            },
            None => 0
        };

        let rx_route = match rx {
            Some((port, pin)) => {
                gpio::pin_mode_set(port, pin, gpio::Mode::Input, 0);
                usart::ROUTE_RXPEN
            },
            None => 0
        };

        let clk_route = match clk {
            Some((port, pin)) => {
                gpio::pin_mode_set(port, pin, gpio::Mode::PushPull, 1);
                usart::ROUTE_CLKPEN
            },
            None => 0
        };

        let cs_route = match cs {
            Some((port, pin)) => {
                gpio::pin_mode_set(port, pin, gpio::Mode::PushPull, 1);
                usart::ROUTE_CSPEN
            },
            None => 0
        };

        self.usart.ROUTE |= rx_route | tx_route | clk_route | cs_route;
    }
}
