# peperone

A stupidly simple command-line stopwatch.
No blocking, no daemons, just some file.

I wanted a simple stopwatch in [polybar] for basic project time tracking, and so
I built `peperone`. The 'peperone' name is a silly play on 'pomodoro', a well known
timer and technique for project time tracking.

```bash
$ peperone new mytimer

$ peperone show mytimer
00:00

$ peperone list
mytimer

# -- 21 seconds later --

$ peperone show mytimer
00:21

$ peperone tail mytimer
00:21
00:22
00:23
00:24
00:25
00:26
# ...

$ peperone remove mytimer
```

See my polybar config [here][polybar-config].

## FAQ

#### Why Rust?
I like Rust.

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.

[polybar]: https://polybar.github.io/
[polybar-config]: https://github.com/timvisee/dotfiles/blob/88795797d7ab0f3af350108480b5e0e1caa737f5/polybar/config#L478-L486
