//PDX short identifier: AGPL-3.0
//Copyright (C) 2026 SAP2B

#[macro_export]
macro_rules! bitformat {
    ($( $s:expr ),* $(,)?) => {{
        const LEN: usize = 0 $( + $s.len() )*;
        const BUF: [u8; LEN] = {
            let mut buf = [0u8; LEN];
            let mut pos = 0;
            $(
                let bytes = $s.as_bytes();
                let mut i = 0;
                while i < bytes.len() {
                    buf[pos] = bytes[i];
                    pos += 1;
                    i += 1;
                }
            )*
            buf
        };

        const STR: &str = match core::str::from_utf8(&BUF) {
            Ok(valid_str) => valid_str,
            Err(_) => panic!("Invalid UTF-8 in const concatenation"),
        };

        STR
    }};
}

#[macro_export]
macro_rules! bitif {
    (let $pattern:pat = $expr:expr => $if_true:expr, $if_false:expr) => {
        if let $pattern = $expr {
            $if_true
        } else {
            $if_false
        }
    };
    ($condition:expr => $if_true:expr, $if_false:expr) => {
        if $condition { $if_true } else { $if_false }
    };
}

#[macro_export]
macro_rules! bitmatch {
    ($val:expr => { $( $pattern:pat $(if $guard:expr)? => $result:expr ),* $(,)? }) => {
        $crate::bitmatch!($val => $( $pattern $(if $guard)? => $result ),*)
    };

    ($val:expr => $( $pattern:pat $(if $guard:expr)? => $result:expr ),* $(,)?) => {{
        const _ARMS_COUNT: usize = 0 $( + (stringify!($pattern), 1).1 )*;
        const _: () = assert!(_ARMS_COUNT <= 8, "bitmatch! supports a maximum of 8 arms");

        match $val {
            $(
                $pattern $(if $guard)? => $result,
            )*
        }
    }};
}

const _: () = {
    enum Status {
        Ok = 200,
        NotFound = 404,
        ServerError = 500,
    }

    let st = Status::Ok;

    let status = bitmatch!(st =>
        Status::Ok => "Ok",
        Status::NotFound => "NotFound",
        Status::ServerError => "ServerError"
    );
    assert!(matches!(status.as_bytes(), b"Ok"));

    let tnumber = bitif!(true => 10, 20);
    let fnumber = bitif!(false => 10, 20);
    assert!(tnumber == 10);
    assert!(fnumber == 20);

    let unwrapped_some = bitif!(let Some(x) = Some(42) => x, 0);
    let unwrapped_none = bitif!(let Some(x) = None => x, -1);

    assert!(unwrapped_some == 42);
    assert!(unwrapped_none == -1);

    let value = "hello";
    let status_code = bitmatch!(value.as_bytes() => {
        b"hello" => 1,
        b"world" => 2,
        _ => 0,
    });
    assert!(status_code == 1);

    let concatenated = bitformat!("Hello, ", "world!");
    assert!(matches!(concatenated.as_bytes(), b"Hello, world!"));
};
