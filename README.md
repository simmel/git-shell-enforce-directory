Inspired by http://eagain.net/blog/2007/03/22/howto-host-git.html I needed a way to restrict git into a specific git repository for a specific ssh-key.

# Usage

1. Install
```
$ cargo install --git https://github.com/simmel/git-shell-enforce-directory
```

2. Modify and run:
```
$ cat >> ~/.ssh/authorized_keys <<EOF
command="$HOME/.cargo/bin/git-shell-enforce-directory $PATH_TO_GIT_REPO",no-port-forwarding,no-X11-forwarding,no-agent-forwarding,no-pty ssh-[...] AAAA[...]
EOF
```

# TODO
[ ] [Use logging](https://rust-lang-nursery.github.io/rust-cookbook/logging.html)
  [ ] `-v` enables debug level
  [ ] Create `fatal!` macro which does `error!` and then exits
[ ] Add `--read-only` flag
