1. Init library
```
cargo init demo --lib
cd demo 
```

2. Run local validator
```
cargo add solana-program
cargo add borsh
```

3. Listen to local host logs
```
solana logs --url localhost
```

4. Add solana-program dep
```
cargo add solana-program
```

5. Add lib dependency to Cargo toml
```
[lib]
name = "demo"
crate-type = ["cdylib", "lib"]
```

6. Build program
```
cargo build-bpf
```

7. Deploy program. Once your Solana program has been deployed (and the transaction finalized), the above command will output your program's public address (aka its "program id").
```
solana program deploy ./target/deploy/demo.so
```

8. Execute transaction
```
npm start
```
