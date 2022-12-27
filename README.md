# rust-ppm-graphics

This is a really simple graphics library written in rust. Basically, 
we generate images byte-to-byte and save them using the [ppm format](https://netpbm.sourceforge.net/doc/ppm.html)

## how do i run it?
Running this project is real easy, just make sure you got the [rust programming language](https://www.rust-lang.org/) properly 
installed on your machine, then at the root folder, run the following command: 

```bash
cargo run
```

that one is going to compile the whole project and run it automatically. At the end, you should have got 
a *out.ppm* file, which is your generated image.

## Examples
A really simple example of what such a library can generate is a flag. If you go to the code, you will see the
_draw_japan_flag_ function, the result is the following: 

<img src="https://github.com/KPMGE/rust-ppm-graphics/blob/main/examples/japan.ppm" height="600"/>
