# Hermes

```text
 __
/\ \
\ \ \___      __   _ __    ___ ___      __    ____
 \ \  _ `\  /'__`\/\`'__\/' __` __`\  /'__`\ /',__\
  \ \ \ \ \/\  __/\ \ \/ /\ \/\ \/\ \/\  __//\__, `\
   \ \_\ \_\ \____\\ \_\ \ \_\ \_\ \_\ \____\/\____/
    \/_/\/_/\/____/ \/_/  \/_/\/_/\/_/\/____/\/___/


    Hermes: A concurrent web-server written in Rust
```

Hermes is a concurrent web server written in Rust, with a simple thread pool implementation and graceful shutdown.

## Usage

```sh
cargo install --git https://github.com/anirudhsudhir/hermes.git
hermes #[port(optional, defaults to 8000)]

#Over port 8080
hermes 8080
```
