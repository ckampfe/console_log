#[macro_export]
macro_rules! console_log {
    () => {
        web_sys::console::log_0();
    };
    ($e1:expr) => {
        web_sys::console::log_1(&$e1.into());
    };
    ($e1:expr, $e2:expr) => {
        web_sys::console::log_2(&$e1.into(), &$e2.into());
    };
    ($e1:expr, $e2:expr, $e3:expr) => {
        web_sys::console::log_3(&$e1.into(), &$e2.into(), &$e3.into());
    };
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr) => {
        web_sys::console::log_4(&$e1.into(), &$e2.into(), &$e3.into(), &$e4.into());
    };
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr) => {
        web_sys::console::log_5(
            &$e1.into(),
            &$e2.into(),
            &$e3.into(),
            &$e4.into(),
            &$e5.into(),
        );
    };
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr) => {
        web_sys::console::log_6(
            &$e1.into(),
            &$e2.into(),
            &$e3.into(),
            &$e4.into(),
            &$e5.into(),
            &$e6.into(),
        );
    };
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr) => {
        web_sys::console::log_7(
            &$e1.into(),
            &$e2.into(),
            &$e3.into(),
            &$e4.into(),
            &$e5.into(),
            &$e6.into(),
            &$e7.into(),
        );
    };
}

#[cfg(test)]
mod tests {
    use super::console_log;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn it_works() {
        console_log!();
        console_log!("one");
        console_log!("one", "two");
        console_log!("one", "two", "three");
        console_log!("one", "two", "three", "four");
        console_log!("one", "two", "three", "four", "five");
        console_log!("one", "two", "three", "four", "five", "six");
        console_log!("one", "two", "three", "four", "five", "six", "seven");
        console_log!("one", String::from("two"), true, 4u8, 5u16, 6u32, 7.0);
    }
}
