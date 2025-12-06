"""
Python wrapper for the OPAQUE-KE protocol.

OPAQUE is an asymmetric password-authenticated key exchange (aPAKE) protocol
that provides strong security guarantees without requiring the server to store
passwords in plaintext or even hashed form.
"""

class ServerSetupData:
    """Server setup containing the server's keypair."""

    def get_public_key(self) -> bytes:
        """Get the server's public key as bytes."""
        ...

    def to_bytes(self) -> bytes:
        """Serialize the server setup to bytes."""
        ...

    @staticmethod
    def from_bytes(data: bytes) -> ServerSetupData:
        """Deserialize server setup from bytes."""
        ...

class ClientRegistrationStartData:
    """Client registration start result."""

    def get_message(self) -> bytes:
        """Get the registration message to send to server."""
        ...

    def get_state(self) -> bytes:
        """Get the client state (keep private)."""
        ...

class ServerRegistrationStartData:
    """Server registration start result."""

    def get_message(self) -> bytes:
        """Get the registration response to send to client."""
        ...

class ClientRegistrationFinishData:
    """Client registration finish result."""

    def get_message(self) -> bytes:
        """Get the final registration message to send to server."""
        ...

    def get_export_key(self) -> bytes:
        """Get the export key (can be used for additional key derivation)."""
        ...

class ServerRegistrationFinishData:
    """Server registration finish result containing the password file."""

    def get_password_file(self) -> bytes:
        """Get the password file (store this securely on server)."""
        ...

class ClientLoginStartData:
    """Client login start result."""

    def get_message(self) -> bytes:
        """Get the login message to send to server."""
        ...

    def get_state(self) -> bytes:
        """Get the client state (keep private)."""
        ...

class ServerLoginStartData:
    """Server login start result."""

    def get_message(self) -> bytes:
        """Get the login response to send to client."""
        ...

    def get_state(self) -> bytes:
        """Get the server state (keep private)."""
        ...

class ClientLoginFinishData:
    """Client login finish result containing session key."""

    def get_message(self) -> bytes:
        """Get the final login message to send to server."""
        ...

    def get_session_key(self) -> bytes:
        """Get the session key (use this for encrypting communications)."""
        ...

    def get_export_key(self) -> bytes:
        """Get the export key."""
        ...

class ServerLoginFinishData:
    """Server login finish result containing session key."""

    def get_session_key(self) -> bytes:
        """Get the session key (use this for encrypting communications)."""
        ...

def server_setup() -> ServerSetupData:
    """
    Generate server setup (run once per server).

    Returns:
        ServerSetupData: The server setup containing keypair information.
    """
    ...

def client_registration_start(password: bytes) -> ClientRegistrationStartData:
    """
    Client: Start registration.

    Args:
        password: The password to register (as bytes).

    Returns:
        ClientRegistrationStartData: Contains message to send to server and client state.
    """
    ...

def server_registration_start(
    server_setup: ServerSetupData, registration_request: bytes, username: bytes
) -> ServerRegistrationStartData:
    """
    Server: Start registration.

    Args:
        server_setup: The server's setup data.
        registration_request: The registration request from client.
        username: The username being registered (as bytes).

    Returns:
        ServerRegistrationStartData: Contains message to send back to client.
    """
    ...

def client_registration_finish(
    password: bytes, client_state: bytes, registration_response: bytes
) -> ClientRegistrationFinishData:
    """
    Client: Finish registration.

    Args:
        password: The password being registered (as bytes).
        client_state: The client state from registration_start.
        registration_response: The response from server.

    Returns:
        ClientRegistrationFinishData: Contains final message and export key.
    """
    ...

def server_registration_finish(
    registration_upload: bytes,
) -> ServerRegistrationFinishData:
    """
    Server: Finish registration.

    Args:
        registration_upload: The final registration message from client.

    Returns:
        ServerRegistrationFinishData: Contains password file to store.
    """
    ...

def client_login_start(password: bytes) -> ClientLoginStartData:
    """
    Client: Start login.

    Args:
        password: The password to login with (as bytes).

    Returns:
        ClientLoginStartData: Contains message to send to server and client state.
    """
    ...

def server_login_start(
    server_setup: ServerSetupData,
    password_file: bytes,
    credential_request: bytes,
    username: bytes,
) -> ServerLoginStartData:
    """
    Server: Start login.

    Args:
        server_setup: The server's setup data.
        password_file: The stored password file from registration.
        credential_request: The credential request from client.
        username: The username attempting to login (as bytes).

    Returns:
        ServerLoginStartData: Contains message to send to client and server state.
    """
    ...

def client_login_finish(
    password: bytes, client_state: bytes, credential_response: bytes
) -> ClientLoginFinishData:
    """
    Client: Finish login.

    Args:
        password: The password being used to login (as bytes).
        client_state: The client state from login_start.
        credential_response: The response from server.

    Returns:
        ClientLoginFinishData: Contains final message, session key, and export key.
    """
    ...

def server_login_finish(
    server_state: bytes, credential_finalization: bytes
) -> ServerLoginFinishData:
    """
    Server: Finish login.

    Args:
        server_state: The server state from login_start.
        credential_finalization: The final credential message from client.

    Returns:
        ServerLoginFinishData: Contains session key.
    """
    ...
