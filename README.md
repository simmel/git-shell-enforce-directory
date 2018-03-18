Restrict an ssh-key via force-command to a specific git repository.

Inspired by http://eagain.net/blog/2007/03/22/howto-host-git.html

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
