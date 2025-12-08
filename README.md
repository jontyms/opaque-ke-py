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

You'll need Rust and Python 3.12+ installed.

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

### ⚠️ CRITICAL: Python Memory Limitations

**Python bytes are immutable and cannot be reliably zeroized from memory.** This is a fundamental limitation of Python's memory management model.

**What this means:**
- Passwords passed to this library may persist in Python's heap memory until garbage collection
- They may be written to swap files or appear in core dumps
- Memory scraping malware could potentially extract passwords from process memory


**When it's appropriate:**
- ✅ Standard web applications with reasonable security requirements
- ✅ Internal tools with trusted environments
- ✅ Prototyping and development
- ✅ Applications where TLS + standard practices provide sufficient security

### General Security Best Practices

1. **Transport Security**: ALWAYS use TLS 1.3+ to prevent man-in-the-middle attacks
2. **Rate Limiting**: Implement rate limiting to prevent online guessing attacks (see below)
3. **Server Setup Storage**: Encrypt server setup before storing (contains private key)
4. **Password File Storage**: Store password files securely in your database (they're safe to store)
5. **State Management**: Keep client/server states in memory only, never log or persist them
6. **Session Key Comparison**: Use `constant_time_compare()` for comparing sensitive values
7. **Username Binding**: Usernames are cryptographically bound to registrations

### Rate Limiting (REQUIRED)

OPAQUE protects against offline dictionary attacks, but you **MUST** implement rate limiting to prevent online guessing:

```python
from time import time
from collections import defaultdict

# Simple rate limiting example
failed_attempts = defaultdict(list)  # username -> [timestamps]

def check_rate_limit(username: str, max_attempts: int = 5, window_seconds: int = 900):
    """Allow max_attempts failures per window_seconds (default: 5 per 15 min)"""
    now = time()
    # Remove old attempts outside the window
    failed_attempts[username] = [
        ts for ts in failed_attempts[username]
        if now - ts < window_seconds
    ]

    if len(failed_attempts[username]) >= max_attempts:
        raise Exception(f"Rate limit exceeded. Try again later.")

def record_failed_login(username: str):
    """Record a failed login attempt"""
    failed_attempts[username].append(time())

# Use before login
check_rate_limit(username)
try:
    # ... perform login ...
    pass
except ValueError:
    record_failed_login(username)
    raise
```

**Recommended mitigations:**
- **Rate limiting**: 5 failed attempts per 15 minutes per username
- **IP-based limits**: 20 failed attempts per hour per IP address
- **Progressive delays**: Exponential backoff after repeated failures
- **Account lockout**: Temporary lockout after 10 failures in 1 hour
- **Monitoring**: Alert on suspicious patterns (distributed attacks, enumeration attempts)

### Server Setup Key Storage

The server setup contains the server's **private key**. You MUST encrypt it before storage:

```python
from cryptography.fernet import Fernet
import os

# Generate encryption key (store in environment variable or KMS)
storage_key = os.environ.get('SERVER_SETUP_ENCRYPTION_KEY').encode()
cipher = Fernet(storage_key)

# Encrypt before storage
server_setup = opaque_ke_py.server_setup()
plaintext = server_setup.to_bytes()
encrypted = cipher.encrypt(plaintext)

# Store encrypted bytes to disk/database
with open('server_setup.enc', 'wb') as f:
    f.write(encrypted)

# Later: decrypt when loading
with open('server_setup.enc', 'rb') as f:
    encrypted = f.read()
plaintext = cipher.decrypt(encrypted)
server_setup = opaque_ke_py.ServerSetupData.from_bytes(plaintext)
```

**⚠️ NEVER:**
- Log the server setup or its private key
- Store server setup in plaintext
- Transmit server setup over the network
- Include server setup in backups without encryption

### State Security

States returned by login/registration functions contain sensitive cryptographic material:

**You MUST:**
- ✅ Keep states in memory only (never persist to disk/database)
- ✅ Never reuse states across different login/registration attempts
- ✅ Never log or serialize states
- ✅ Discard states immediately after the protocol completes

**States are single-use only.** Reusing a state violates the protocol's security guarantees.

### Constant-Time Comparisons

When comparing session keys or other sensitive values, use the provided constant-time comparison:

```python
import opaque_ke_py

# ❌ WRONG: Timing attack vulnerable
if client_session_key == server_session_key:
    print("Keys match!")

# ✅ CORRECT: Constant-time comparison
if opaque_ke_py.constant_time_compare(client_session_key, server_session_key):
    print("Keys match!")
```

### Error Handling

This library uses generic error messages to prevent information leakage:
- `"Authentication failed"` - Login/registration cryptographic operation failed
- `"Invalid message format"` - Deserialization or format error
- `"Registration failed"` - Registration protocol error
- `"input too large"` - Input exceeds 1 MB limit

**Never expose these errors directly to end users.** Instead, show user-friendly messages like:
- "Invalid username or password"
- "Registration failed, please try again"
- "An error occurred, please contact support"

### Audit Status

- ✅ **Underlying Rust library (`opaque-ke` v4.0.1)**: Audited by NCC Group for WhatsApp (2021)
- ⚠️ **This Python wrapper**: Not independently audited

The underlying cryptographic implementation is solid and has been professionally reviewed. The Python binding layer has been designed following security best practices but has not undergone formal security audit.

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

- [OPAQUE RFC 9807](https://datatracker.ietf.org/doc/rfc9807/)
- [OPAQUE Paper](https://eprint.iacr.org/2018/163)
- [opaque-ke Rust crate](https://crates.io/crates/opaque-ke)
