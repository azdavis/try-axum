# try-axum

an investigation in actually deploying an actual app with rust on fly.io

## what i did

- heard about fly.io via blog posts to hacker news
- investigated fly.io for rust
- understand that the best way is to make a dockerfile for my app
- investigated which docker to install (cli vs desktop app)
- chose desktop app because that has the daemon
- installed it
- launched it (the daemon) in the background by launching the docker desktop app
- wrote a dockerfile and some scripts for it for a hello-world app
- investigated which rust web thing to use
- chose axum
- made an axum hello world
- confirmed it works locally
- confirmed it works on docker locally
- installed fly cli with brew
- created account
- create fly.toml with `fly launch`
- deployed my app with `fly deploy`
- confirmed it works on fly.io
- think about what db system to use
- postgres seems good, relational, and well supported by fly.io
- run `brew install postgres`
- make a db: `initdb db` makes a database directory called `db`
- try `postgres -D db`, it starts up
- while that's running, run `createuser -s postgres` in another tab
- integrate deps and example code from tokio-postgres axum example
- tried making tables with postgres
- use `pg_ctl start -D db` to start `postgres` in background
- use `psql -U postgres` to bring up sql prompt
- tried this to make some example data:

```sql
create table if not exists points (x int, y int);
insert into points values (1, 2), (3, 5), (6, 1);
```

- fiddled around with axum to mutate the db

## todo

- figure out axum more
- use postgres locally/with docker?
- use postgres with fly
- seems like it's its own app that i connect to somehow
- how to do migrations to set up tables?
