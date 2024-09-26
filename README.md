# xxd-rs

```bash
$ cargo run hello.txt -c 14 -g 2 -u
00000000: 4865 6C6C 6F2C 2078 7864 2D72 7321  Hello, xxd-rs!
0000000e: 0A41 6E20 7878 6420 696D 706C 656D  .An xxd implem
0000001c: 656E 7461 7469 6F6E 2077 7269 7474  entation writt
0000002a: 656E 2069 6E20 5275 7374 20F0 9FA6  en in Rust ...
00000038: 800A                                ..

$ xxd -c 14 -g 2 -u hello.txt
00000000: 4865 6C6C 6F2C 2078 7864 2D72 7321  Hello, xxd-rs!
0000000e: 0A41 6E20 7878 6420 696D 706C 656D  .An xxd implem
0000001c: 656E 7461 7469 6F6E 2077 7269 7474  entation writt
0000002a: 656E 2069 6E20 5275 7374 20F0 9FA6  en in Rust ...
00000038: 800A                                ..
```