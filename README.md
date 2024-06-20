# signalfun: SIG_IGN is inherited across exec*()

If a process ignores signals by setting one to `SIG_IGN` instead of `SIG_DFL`, it will propagate to processes across `exec*()` system calls. If a process sets an explicit signal handler, it will be reset to the default. This is a demonstration program of this.

The parent process prints the results from `sigaction()`, then calls `sigaction(SIGINT, ...)` to set the action to `SIG_IGN`. It then executes itself again, which prints the status in child.

## Output

```
parent starting (num_args=1)
parent before sigint_sigaction.sa_sigaction=0 (SIG_IGN=1, SIG_DFL=0)
parent before sigint_sigaction.sa_flags: 0
parent called sigaction to set SIGINT to SIG_IGN; spawning child ...
child starting (num_args=2)
child before sigint_sigaction.sa_sigaction=1 (SIG_IGN=1, SIG_DFL=0)
child before sigint_sigaction.sa_flags: 0
child exiting
parent exiting
```

Note that the parent sees `sigint_sigaction.sa_sigaction=0 == SIG_DFL` but the child sees `sigint_sigaction.sa_sigaction=1 == SIG_IGN`.


## Manual pages

This is documented, but it the man pages must be read very carefully.


`man 2 execve`:

```
All process attributes are preserved during an execve(), except the following:
*  The dispositions of any signals that are being caught are reset to the default (signal(7)).
```

The key words there are "*signals that are being caught*". Ignored signals are not being caught.

`man 7 signal`:
```
A child created via fork(2) inherits a copy of its parent's signal dispositions.  During an
execve(2), the dispositions of handled signals are reset to the default; the dispositions of
ignored signals are left unchanged.
```

The key part here is "the dispositions of *ignored signals are left unchanged*".