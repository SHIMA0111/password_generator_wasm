# Password Generator with WebAssembly
You can generate WebAssembly Application for [password_generator](https://github.com/SHIMA0111/password_generator).
<img width="1729" alt="スクリーンショット 2024-02-27 5 36 27" src="https://github.com/SHIMA0111/password_generator_wasm/assets/140591851/87a4f1d4-30c9-4c2d-9dd8-43b429ab1b14">


# Installation
1. Install wasm-pack: 
   ```shell
   cargo install wasm-pack
   ```
2. Build the application: 
   ```shell
   wasm-pack build --target web --out-name password_generator --out-dir ./static
   ```
3. Set the static dir to the web server like [Apache Http Server](https://httpd.apache.org/) or so \
   Here, I introduce more simple way to check the application. Using [miniserve](https://crates.io/crates/miniserve/0.12.1)
   1. Install miniserve using cargo
      ```shell
      cargo install miniserve
      ```
   2. Start miniserve on the static dir
      ```shell
      miniserve ./static --index index.html
      ```
   3. Access localhost:8080 or your setting port via web browser.

# Usage Example
<img width="1731" alt="スクリーンショット 2024-02-27 5 47 42" src="https://github.com/SHIMA0111/password_generator_wasm/assets/140591851/e579a868-72b3-4ef4-ae2f-12a8bd8211ba">

# License
[MIT License](/LICENSE)