# APEC Certificate Program

A Solana blockchain-based certificate issuance and verification system built with Anchor framework. This program enables educational providers to issue verifiable, non-transferable certificates as NFTs using Solana's Token-2022 standard with Merkle tree proof verification.

## Overview

The APEC Certificate Program provides a decentralized solution for issuing and verifying educational certificates on the Solana blockchain. Certificates are issued as soulbound tokens (non-transferable NFTs) with metadata, ensuring authenticity and preventing fraud.

### Key Features

- **Provider Management**: Educational institutions can register as certificate providers
- **Course Creation**: Providers can create and manage multiple courses
- **Course Enrollment**: Students can enroll in courses and receive enrollment tokens
- **Merkle Tree Verification**: Efficient batch certificate issuance using Merkle tree proofs
- **Soulbound Certificates**: Non-transferable NFT certificates using Token-2022 standard
- **Metadata Support**: Rich certificate metadata including name, symbol, and URI

## Architecture

### Program Structure

```
programs/apec-cert/
├── src/
│   ├── lib.rs              # Program entry point
│   ├── constants.rs        # Program constants and seeds
│   ├── error.rs            # Custom error definitions
│   ├── state/              # Account state definitions
│   │   ├── provider.rs     # Provider account
│   │   ├── course.rs       # Course account
│   │   └── cert_proof.rs   # Certificate proof account
│   ├── instructions/       # Program instructions
│   │   ├── init_provider.rs
│   │   ├── create_course.rs
│   │   ├── enroll_course.rs
│   │   ├── create_cert_proof.rs
│   │   └── claim_cert.rs
│   └── utils/              # Utility functions
```

### Core Instructions

1. **init_provider**: Initialize a new certificate provider
2. **create_course**: Create a new course under a provider
3. **enroll_course**: Enroll a student in a course
4. **create_cert_proof**: Create a Merkle tree proof for batch certificate issuance
5. **claim_cert**: Claim a certificate using Merkle proof verification

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (v1.89.0)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v2.3.0+)
- [Anchor](https://www.anchor-lang.com/docs/installation) (v0.32.1)
- [Bun](https://bun.sh/) (package manager)
- [Node.js](https://nodejs.org/) (v18+)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd apec-certificate-program
```

2. Install dependencies:
```bash
bun install
```

3. Build the program:
```bash
anchor build
```

4. Generate TypeScript client:
```bash
bunx codama
```

## Configuration

The program is configured via `Anchor.toml`:

```toml
[programs.localnet]
apec_cert = "CAxe8JydEaRrtF3DVdPATw9XwAgYZUnCJ4wr5ZbvUFMp"

[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"
```

## Usage

### Running Tests

Run the complete test suite:
```bash
anchor test
```

Or run tests with Bun:
```bash
bun run test
```

### Deploying

1. Start local validator:
```bash
solana-test-validator
```

2. Deploy the program:
```bash
anchor deploy
```

### Example Workflow

#### 1. Initialize a Provider

```typescript
const tx = await program.methods
  .initProvider(new BN(1), "APEC")
  .accountsPartial({
    authority: authority.publicKey,
  })
  .rpc();
```

#### 2. Create a Course

```typescript
const tx = await program.methods
  .createCourse(new BN(1), "Blockchain 101")
  .accountsPartial({
    authority: authority.publicKey,
    provider: providerPda,
  })
  .rpc();
```

#### 3. Enroll in a Course

```typescript
const tx = await program.methods
  .enrollCourse(
    "Blockchain 101 Certificate",
    "BLK101",
    "https://example.com/certificates/blockchain101.json"
  )
  .accountsPartial({
    user: user.publicKey,
    course: coursePda,
    tokenProgram: TOKEN_2022_PROGRAM_ID,
  })
  .rpc();
```

#### 4. Create Certificate Proof (Merkle Tree)

```typescript
// Create list of eligible students
const certList = [student1.publicKey, student2.publicKey];

// Generate Merkle tree
const leaves = certList.map(addr => keccak_256(addr.toBuffer()));
const tree = new MerkleTree(leaves, keccak_256, { sortPairs: true });
const root = tree.getRoot();

// Create proof on-chain
const tx = await program.methods
  .createCertProof(root, new BN(certList.length))
  .accountsPartial({
    provider: providerPda,
    course: coursePda,
    authority: authority.publicKey,
  })
  .rpc();
```

#### 5. Claim Certificate

```typescript
// Generate Merkle proof for specific student
const leaf = keccak_256(student.publicKey.toBuffer());
const proof = tree.getProof(leaf).map(p => p.data);

// Claim certificate
const tx = await program.methods
  .claimCert(
    proof,
    "Blockchain 101 Certificate",
    "https://example.com/certificates/student123.json"
  )
  .accountsPartial({
    payer: payer.publicKey,
    claimer: student.publicKey,
    provider: providerPda,
    course: coursePda,
    certProof: certProofPda,
    tokenProgram: TOKEN_2022_PROGRAM_ID,
  })
  .rpc();
```

## Account Structure

### Provider
- `id`: Unique provider identifier
- `authority`: Provider's authority public key
- `short_name`: Provider's short name (max 10 chars)
- `bump`: PDA bump seed

### Course
- `id`: Unique course identifier
- `provider`: Associated provider public key
- `short_name`: Course short name (max 20 chars)
- `bump`: PDA bump seed

### CertProof
- `provider`: Associated provider public key
- `course`: Associated course public key
- `root`: Merkle tree root hash
- `total`: Total number of certificates
- `claimed`: Number of claimed certificates
- `bump`: PDA bump seed

## Security Features

- **PDA-based Access Control**: All accounts use Program Derived Addresses for security
- **Authority Verification**: Provider authority is verified for sensitive operations
- **Merkle Proof Verification**: Ensures only eligible students can claim certificates
- **Soulbound Tokens**: Mint authority is removed after certificate issuance, preventing transfers
- **Token-2022 Extensions**: Utilizes close authority and metadata pointer extensions

## Development

### Code Formatting

Format code:
```bash
bun run lint:fix
```

Check formatting:
```bash
bun run lint
```

### Project Structure

- `programs/`: Solana program source code (Rust)
- `tests/`: Integration tests (TypeScript)
- `clients/js/`: Auto-generated TypeScript client
- `migrations/`: Deployment scripts
- `target/`: Build artifacts and IDL

## Technologies Used

- **Solana**: Blockchain platform
- **Anchor**: Solana development framework
- **Token-2022**: Next-generation token standard
- **MerkleTree.js**: Merkle tree implementation
- **TypeScript**: Client-side development
- **Bun**: Fast JavaScript runtime and package manager

## Program ID

```
CAxe8JydEaRrtF3DVdPATw9XwAgYZUnCJ4wr5ZbvUFMp
```

## License

ISC

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

For issues and questions, please open an issue in the repository.

