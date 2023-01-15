# pingalive
A very tiny Windows command line tool, endlessly pinging a specific server.

### What it is
- It´s a very tiny (3 lines of code) command line executable for Windows, written in Rust.
- It endlessly pings a specific server when executed.
- It´s used for internet connection testing, for my personal use.

### How it works
- It just executes the Windows `ping.exe` with `-t 194.25.2.129` as parameters.
- The IP address (_194.25.2.129_) is Telekom´s main DNS server.
- Telekom is the largest ISP company in Germany.
- That DNS server is a rather stable and longliving server (since more than 40 years now).
- The IP address of that server has never changed (and will never, in my opinion).
- The tool is written in Rust and compiled on 64-bit Windows 10.

### Why it exists
- I wasn´t able to successfully add a batch file (`.bat`) or a link (`.lnk`) to the Windows 10 Taskbar.
- At least not in a way it works properly (maybe i´m just too stupid, but all tries had some issues).
- So i decided to quickly write a tiny executable doing that job instead.
- Then i was able to add that executable to the Taskbar, without any problems.
- That´s the sole reason why this tool exists. ::grin::

### Requirements

- 64-bit Windows

There are not any other special requirements. It is written in Rust and the release binaries are natively compiled with "rustc" for the Windows x64 platform, assuming you are using some 64-bit Windows (and that's quite likely).

#### Have fun.
