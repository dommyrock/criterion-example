#### Build.rs

In this project i use basic "string concat" to build up multi executable bin folder **src/bin**.
> This is required by the cargo-flamegraph cli (which uses **inferno** under the hood). 

---
Note: 
The right way to do is to use **proc_macro** to read token stream.
When **fn exec(...)** is detected, macro should read it's signature/params and then decide whay the main body should look like when copied over to src/bin directory.
