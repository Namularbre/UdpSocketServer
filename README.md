# UdpSockerServer

This program listen on an ip for UDP message and print them. You can configure it to forward a message from an address
to another, are blacklist a list of address.

## Build

Use this command

````
cargo build
````

## Run

Use this command, it will compile the source code to an executable
````
cargo run
````

## Installation

Go to release and download the .exe file, then run it. A version for linux will be available soon.

## Configuration

You have a config.json file, like this: 
````
{
  "forwarding": [
    {
      "from": "127.0.0.1:58545",
      "to": "127.0.0.1:12346",
      "doReverse": true
    }
  ],
  "blacklist": [
    "127.0.0.1:11111",
    "127.0.0.1:55527"
  ]
}
````

The blacklist field is easy to understand: all addresses written are ignored by the server.
The forwarding field is a list of object indicating that if the server receive a message from the address at "from" field
it should send it back to "to" address. The doReverse field, if set to true, will permit that when we a message from "to" address,
it is sent to "from" address.

## Authors

[Namularbre](https://github.com/Namularbre/)
