#playenv
> 打印环境变量的程序

# test
```sh
cargo test
```

# setup
```sh
cargo build --release
cp dist/release/playenv /usr/local/bin/
```

# example
```sh
playenv HOME PATH path non_path

[
        HOME ==> /home/renshan
        PATH ==> /home/renshan/bin:/home/renshan/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin:/home/renshan/Development/Golang/bin:/home/renshan/.golang/bin:/usr/local/php/bin:/home/renshan/.Android/tools:/home/renshan/.Android/platform-tools://home/renshan/.cargo/bin
    non_path ==> None
        path ==> None
]

```
