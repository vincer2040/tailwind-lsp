# tailwind-lsp
tailwind lsp implemented in rust

## Why 

The inevitable Rust rewrite hits tailwind lsp. Most would agree that the current implementation of 
tailwind lsp is kind of slow. This project aims to solve that. 

## still a lot of work to be done on this, and is currently not even an MVP. 

## Goals 

1. provide completion for all tailwindcss classes

2. provide hover information for all tailwindcss classes

## usage/contributing

1. clone

```
git clone git@github.com:vincer2040/tailwind-lsp.git
```

2. build in watch mode 

```
cargo watch -x build 
```

3. Use [lsp-debug-tools](https://github.com/ThePrimeagen/lsp-debug-tools.nvim) for better debugging experience

