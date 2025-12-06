# opaque-ke-py

Python bindings for the [OPAQUE-KE](https://github.com/facebook/opaque-ke) asymmetric password-authenticated key exchange (aPAKE) protocol.

## What is OPAQUE?

OPAQUE is a secure asymmetric password-authenticated key exchange protocol that provides strong security guarantees:

- **Server never sees the password**: The server doesn't store passwords in plaintext or even hashed form
- **Resistant to pre-computation attacks**: Even if the server is compromised, attackers cannot perform offline dictionary attacks
- **Mutual authentication**: Both client and server authenticate each other
- **Establishes a shared session key**: After successful authentication, both parties share a strong cryptographic session key

## Installation

### From source

You'll need Rust and Python 3.8+ installed.

```bash
# Install maturin (Rust/Python build tool)
pip install maturin

# Build and install
maturin build --release
pip install target/wheels/opaque_ke_py-*.whl
```

### From PyPI (when published)

```bash
pip install opaque-ke-py
```

## Quick Start

```python
import opaque_ke_py

# Server setup (done once when server initializes)
server_setup = opaque_ke_py.server_setup()

# Registration flow
username = b"alice"
password = b"correct-horse-battery-staple"

# 1. Client starts registration
client_reg_start = opaque_ke_py.client_registration_start(password)
registration_request = client_reg_start.get_message()
client_reg_state = client_reg_start.get_state()

# 2. Server processes registration request
server_reg_start = opaque_ke_py.server_registration_start(
    server_setup,
    registration_request,
    username
)
registration_response = server_reg_start.get_message()

# 3. Client finishes registration
client_reg_finish = opaque_ke_py.client_registration_finish(
    password,
    client_reg_state,
    registration_response
)
registration_upload = client_reg_finish.get_message()

# 4. Server stores password file
server_reg_finish = opaque_ke_py.server_registration_finish(registration_upload)
password_file = server_reg_finish.get_password_file()
# Store password_file securely in database

# Login flow
# 1. Client starts login
client_login_start = opaque_ke_py.client_login_start(password)
credential_request = client_login_start.get_message()
client_login_state = client_login_start.get_state()

# 2. Server processes login request
server_login_start = opaque_ke_py.server_login_start(
    server_setup,
    password_file,
    credential_request,
    username
)
credential_response = server_login_start.get_message()
server_login_state = server_login_start.get_state()

# 3. Client finishes login
client_login_finish = opaque_ke_py.client_login_finish(
    password,
    client_login_state,
    credential_response
)
credential_finalization = client_login_finish.get_message()
client_session_key = client_login_finish.get_session_key()

# 4. Server finishes login
server_login_finish = opaque_ke_py.server_login_finish(
    server_login_state,
    credential_finalization
)
server_session_key = server_login_finish.get_session_key()

# Both client and server now have matching session keys
assert client_session_key == server_session_key
```

## Complete Example

See [example.py](example.py) for a complete working example demonstrating:
- Server setup
- User registration
- User login
- Session key establishment
- Failed login handling

Run it with:
```bash
python example.py
```

## API Reference

### Server Setup

```python
server_setup() -> ServerSetupData
```

Generate server setup containing the server's keypair. This should be done once when the server starts and the result should be stored securely.

### Registration Flow

#### 1. Client Registration Start

```python
client_registration_start(password: bytes) -> ClientRegistrationStartData
```

Client initiates registration with their password.

**Returns:**
- `get_message()`: Message to send to server
- `get_state()`: Client state to keep private

#### 2. Server Registration Start

```python
server_registration_start(
    server_setup: ServerSetupData,
    registration_request: bytes,
    username: bytes
) -> ServerRegistrationStartData
```

Server processes the registration request.

**Returns:**
- `get_message()`: Message to send back to client

#### 3. Client Registration Finish

```python
client_registration_finish(
    password: bytes,
    client_state: bytes,
    registration_response: bytes
) -> ClientRegistrationFinishData
```

Client completes registration.

**Returns:**
- `get_message()`: Final message to send to server
- `get_export_key()`: Export key for additional key derivation

#### 4. Server Registration Finish

```python
server_registration_finish(
    registration_upload: bytes
) -> ServerRegistrationFinishData
```

Server completes registration and generates password file.

**Returns:**
- `get_password_file()`: Password file to store in database

### Login Flow

#### 1. Client Login Start

```python
client_login_start(password: bytes) -> ClientLoginStartData
```

Client initiates login with their password.

**Returns:**
- `get_message()`: Message to send to server
- `get_state()`: Client state to keep private

#### 2. Server Login Start

```python
server_login_start(
    server_setup: ServerSetupData,
    password_file: bytes,
    credential_request: bytes,
    username: bytes
) -> ServerLoginStartData
```

Server processes the login request.

**Returns:**
- `get_message()`: Message to send back to client
- `get_state()`: Server state to keep private

#### 3. Client Login Finish

```python
client_login_finish(
    password: bytes,
    client_state: bytes,
    credential_response: bytes
) -> ClientLoginFinishData
```

Client completes login. Raises `ValueError` if authentication fails.

**Returns:**
- `get_message()`: Final message to send to server
- `get_session_key()`: Shared session key for encrypted communication
- `get_export_key()`: Export key for additional key derivation

#### 4. Server Login Finish

```python
server_login_finish(
    server_state: bytes,
    credential_finalization: bytes
) -> ServerLoginFinishData
```

Server completes login. Raises `ValueError` if authentication fails.

**Returns:**
- `get_session_key()`: Shared session key for encrypted communication

## Security Considerations

1. **Transport Security**: OPAQUE should be used over a secure transport layer (e.g., TLS) to prevent MITM attacks
2. **Server Setup Storage**: Store the server setup securely and never expose it
3. **Password File Storage**: Store password files securely in your database
4. **State Management**: Keep client/server states private during multi-step protocols
5. **Session Keys**: Use the established session keys for encrypting subsequent communications
6. **Username Binding**: Usernames are cryptographically bound to registrations to prevent confusion attacks

## Cipher Suite

This wrapper uses the following cryptographic primitives:

- **OPRF**: Ristretto255
- **Key Exchange**: TripleDH over Ristretto255 with SHA-512
- **Key Stretching Function**: Argon2

These provide strong security guarantees and are recommended by the OPAQUE specification.

## Development

### Building

```bash
# Debug build
maturin develop

# Release build
maturin build --release
```

### Testing

```bash
# Run the example
python example.py

# Check types
mypy example.py
```

## License

MIT

## Credits

This project wraps the [opaque-ke](https://github.com/facebook/opaque-ke) Rust implementation by Meta Platforms, Inc.

## References

- [OPAQUE RFC Draft](https://datatracker.ietf.org/doc/draft-irtf-cfrg-opaque/)
- [OPAQUE Paper](https://eprint.iacr.org/2018/163)
- [opaque-ke Rust crate](https://crates.io/crates/opaque-ke)
