# Rust Mini matrix

![](images/1.png)

🦀 rust implementation of mini-matrix - much faster

👨‍💻 Output random 0 and 1 or custom characters with a matrix-like effect

# Installation

## Arch 

rmini-matrix is in the AUR

```bash
yay -S rmini-matrix
```

## Other distros 

### With make

Run make

```bash
# 📂 rmini-matrix/
make
```

### Or

clone github repo

```bash
git clone https://github.com/SkwalExe/rmini-matrix
```

build with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```bash
# 📂 rmini-matrix/
cargo build --release
```

Move the binary

```bash
# 📂 rmini-matrix/
sudo cp target/release/lsd-print /usr/bin/lsd-print
```

# Usage 

![](images/usage.png)

## Example

`rmini-matrix -c 20 --custom-chars "abcdefghijklmnopqrstuvwxyz"`

![](images/2.png)

## Example with polybar  

```ini
[module/matrix]
type = custom/script
exec = rmini-matrix -c 20 -s 0.07
tail = true
```

![](images/screenshot.gif)

# Uninstall 🗑

## With make

Run make uninstall

```bash
# 📂 rmini-matrix/
make uninstall
```

## Or

Just remove the binary

```bash
sudo rm /usr/bin/rmini-matrix
```

# final

If you have any probleme, don't hesitate to open an issue

# contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

<a href="https://github.com/SkwalExe#ukraine"><img src="https://raw.githubusercontent.com/SkwalExe/SkwalExe/main/ukraine.jpg" width="100%" height="15px" /></a>