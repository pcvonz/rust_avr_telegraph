#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::port;
use ruduino::legacy::serial;
const DIT: u32 = 40;
const DAH: u32 = DIT * 3;
const SPACE: u32 = DIT * 7;


pub fn pulse(length: u32) {
        port::B5::set_high();
        port::D5::set_high();
        ruduino::delay::delay_ms(length);

        port::B5::set_low();
        port::D5::set_low();

        ruduino::delay::delay_ms(DIT);
}

pub fn pulse_character(morse: &str) {
    for i in morse.chars() {
        match i {
            '.' => {
                pulse(DIT);
            }
            '-' => {
                pulse(DAH);
            }
            _ => {
            }
        }
    }
    ruduino::delay::delay_ms(DAH);
}

// Alphabet
// 
//     A .-
pub fn a() {
    pulse_character(".-");
}
//     B
pub fn b() {
    pulse_character("-...");
}
//     C
pub fn c() {
    pulse_character("-.-.");
}
//     D
pub fn d() {
    pulse_character("-..");
}
//     E
pub fn e() {
    pulse_character(".");
}
//     F
pub fn f() {
    pulse_character("..-.");
}
//     G
pub fn g() {
    pulse_character("--.");
}
//     H
pub fn h() {
    pulse_character("....");
}
//     I
pub fn i() {
    pulse_character("..");
}
//     J
pub fn j() {
    pulse_character(".---");
}
//     K
pub fn k() {
    pulse_character("-.-");
}
//     L
pub fn l() {
    pulse_character(".-..");
}
//     M
pub fn m() {
    pulse_character("--");
}
//     N
pub fn n() {
    pulse_character("-.");
}
//     O
pub fn o() {
    pulse_character("---");
}
//     P
pub fn p() {
    pulse_character(".--.");
}
//     Q
pub fn q() {
    pulse_character("--.-");
}
//     R
pub fn r() {
    pulse_character(".-.");
}
//     S
pub fn s() {
    pulse_character("...");
}
//     T
pub fn t() {
    pulse_character("-");
}
//     U
pub fn u() {
    pulse_character("..-");
}
//     V
pub fn v() {
    pulse_character("...-");
}
//     W
pub fn w() {
    pulse_character(".--");
}
//     X
pub fn x() {
    pulse_character("-..-");
}
//     Y
pub fn y() {
    pulse_character("-.--");
}
//     Z
pub fn z() {
    pulse_character("--..");
}
// 
// Digit
// 
//     0
pub fn zero() {
    pulse_character("-----");
}
//     1
pub fn one() {
    pulse_character(".----");
}
//     2
pub fn two() {
    pulse_character("..---");
}
//     3
pub fn three() {
    pulse_character("...--");
}
//     4
pub fn four() {
    pulse_character("....-");
}
//     5
pub fn five() {
    pulse_character(".....");
}
//     6
pub fn six() {
    pulse_character("-....");
}
//     7
pub fn seven() {
    pulse_character("--...");
}
//     8
pub fn eight() {
    pulse_character("---..");
}
//     9
pub fn nine() {
    pulse_character("----.");
}
// 
// Punctuation Mark
// 
//     f
pub fn full_stop() {
    pulse_character(".-.-.-");
}
//     c
pub fn comma() {
    pulse_character("--..--");
}
//     c
pub fn colon() {
    pulse_character("---...");
}
//     q
pub fn question_mark() {
    pulse_character("..--..");
}
//     a
pub fn apostrophe() {
    pulse_character(".----.");
}
//     h
pub fn hyphen() {
    pulse_character("-....-");
}
//     f
pub fn forward_slash() {
    pulse_character("-..-.");
}
//     p
pub fn parentheses() {
    pulse_character("-.--.-");
}
//     q
pub fn quotation_mark() {
    pulse_character(".-..-.");
}
//     a
pub fn at_sign() {
    pulse_character(".--.-.");
}
//     e
pub fn equals_sign() {
    pulse_character("-...-");
}
// 


#[no_mangle]
pub extern fn main() {
    port::B5::set_output();
    port::D5::set_output();
    const CPU_FREQUENCY_HZ: u64 = 16_000_000;
    const BAUD: u64 = 9600;
    const UBRR: u16 = (CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;
    h();
    e();
    l();
    l();
    o();

    serial::Serial::new(UBRR)
        .character_size(serial::CharacterSize::EightBits)
        .mode(serial::Mode::Asynchronous)
        .parity(serial::Parity::Disabled)
        .stop_bits(serial::StopBits::OneBit)
        .configure();

    loop {
        let character = serial::receive();
        match character {
            b'A' => {
                a();
            }
            b'B' => {
                b();
            }
            b'C' => {
                c();
            }
            b'D' => {
                d();
            }
            b'E' => {
                e();
            }
            b'F' => {
                f();
            }
            b'G' => {
                g();
            }
            b'H' => {
                h();
            }
            b'I' => {
                i();
            }
            b'J' => {
                j();
            }
            b'K' => {
                k();
            }
            b'L' => {
                l();
            }
            b'M' => {
                m();
            }
            b'N' => {
                n();
            }
            b'O' => {
                o();
            }
            b'P' => {
                p();
            }
            b'Q' => {
                q();
            }
            b'R' => {
                r();
            }
            b'S' => {
                s();
            }
            b'T' => {
                t();
            }
            b'U' => {
                u();
            }
            b'V' => {
                v();
            }
            b'W' => {
                w();
            }
            b'X' => {
                x();
            }
            b'Y' => {
                y();
            }
            b'Z' => {
                z();
            }
            b'a' => {
                a();
            }
            b'b' => {
                b();
            }
            b'c' => {
                c();
            }
            b'd' => {
                d();
            }
            b'e' => {
                e();
            }
            b'f' => {
                f();
            }
            b'g' => {
                g();
            }
            b'h' => {
                h();
            }
            b'i' => {
                i();
            }
            b'j' => {
                j();
            }
            b'k' => {
                k();
            }
            b'l' => {
                l();
            }
            b'm' => {
                m();
            }
            b'n' => {
                n();
            }
            b'o' => {
                o();
            }
            b'p' => {
                p();
            }
            b'q' => {
                q();
            }
            b'r' => {
                r();
            }
            b's' => {
                s();
            }
            b't' => {
                t();
            }
            b'u' => {
                u();
            }
            b'v' => {
                v();
            }
            b'w' => {
                w();
            }
            b'x' => {
                x();
            }
            b'y' => {
                y();
            }
            b'z' => {
                z();
            }
            b'0' => {
                zero();
            }
            b'1' => {
                one();
            }
            b'2' => {
                two();
            }
            b'3' => {
                three();
            }
            b'4' => {
                four();
            }
            b'5' => {
                five();
            }
            b'6' => {
                six();
            }
            b'7' => {
                seven();
            }
            b'8' => {
                eight();
            }
            b'9' => {
                nine();
            }
            b'.' => {
                full_stop();
            }
            b',' => {
                comma();
            }
            b':' => {
                colon();
            }
            b'?' => {
                question_mark();
            }
            b'\'' => {
                apostrophe();
            }
            b'-' => {
                hyphen();
            }
            b'/' => {
                forward_slash();
            }
            b'(' => {
                parentheses();
            }
            b')' => {
                parentheses();
            }
            b'"' => {
                quotation_mark();
            }
            b'@' => {
                at_sign();
            }
            b'=' => {
                equals_sign();
            }
            b' ' => {
                ruduino::delay::delay_ms(SPACE);
            }
            _ => {}
        }
        
        serial::transmit(character);
    }
}
