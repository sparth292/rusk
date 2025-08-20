curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
sudo apt install build-essential
~/.cargo/bin/rustc --version
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
cargo new get-dependencies
cd get-dependencies
cargo add rand@0.8.5 trpl@0.2.0
cargo install cargo-edit

