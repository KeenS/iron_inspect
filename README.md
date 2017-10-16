Inspect the result of iron like below

``` rust
let mut chain = Chain::new(handler);
chain.link_after(Inspect::new(|_req, res| match res {
    Ok(r) => println!("ok: {:?}", r),
    Err(e) => println!("err: {:?}", e),
}));
chain.link_after(Inspect::response(
    |_req, res| println!("response: {:?}", res),
));

chain.link_after(Inspect::error(|_req, err| println!("error: {:?}", err)));
```


# Documentation
[docs.rs](https://docs.rs/iron_inspect)
