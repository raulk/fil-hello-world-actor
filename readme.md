# Filecoin Hello World Actor

This repo contains a simple "Hello, World!" Filecoin actor, written in Rust. It is provided as an example for early builders to get experimenting with developing and deploying actors for the Filecoin Virtual Machine.

## Instructions

1. Compile the actor to wasm.

```
cargo build
```

2. Set up a Lotus devnet

You should use checkout the `experimental/fvm-m2` branch, and build from source -- follow the instructions [here](https://lotus.filecoin.io/developers/local-network/)

3. _Install_ the actor.

```
lotus chain install-actor <path-to-wasm-bytecode>

```

Here `path-to-wasm-bytecode = ./target/debug/wbuild/fil-hello-world-actor/fil-hellow-world-actor.compact.wasm`

You should see an actor code CID printed.

4. _Instantiate_ the actor.

```
lotus chain create-actor <code-cid> <encoded-params>
```

Here the `code-cid` can be copy-pasted from the output of the previous step, and `encoded-params` can be left empty (since the constructor does not take any parameters).

This command sends a message to the network invoking a method on the builtin init actor.

5. _Invoke_ the actor.

```
lotus chain invoke <address> <method_num>
```

Here the `address` can be copy-pasted from the output of the previous step, and `method_num` should be 2.

You can pipe the output through `base64` to decode it:

```
echo "<output>" | base64 -d
```

If you invoke the method again, you should see a different output due to a change of state of the actor.

## Next steps

You can then make some interesting changes to the actor and repeat the process, and go on to develop your own use case or developer tooling based on what you learn.

You can feedback your experience via the fvm early builders working group.
