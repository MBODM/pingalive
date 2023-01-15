# pingalive
A very tiny Windows command line tool, endlessly pinging a specific server.

### What it is
- It´s a very tiny (3 lines of code) command line executable for Windows, written in Rust.
- It endlessly pings a specific server when executed.
- It´s used for internet connection testing, for my personal use.

### How it works
- It just executes the Windows command line tool `ping.exe` with `-t 194.25.2.129` as parameters.
- The IP address "194.25.2.129" is Telekom´s main DNS server.
- Telekom is the largest ISP company in Germany.
- That DNS server is rather stable and longliving (since more than 40 years now).
- It is written in Rust and compiled on 64-bit Windows 10.

### Why it exists
It solely exists, because i wasn´t able to add a batch file (".bat") or a link (".lnk") to the Windows 10 TaskBar in a way it works properly. So i decided to quickly write a small executable, doing that job. Then i was able to easily add that executable to the TaskBar. That´s the only reason why this tool exists. :)

### Requirements

- 64-bit Windows

There are not any other special requirements. It is written in Rust and the release binaries are natively compiled with "rustc" for the Windows x64 platform, assuming you are using some 64-bit Windows (and that's quite likely).

#### Have fun.
