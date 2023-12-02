# opaque-wallet

Simple Bitcoin wallet with a graphical user interface, built in Rust.

To maintain proper security, must be connected to a bitcoin-core node. Usually this is ran locally unless you are using a service that offers a remote bitcoin-core client.\

Must be ran from the folder that is set as the datadir of the bitcoin-core node.\

Bitcoin-core node should not have --rpcuser or --rpcpassword set, either at startup or in the config file.\

These are both no longer the preferred way of authentication. Opaque Wallet will handle authentication itself, so you don't need to worry about authentication paraments when starting the bitcoin-core node.\

Simply ./bitcoind or ./bitcoind --datadir=your/preferred/data/directory will work fine (along with any other parameters you usually pass in)
