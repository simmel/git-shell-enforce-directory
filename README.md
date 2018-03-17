Inspired by http://eagain.net/blog/2007/03/22/howto-host-git.html I needed a way to restrict git into a specific git repository for a specific ssh-key.

# Usage

Modidy and run:
```
cat >> ~/.ssh/authorized_keys <<EOF
command="$HOME/.cargo/bin/git-shell-enforce-directory $PATH_TO_GIT_REPO",no-port-forwarding,no-X11-forwarding,no-agent-forwarding,no-pty ssh-[...] AAAA[...]
EOF
```
