# paxit
Project in Rust to apply paxctl to all binaries in $PATH. 

This should work on any nix system, but there are a couple dependancies:

```
cargo
paxctl
```

A few months after completing this project, the package for paxctl was removed from new Debian distros, but still remains here:

https://github.com/ystk/debian-paxctl

Once that is built and installed you can clone this repository like so:

```git clone https://github.com/mephistolist/paxit.git```

Then change into the directory:

```cd paxit```

Finally build with the following:

```cargo build --release```

You can then copy the binary to /usr/bin or your prefered location:

```cp target/release/paxit /usr/bin```

After this you can run like so:

```
$ sudo paxit
[sudo] password for me:
About to commit 'paxctl -PEMRXS' to all ELF binaries in $PATH directories.
Would you like to proceed? [Y/N]
```

After this you should find any binaries in the folders of your $PATH will have the following flags from paxctl applied:

```
$ paxctl -v /bin/bash                
PaX control v0.9
Copyright 2004,2005,2006,2007,2009,2010,2011,2012,2014 PaX Team <pageexec@freemail.hu> 

- PaX flags: P-S-M--xE-R- [/bin/bash]
	PAGEEXEC is enabled
	SEGMEXEC is enabled
	MPROTECT is enabled
	RANDEXEC is disabled
	EMUTRAMP is enabled
	RANDMMAP is enabled
```

Keep in mind RANDEXEC will not work with kernels past 2.6. This was due to RANDEXEC causing more problems than it solved. So it is not needed. 

Remember to only run this with sudo or doas. Root or non-root usage may be problematic. 
