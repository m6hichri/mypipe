## mypipe

You have to recode a small pipe-like program, working like this:

```shell
$ mypipe --in fortune --out cowsay
```

```
 _______________________________________
/ Q: What's tiny and yellow and very,   \
| very, dangerous? A: A canary with the |
\ super-user password.                  /
 ---------------------------------------
          \   ^__^
           \  (oo)\_______
              (__)\       )\/\
                  ||----w |
                  ||     ||
```

You can use <https://clap.rs> to parse the command-line arguments, and also follow the guide <https://rust-lang-nursery.github.io/cli-wg/>
