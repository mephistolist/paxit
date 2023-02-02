# paxit
Project in Rust to apply paxctl to all binaries in $PATH. 

This should work on any nix system, but there are a couple dependancies:

cargo<br>
paxctl<br>

Once these are installed you can clone this repository like so:

git clone https://github.com/mephistolist/paxit.git

Then change into the directory:

cd paxit

Finally build with the following:

cargo build --release

You can then copy the binary to /usr/bin or your prefered location:

cp target/release/paxit /usr/bin

After this you can run like so:

$ sudo paxit
[sudo] password for me:<br>
About to commit 'paxctl -PEMRXS' to all ELF binaries in $PATH directories.<br>
Would you like to proceed? [Y/N] <br>

After this you should find any binaries in the folders of your $PATH will have the following flags from paxctl applied:

$ paxctl -v /bin/bash                
PaX control v0.9
Copyright 2004,2005,2006,2007,2009,2010,2011,2012,2014 PaX Team <pageexec@freemail.hu> <br>

- PaX flags: P-S-M--xE-R- [/bin/bash]<br>
	PAGEEXEC is enabled<br>
	SEGMEXEC is enabled<br>
	MPROTECT is enabled<br>
	RANDEXEC is disabled<br>
	EMUTRAMP is enabled<br>
	RANDMMAP is enabled<br>

Keep in mind RANDEXEC will not work with kernels past 2.6. This was due to RANDEXEC causing more problems than it solved. So it is not needed. 

Remember to only run this with sudo or doas. Root or non-root usage may be problematic. 
