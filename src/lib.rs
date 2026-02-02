//! A platform agnostic Rust driver for the Akafugu TWIDisplay 4-digit 7-segment display controller,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//!
//! This driver allows you to:
//! - Display single digits or characters, also at a selected position
//! - Display text, although some characters may not be available (see display documentation)
//! - Clear the display
//! - Show the current I2C address
//! - Change the I2C address (experimental function)
//! - Display time in HH.MM format
//! - Display temperature or humidity, with settable lower/upper threshold
//!
//!## The device
//! The TWI 7-segment Display is an easy to use 4-digit 7-segment display that is controlled using the TWI (I2C compatible) protocol.
//! It is based on an ATMega4313 MCU, acting as a peripheral I2C device.
//!
//! ### Information: [TWIDisplay](https://www.akafugu.jp/posts/products/twidisplay/)
//!
//! ## Usage examples (see also examples folder)
//!
//! Please find additional examples using hardware in this repository: [examples]
//!
//! [examples]: https://github.com/nebelgrau77/akafugu_twidisplay-rs-async/tree/main/examples
//!
//! ### Initialization
//! A new instance of the device is created as follows:
//!
//! ```rust
//! use akafugu_twidisplay::*:
//!
//! let mut akafugu = TWIDisplay::new(i2c, DEFAULT_ADDRESS);
//! ```
//!
//! The default address is 0x12. If the address was changed with the `set_address()` function,
//! the new address must be used after a power down-power up sequence.  
//!
//!
//! ### Main functions
//!
//! Display can be cleared with the following command:
//! ```rust
//! akafugu.clear_display().await.unwrap();
//! ```
//!
//! Digits and/or characters can either be simply sent to display, or displayed at defined positions.
//!
//! ```rust
//! // display digit '7' at position 2 (positions are 0,1,2,3 from left to right)
//! akafugu.display_digit(2, 7).await.unwrap();
//! // display character 'P' at position 3
//! akafugu.display_char(3,'P').await.unwrap();
//! ```
//!
//! If a digit/character is just sent to the display, it will appear according to the selected mode
//! (scroll or rotate) - please see the documentation.
//!
//! ```rust
//! akafugu.send_char('A').await.unwrap();
//! akafugu.send_char('B').await.unwrap();
//! akafugu.send_char('C').await.unwrap();
//! akafugu.send_char('D').await.unwrap();
//! ```
//!
//! This will display `ABCD`.
//!
//! ```rust
//! akafugu.send_char('E').await.unwrap();
//! ```
//!
//! Depending on the selected mode the display will show now:
//! * in SCROLL mode: 'BCDE'
//! * in ROTATE mode: 'EBCD'
//!
//! Text can be sent to display as string literals:
//!
//! ```rust
//! akafugu.send_text("HELLO LOOP PULL CALL").await.unwrap();
//! ```
//!
//! Numbers from 0-9999 range can be displayed with the following function:
//! ```rust
//! akafugu.display_number(1234).await.unwrap();
//! ```
//! _NOTE_: Numbers will be displayed with leading zeroes, e.g. `0023`.
//!
//! Dots can be turned on or off using this function:
//! ```rust
//! // this will turn on the first and the third dot from the left
//! akafugu.display_dots([true, false, true, false]).await.unwrap();
//! ```
//!
//!
//! ### Control functions
//!
//! Display mode can be changed as follows:
//!
//! ```rust
//! akafugu.set_mode(Mode::Scroll).await.unwrap(); // default mode is `Rotate`
//! ```
//!
//! Brightness can be set between 0 and 255, where 127 is approx. 50% brightness.
//! ```rust
//! akafugu.set_brightness(200).await.unwrap();
//! ```
//!
//! The I2C address of the device can be changed from the default 0x12 as follows:
//! ```rust
//! akafugu.set_address(0x20).await.unwrap();
//! ```
//!
//! The new address will be active after a power down, power up sequence.
//!
//! __NOTE:__ According to the documentation, the allowed range of addresses is 0x00-0x7F,
//! but addresses including and over 0x40 don't seem to work correctly, even though
//! they are correctly displayed. In such case 0x00 must be used to access the device and change the address again.
//! For this reason in this driver the address setting is restricted to 0x00-0x39 range.
//!
//! To show the current I2C address use the following command:
//! ```rust
//! akafugu.display_address().await.unwrap();
//! ```
//! The same can be achieved by simply connecting only the VCC and GND pins of the display.
//!
//! ### Convenience functions
//! The driver has three additional functions, that can be useful for clock or sensor applications.
//!
//! #### Display time
//!
//! Time is displayed in HH.MM format, with the central dot displayed or not:
//!
//! ```rust
//!
//! // get time from the clock
//! let (hours, minutes, seconds) = some_rtc_function();
//!
//! // blink the dot: on if number of seconds is even, otherwise off
//! if seconds % 2 == 0 {
//!     akafugu.display_time(hours, minutes, true).await.unwrap()
//! } else {
//!     akafugu.display_time(hours, minutes, false).await.unwrap()
//! }
//! ```
//!
//! #### Display date
//!  
//!
//! Date can be displayed either in MMDD or DDMM format, with the central dot on or off.
//!
//! ```rust
//!
//! // get date from the clock
//! let (month, day) = some_rtc_function();
//!
//! // display date in MMDD format with the central dot on
//! akafugu.display_date(month, day, DateFormat::MMDD, true).await.unwrap()
//!
//! ```
//!
//!
//! #### Display temperature
//!
//! Displays integer temperature values with a unit of choice (Celsius/Fahrenheit), no leading zeros.
//! The function allows setting lower and upper threshold: if the supplied value is below the lower threshold,
//! the display will show `-LL-`, if above the upper threshold, it will show `-HH-`.
//! This can be useful for sensor applications such as weather stations: thresholds can be set to the limits of
//! reliable readings, e.g. -30 and +60 Celsius degrees, etc.
//! Thresholds are optional and if not given, will default to the minimum and maximum limits, which are set to -99 and 999, respectively.
//! If the supplied value exceeds the limit, the display will show `----`.  
//!
//! ```rust
//! let temp_reading = some_sensor_reading();
//! // display temperature with unit 'C', lower threshold at -50 degrees,
//! // no upper threshold (defaults to +999)
//! // temp_reading < -50 will show as `-LL-`, temp_reading < -99 will show as `----`
//! akafugu.display_temperature(temperature, TempUnits::Celsius, Some(-50), None).await.unwrap();
//! ```
//!
//! #### Display humidity
//!
//! Displays integer humidity values with a default unit 'H', no leading zeros.
//! The function allows setting lower and upper threshold: if the supplied value is below the lower threshold,
//! the display will show `-LL-`, if above the upper threshold, it will show `-HH-`.
//! This can be useful for sensor applications such as weather stations: thresholds can be set to the limits of
//! reliable readings, e.g. between 10 and 90% of relative humidity.
//! Thresholds are optional and if not given, will default to the minimum and maximum limits, which are set to 0 and 100, respectively.
//! If the supplied value exceeds the limit, the display will show `----`.  
//!
//! ```rust
//! let hum_reading = some_sensor_reading();
//! // display humidity, lower threshold at 10%, upper threshold at 90%.
//! // temp_reading < 10 will show as `-LL-`, temp_reading > 90 will show as `-HH-`,
//! // readings below 0 or above 100 will show as `----`
//! akafugu.display_humidity(humidity, Some(10), Some(90)).await.unwrap();
//! ```


#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

use embedded_hal_async as hal;

use hal::i2c::I2c;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2C(E),
    /// Invalid input data
    InvalidInputData,
}

struct Register;

// THESE WILL BE USED FOR VARIOUS OPERATIONS, E.G. SETTING POSITION
impl Register {
    const BRIGHTNESS_SETTING: u8 = 0x80;
    const I2C_ADDRESS_SETTING: u8 = 0x81;
    const CLEAR_DISPLAY: u8 = 0x82;
    const MODE_SETTING: u8 = 0x83;
    const _CUSTOM_CHAR: u8 = 0x84; // not implemented yet
    const DOTS: u8 = 0x85;
    //const _DISPLAY_TIME          :u8 = 0x87; // not sure if this works
    //const _DISPLAY_WORD          :u8 = 0x88;
    const POSITION_SETTING: u8 = 0x89;
    const _FIRMWARE_REV: u8 = 0x8a;
    const _NUMBER_DIGITS: u8 = 0x8b;
    const DISPLAY_ADDRESS: u8 = 0x90;
}

/// Default I2C address for the device
pub const DEFAULT_ADDRESS: u8 = 0x12;

/// Possible choices for temperature units
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum TempUnits {
    /// Celsius degrees
    Celsius,
    /// Fahrenheit degrees
    Fahrenheit,
}

/// Possible choices for date format
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum DateFormat {
    /// Month Day
    MMDD,
    /// Day Month (American style)
    DDMM,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Two possible display modes
pub enum Mode {
    /// Scroll
    Scroll,
    /// Rotate
    Rotate,
}

/// TWIDisplay driver, that holds the I2C bus instance and the I2C address used
#[derive(Debug, Default)]
pub struct TWIDisplay<I2C> {
    /// The concrete I2C device implementation.
    i2c: I2C,
    dev_addr: u8,
}

impl<I2C, E> TWIDisplay<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Create a new instance of the TWIDisplay driver.    
    pub fn new(i2c: I2C, dev_addr: u8) -> Self {
        TWIDisplay { i2c, dev_addr }
    }

    /// Destroy driver instance, return I2C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write data to the I2C bus
    async fn write(&mut self, payload: &[u8]) -> Result<(), Error<E>> {
        self.i2c.write(self.dev_addr, payload).await.map_err(Error::I2C)
    }

    /*

    DOESN'T SEEM TO WORK - NEED TO TEST MORE

    /// Read data from the I2C bus
    fn read(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
        .write_read(self.dev_addr, &[register], &mut data)
        .map_err(Error::I2C)
        .and(Ok(data[0]))
    }

    /// Read the firmware revision number (currently 1)
    pub fn get_firmware_rev(&mut self) -> Result<u8, Error<E>> {
        let data = self.read(Register::FIRMWARE_REV)?;
        Ok(data)
    }

    /// Read the number of digits
    pub fn get_number_digits(&mut self) -> Result<u8, Error<E>> {
        let data = self.read(Register::NUMBER_DIGITS)?;
        Ok(data)
    }

     */

    /// Clear the display
    pub async fn clear_display(&mut self) -> Result<(), Error<E>> {
        self.write(&[Register::CLEAR_DISPLAY]).await?;
        Ok(())
    }

    // NEED TO TEST MORE: TRIED WITH VALUE 0x69, CORRECTLY DISPLAYED A105 ON POWER-UP
    // BUT DID NOT RESPOND ON 0x69 I2C ADDRESS, RESPONDED ONLY WITH ADDRESS 0
    // SEEMS TO WORK OK UP TO 0x39
    // -- USE ADDRESS 0x00 TO RESET IN CASE OF PROBLEMS

    /// Set I2C address, defaults to 0x12
    pub async fn set_address(&mut self, address: u8) -> Result<(), Error<E>> {
        //let mut dev_address = DEFAULT_ADDRESS;
        match address {
            //a if a < 0x7f => self.write(&[Register::I2C_ADDRESS_SETTING, a])?,
            a if a < 0x40 => self.write(&[Register::I2C_ADDRESS_SETTING, a]).await?,
            _ => (),
        }
        Ok(())
    }

    /// Show the current I2C address on the display
    pub async fn display_address(&mut self) -> Result<(), Error<E>> {
        self.write(&[Register::DISPLAY_ADDRESS]).await?;
        Ok(())
    }

    /// Set display brightness (0 - 255, 127 is 50%)
    pub async fn set_brightness(&mut self, brightness: u8) -> Result<(), Error<E>> {
        self.write(&[Register::BRIGHTNESS_SETTING, brightness]).await?;
        Ok(())
    }

    /// Display the dots, with boolean switches (true is on, false is off)

    // dots are numbered 1,2,3,4 from the left, and they correspond to bits
    // so 0b0000_0010 is bit 1, dot 1, 0b0000_1000 is bit 3, dot 3 and so on

    pub async fn display_dots(&mut self, dots: [bool; 4]) -> Result<(), Error<E>> {
        let mut dotvalues: u8 = 0;

        for (idx, dot) in dots.iter().enumerate() {
            match dot {
                true => dotvalues += 2_u8.pow(idx as u32 + 1_u32),
                false => (),
            }
        }

        self.write(&[Register::DOTS, dotvalues]).await?;
        Ok(())
    }

    /// Send a digit to the display without specifying the position
    pub async fn send_digit(&mut self, number: u8) -> Result<(), Error<E>> {
        if number > 9 {
            return Err(Error::InvalidInputData);
        } else {
            self.write(&[number]).await?
        };
        Ok(())
    }

    /// Write digit D at position P
    pub async fn display_digit(&mut self, position: u8, digit: u8) -> Result<(), Error<E>> {
        // TO DO: include hex digits:
        // 0x00 - 0x0f: Displays a single digit 0-9 or hexadecimal digit A-F.

        if position > 3 || digit > 9 {
            return Err(Error::InvalidInputData);
        } else {
            self.write(&[Register::POSITION_SETTING, position, digit]).await?
        };

        Ok(())
    }

    /// Display a number using all four digits
    // TO DO: ADD A BOOLEAN SWITCH "with_leading_zeros"
    pub async fn display_number(&mut self, number: u16) -> Result<(), Error<E>> {
        if number > 9999 {
            return Err(Error::InvalidInputData);
        }

        let digits = TWIDisplay::<I2C>::get_digits(number);

        for (idx, digit) in digits.iter().enumerate() {
            self.display_digit(idx as u8, *digit).await?
        }

        Ok(())
    }

    /// Send a character to the display without specifying the position
    pub async fn send_char(&mut self, ch: char) -> Result<(), Error<E>> {
        // TO DO: restrict to 0x0g - 0x79

        self.write(&[ch as u8]).await?;
        Ok(())
    }

    /// Write character C at position P
    pub async fn display_char(&mut self, position: u8, ch: char) -> Result<(), Error<E>> {
        // TO DO: restrict to 0x0g - 0x79

        if position > 3 {
            return Err(Error::InvalidInputData);
        } else {
            self.write(&[Register::POSITION_SETTING, position, ch as u8]).await?;
        };
        Ok(())
    }

    /// Send text to the display
    pub async fn send_text(&mut self, text: &str) -> Result<(), Error<E>> {
        for ch in text.chars() {
            self.send_char(ch).await?
        }
        Ok(())
    }

    /// Display time in HH:MM format, with an optional dot between them
    pub async fn display_time(&mut self, hours: u8, minutes: u8, dot: bool) -> Result<(), Error<E>> {
        if hours > 23 || minutes > 59 {
            return Err(Error::InvalidInputData);
        } else {
            let time_value = (hours as u16) * 100 + minutes as u16;

            self.display_number(time_value).await?
        };

        match dot {
            true => self.display_dots([false, true, false, false]).await?, // dot at second position
            false => self.display_dots([false, false, false, false]).await?,
        }

        Ok(())
    }

    // TO DO: add display_date(month, day, format) function
    // format can be MMDD or DDMM
    // no leading zeros?
    // middle dot ON
    // check if month <1,12> and day <1,31>
    /// Display date in a selected format, with or without the central dot
    pub async fn display_date(
        &mut self,
        month: u8,
        day: u8,
        format: DateFormat,
        dot: bool,
    ) -> Result<(), Error<E>> {
        if month > 12 || month < 1 {
            return Err(Error::InvalidInputData);
        } else if day < 1 {
            return Err(Error::InvalidInputData);
        } else if (month == 1
            || month == 3
            || month == 5
            || month == 7
            || month == 8
            || month == 10
            || month == 12)
            && day > 31
        {
            return Err(Error::InvalidInputData);
        } else if (month == 4 || month == 6 || month == 9 || month == 11) && day > 30 {
            return Err(Error::InvalidInputData);
        } else if month == 2 && day > 29 {
            // no checking for leap years
            return Err(Error::InvalidInputData);
        }

        let date_number: u16 = match format {
            DateFormat::DDMM => day as u16 * 100 + month as u16,
            DateFormat::MMDD => month as u16 * 100 + day as u16,
        };

        self.display_number(date_number).await?;

        match dot {
            true => self.display_dots([false, true, false, false]).await?, // dot at second position
            false => self.display_dots([false, false, false, false]).await?,
        }

        Ok(())
    }

    /// Set the display mode: Scroll or Rotate (see documentation)
    pub async fn set_mode(&mut self, mode: Mode) -> Result<(), Error<E>> {
        match mode {
            Mode::Rotate => self.write(&[Register::MODE_SETTING, 0]).await?,
            Mode::Scroll => self.write(&[Register::MODE_SETTING, 1]).await?,
        }
        Ok(())
    }

    /// Display data with units (temperature, humidity) and defined thresholds
    async fn display_data(
        &mut self,
        data: i16,
        unit: char,
        lo_thresh: Option<i16>,
        hi_thresh: Option<i16>,
        min_val: i16,
        max_val: i16,
    ) -> Result<(), Error<E>> {
        let mut min_limit = -99;
        let mut max_limit = 999;

        // check if limits can be accepted, if not reset to -99/999
        if min_val > (-100) {
            min_limit = min_val
        }

        if max_val < 1000 {
            max_limit = max_val
        }

        // thresholds initialized as min/max limits
        let mut lo_th: i16 = min_limit;
        let mut hi_th: i16 = max_limit;

        if let Some(val) = lo_thresh {
            lo_th = val
        }

        if let Some(val) = hi_thresh {
            hi_th = val
        }

        // display -LL- and -HH- for data exceding thresholds,
        // e.g. -20 and +50 for a temperature sensor

        if data < min_val || data > max_val {
            for (pos, ch) in "----".chars().enumerate() {
                self.display_char(pos as u8, ch).await?
            }
        } else if data < lo_th {
            for (pos, ch) in "-LL-".chars().enumerate() {
                self.display_char(pos as u8, ch).await?
            }
        } else if data > hi_th {
            for (pos, ch) in "-HH-".chars().enumerate() {
                self.display_char(pos as u8, ch).await?
            }
        } else {
            let hundreds: u8 = (data.abs() / 100) as u8;
            let decimals: u8 = ((data.abs() % 100) / 10) as u8;

            // position 0 (hundreds or minus sign)
            if data < 0 {
                self.display_char(0, '-').await?
            } else if hundreds == 0 {
                self.display_char(0, ' ').await?
            } else {
                self.display_digit(0, hundreds).await?
            }

            // position 1 (decimals)
            if (hundreds == 0 || data < 0) && decimals == 0 {
                self.display_char(1, ' ').await?
                //self.write(&[Register::POSITION_SETTING, 1, ' ' as u8])?
            } else {
                self.display_digit(1, decimals).await?
            }

            // position 2
            self.display_digit(2, (data.abs() % 10) as u8).await?;

            // position 3 (unit)
            self.display_char(3, unit).await?;
        }

        Ok(())
    }

    /// Display temperature between -99 and 999 with a chosen unit, with lower and upper threshold

    pub async fn display_temperature(
        &mut self,
        temperature: i16,
        unit: TempUnits,
        lo_thresh: Option<i16>,
        hi_thresh: Option<i16>,
    ) -> Result<(), Error<E>> {
        let temp_unit = match unit {
            TempUnits::Celsius => 'C',
            TempUnits::Fahrenheit => 'F',
        };

        self.display_data(temperature, temp_unit, lo_thresh, hi_thresh, -99, 999).await?;

        Ok(())
    }

    /// Display humidity in range 0-100, with lower and upper threshold.

    pub async fn display_humidity(
        &mut self,
        humidity: i16,
        lo_thresh: Option<i16>,
        hi_thresh: Option<i16>,
    ) -> Result<(), Error<E>> {
        self.display_data(humidity, 'H', lo_thresh, hi_thresh, 0, 100).await?;

        Ok(())
    }

    /// Helper function to get digits from a 4-digit number
    fn get_digits(number: u16) -> [u8; 4] {
        let mut data = number;
        let mut digits = [0u8; 4];
        digits[0] = (data / 1000) as u8;
        data = data % 1000;
        digits[1] = (data / 100) as u8;
        data = data % 100;
        digits[2] = (data / 10) as u8;
        data = data % 10;
        digits[3] = data as u8;
        digits
    }
}
