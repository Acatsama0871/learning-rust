FROM gitpod/workspace-full

WORKDIR /workspace/rust-book
CMD curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash -s /workspace/rust-book/srcs
