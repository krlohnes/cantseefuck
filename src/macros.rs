#[macro_export]
macro_rules! assert_stdout_eq {
    ($test:expr, $expected:literal) => {{
        use gag::BufferRedirect;
        use std::io::Read;

        let mut buf = BufferRedirect::stdout().unwrap();

        $test;

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        drop(buf);

        assert_eq!(&output[..], $expected);
    }};
}
