extern "C" {
    #[link_name = "pinMode"]
    fn pin_mode(pin: i32, mode: i32);

    #[link_name = "digitalWrite"]
    fn digital_write(pin: i32, state: i32);

    fn delay(millis: i32);
}

#[no_mangle]
unsafe fn _start() {
    _setup();
    loop {
        _loop();
    }
}

const LED: i32 = 0x02;
const OUTPUT: i32 = 0x01;
const LOW: i32 = 0x00;
const HIGH: i32 = 0x01;

unsafe fn _setup() {
    pin_mode(LED, OUTPUT);
}

unsafe fn _loop() {
    digital_write(LED, LOW);
    delay(300);
    digital_write(LED, HIGH);
    delay(300);
}
