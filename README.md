# esp-idf-n-hal
This is a repository that includes both the C sources from the IDF and the HAL being developed in pure rust. Since it seems like it will take a while to get all the features in RUST this means you can start making real apps for the ESP in rust today!

Why do I use rust for embedded development? All the usual reasons, but I really like the panic and error messages like the following:

```
*** PanicInfo { payload: Any, message: Some(index out of bounds: the len is 4 but the index is 4), location: Location { file: "/home/yonasj/dev/tsim/tsim-common/src/pjon/driver/bit_bang.rs", line: 388, col: 17 } }
```

# History

I am building on the work of the following:
 - https://github.com/ctron/rust-esp-template
