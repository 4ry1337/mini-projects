# IP Sniffer

A multithreaded TCP port scanner. Despite the name, it does not sniff packets.

**Status: runs.**

The scanner tries a TCP connection to every port from 1 to 65535. Thread `i`
checks ports `i + 1`, `i + 1 + threads` and so on. It prints a dot for each
open port it finds. At the end, it lists the open ports in order.

Only scan hosts that you own or have permission to test.

## Usage

```sh
cargo run -- -s <threads> <ip>
```

For example, to scan your own machine with 100 threads:

```sh
cargo run -- -s 100 127.0.0.1
```

The output is a line of dots, then one `<port> is open` line per open port.

| Flag | Action |
| --- | --- |
| `-h`, `--help` | Print the help text |
| `-s`, `--scan` | Scan an address. It takes the thread count, then the IP. |

## Known Issues

- The help text says `-s [IPv4 Address]`, but the real usage is
  `-s <threads> <ip>`.
- `-s` with a thread count but no IP panics with "index out of bounds".
- An IPv6 address parses, but every connection fails. The code builds
  `addr:port` without the `[ ]` that IPv6 needs.
- Port 65535 is never scanned unless the thread count is 65535. Each thread
  stops when its next port would be 65535 or higher.
- A thread count of 0 scans nothing.
- Each connection uses the OS default timeout, so filtered ports can make a
  scan slow.

## License

[MIT](../LICENSE) © 2024-2026 Rakhat Yskak
