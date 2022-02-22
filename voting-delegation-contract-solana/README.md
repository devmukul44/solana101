# Build Voting Contract on Solana /  Anchor / Rust 

## Resources
- https://docs.soliditylang.org/en/v0.8.11/solidity-by-example.html
- https://openquest.xyz/quest/deploying-the-program-on-to-solana
    - https://gist.github.com/madhavanmalolan/1a88f711fb80632f018dd20ce8338bc8


## Deployment Screenshots

solana CLI configurarions

rpc url is set to local
```
solana config get
```
<img width="399" alt="image" src="https://user-images.githubusercontent.com/15700993/154329737-e88f4de0-befb-4d44-9a35-230d88ae5e91.png">


```
solana balance
```
<img width="159" alt="image" src="https://user-images.githubusercontent.com/15700993/154329574-d7eb8119-b77c-42d4-b40f-aca1f78ca3a6.png">


 ```
 solana-test-validator
 ```

<img width="964" alt="image" src="https://user-images.githubusercontent.com/15700993/154328841-24beb862-ffb7-445c-84fd-1a29b91510c8.png">


building bpf for project

```
cargo build-bpf --manifest-path=./Cargo.toml
```

<img width="1741" alt="image" src="https://user-images.githubusercontent.com/15700993/154329145-b2ea4430-0407-4289-856a-a2a3dbab4990.png">

deploying program to solana blockchain

```
solana program deploy target/deploy/voting.so 
```

<img width="463" alt="image" src="https://user-images.githubusercontent.com/15700993/154329325-0ba5a89f-6d4a-4078-bae0-36ae57ff7cc6.png">
