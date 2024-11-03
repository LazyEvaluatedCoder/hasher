# Rust Crypto Hasher

Rust Crypto Hasher is a CLI tool to generate crypto hashes from text, files and stdin.

### Supported Hash Algorithms

- md5
- sha1
- sha256
- sha384
- sha512
- sha3_256
- sha3_384
- sha3_512

## Usage

**Print usage**
```
hasher help
```

**File hashing example**
```
hasher file <PATH>
```

**Text hashing example**
```
hasher text <TEXT>
```

**Stdin hashing example**
```
echo "hello" | hasher stdin
```

### Options

| flag | Description |
|-|-|
| -a, --algorithm | Select hash algorithm [default: sha256] |
| --all | Use all available hash algorithms
| -v, --verbose | Displays input path or size |