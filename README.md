# `ipc`

A straight forward CLI utility to display all the relevant information
associated with an IPv4 Address. I originally wrote it because dealing with IP
address manually got old quickly.

# Usage

Simply pass an IP address as the first argument:

```
$ ipc 192.168.10.5

Address              : 192.168.10.5    11000000.10101000.00001010.00000101
Network Address      : 192.168.10.0    11000000.10101000.00001010.00000000
Broadcast Address    : 192.168.10.255  11000000.10101000.00001010.11111111
Prefix length        : 24
Class                : C
```

You can also pass a prefix length:

```
$ ipc 192.168.10.5/20

Address              : 192.168.10.5    11000000.10101000.00001010.00000101
Network Address      : 192.168.0.0     11000000.10101000.00000000.00000000
Broadcast Address    : 192.168.15.255  11000000.10101000.00001111.11111111
Prefix length        : 20
```

