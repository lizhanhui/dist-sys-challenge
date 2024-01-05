# Dist-Sys-Challenge
https://fly.io/dist-sys/

# Echo

```sh
~/Downloads/maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10
```

# Unique-Ids
```sh
~/Downloads/maelstrom/maelstrom test -w unique-ids --bin target/debug/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
```

# Broadcast

## Single-Node Broadcast

```sh
~/Downloads/maelstrom/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 1 --time-limit 20 --rate 10
```

## Multi-Node Broadcast

```sh
~/Downloads/maelstrom/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 5 --time-limit 20 --rate 10
```

## Fault Tolerant Broadcast

```sh
~/Downloads/maelstrom/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 5 --time-limit 20 --rate 10 --nemesis partition
```