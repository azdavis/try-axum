# try-axum

an investigation in actually deploying an actual app with rust on fly.io

## what i did

- heard about fly.io via blog posts to hacker news
- investigated fly.io for rust
- understand that the best way is to make a dockerfile for my app
- investigated which docker to install (cli vs desktop app)
- chose desktop app because that has the daemon
- installed it
- launched it (the daemon) in the background
- wrote a dockerfile and some scripts for it for a hello-world app
- investigated which rust web thing to use
- chose axum
- made an axum hello world
- confirmed it works locally
- confirmed it works on docker locally
- installed fly cli with brew
- created account
- deployed example app
- deployed my app
- confirmed it works on fly.io
