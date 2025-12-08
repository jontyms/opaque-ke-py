#!/usr/bin/env python3
"""
Example usage of opaque-ke-py Python wrapper.

This demonstrates the complete OPAQUE protocol flow:
1. Server setup (done once)
2. Client registration (password registration)
3. Client login (password authentication)
"""

import opaque_ke_py


def main():
    print("=== OPAQUE-KE Python Example ===\n")

    # Server setup - done once when server starts
    print("1. Server setup...")
    server_setup = opaque_ke_py.server_setup()
    print("   ✓ Server setup complete\n")

    # Registration flow
    print("2. Registration flow")
    username = b"alice"
    password = b"correct-horse-battery-staple"

    # Client starts registration
    print("   Client: Starting registration...")
    client_reg_start = opaque_ke_py.client_registration_start(password)
    registration_request = client_reg_start.get_message()
    client_reg_state = client_reg_start.get_state()
    print("   ✓ Client sent registration request")

    # Server responds to registration
    print("   Server: Processing registration...")
    server_reg_start = opaque_ke_py.server_registration_start(
        server_setup, registration_request, username
    )
    registration_response = server_reg_start.get_message()
    print("   ✓ Server sent registration response")

    # Client finishes registration
    print("   Client: Finishing registration...")
    client_reg_finish = opaque_ke_py.client_registration_finish(
        password, client_reg_state, registration_response
    )
    registration_upload = client_reg_finish.get_message()
    export_key_reg = client_reg_finish.get_export_key()
    print(
        f"   ✓ Client finished registration (export_key: {export_key_reg.hex()[:32]}...)"
    )

    # Server stores password file
    print("   Server: Storing password file...")
    server_reg_finish = opaque_ke_py.server_registration_finish(registration_upload)
    password_file = server_reg_finish.get_password_file()
    print("   ✓ Server stored password file\n")

    # Login flow
    print("3. Login flow")

    # Client starts login
    print("   Client: Starting login...")
    client_login_start = opaque_ke_py.client_login_start(password)
    credential_request = client_login_start.get_message()
    client_login_state = client_login_start.get_state()
    print("   ✓ Client sent credential request")

    # Server responds to login
    print("   Server: Processing login...")
    server_login_start = opaque_ke_py.server_login_start(
        server_setup, password_file, credential_request, username
    )
    credential_response = server_login_start.get_message()
    server_login_state = server_login_start.get_state()
    print("   ✓ Server sent credential response")

    # Client finishes login
    print("   Client: Finishing login...")
    client_login_finish = opaque_ke_py.client_login_finish(
        password, client_login_state, credential_response
    )
    credential_finalization = client_login_finish.get_message()
    client_session_key = client_login_finish.get_session_key()
    export_key_login = client_login_finish.get_export_key()
    print(
        f"   ✓ Client finished login (session_key: {client_session_key.hex()[:32]}...)"
    )
    print(f"     Export key: {export_key_login.hex()[:32]}...")

    # Server finishes login
    print("   Server: Finishing login...")
    server_login_finish = opaque_ke_py.server_login_finish(
        server_login_state, credential_finalization
    )
    server_session_key = server_login_finish.get_session_key()
    print(
        f"   ✓ Server finished login (session_key: {server_session_key.hex()[:32]}...)\n"
    )

    # Verify session keys match using constant-time comparison
    print("4. Verification")
    if opaque_ke_py.constant_time_compare(client_session_key, server_session_key):
        print("   ✓ SUCCESS! Session keys match!")
        print(f"   Session key: {client_session_key.hex()}")
    else:
        print("   ✗ FAILURE! Session keys don't match!")
        return 1

    # Test wrong password
    print("\n5. Testing login with wrong password")
    wrong_password = b"wrong-password"

    try:
        client_login_start = opaque_ke_py.client_login_start(wrong_password)
        credential_request = client_login_start.get_message()
        client_login_state = client_login_start.get_state()

        server_login_start = opaque_ke_py.server_login_start(
            server_setup, password_file, credential_request, username
        )
        credential_response = server_login_start.get_message()

        # This should fail
        client_login_finish = opaque_ke_py.client_login_finish(
            wrong_password, client_login_state, credential_response
        )
        print("   ✗ Login should have failed but didn't!")
        return 1
    except ValueError as e:
        print(f"   ✓ Login correctly failed: {e}")

    print("\n=== All tests passed! ===")
    return 0


if __name__ == "__main__":
    exit(main())
