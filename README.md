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
[polybar-config]: https://github.com/timvisee/dotfiles/blob/60ca6fff90e3ef77ac56b417706d3ad4d669ea1b/polybar/config#L478-L491
