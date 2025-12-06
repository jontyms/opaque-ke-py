use opaque_ke::argon2::Argon2;
use opaque_ke::ciphersuite::CipherSuite;
use opaque_ke::{
    ClientLogin, ClientLoginFinishParameters, ClientRegistration,
    ClientRegistrationFinishParameters, CredentialFinalization, CredentialRequest,
    CredentialResponse, RegistrationRequest, RegistrationUpload, ServerLogin,
    ServerLoginParameters, ServerRegistration, ServerSetup,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use rand::rngs::OsRng;
use sha2::Sha512;

// Default cipher suite using Ristretto255 and Sha512
struct DefaultCipherSuite;

impl CipherSuite for DefaultCipherSuite {
    type OprfCs = opaque_ke::Ristretto255;
    type KeyExchange = opaque_ke::TripleDh<opaque_ke::Ristretto255, Sha512>;
    type Ksf = Argon2<'static>;
}

/// Server setup containing the server's keypair
#[pyclass]
struct ServerSetupData {
    inner: ServerSetup<DefaultCipherSuite>,
}

#[pymethods]
impl ServerSetupData {
    /// Get the server's public key as bytes
    fn get_public_key(&self, py: Python) -> PyResult<Py<PyBytes>> {
        let pubkey_bytes = self.inner.serialize();
        Ok(PyBytes::new(py, &pubkey_bytes).into())
    }

    /// Serialize the server setup to bytes
    fn to_bytes(&self, py: Python) -> PyResult<Py<PyBytes>> {
        let bytes = self.inner.serialize();
        Ok(PyBytes::new(py, &bytes).into())
    }

    /// Deserialize server setup from bytes
    #[staticmethod]
    fn from_bytes(data: &[u8]) -> PyResult<Self> {
        let inner = ServerSetup::<DefaultCipherSuite>::deserialize(data)
            .map_err(|e| PyValueError::new_err(format!("Deserialization failed: {:?}", e)))?;
        Ok(Self { inner })
    }
}

/// Client registration start result
#[pyclass]
struct ClientRegistrationStartData {
    message: Vec<u8>,
    state: Vec<u8>,
}

#[pymethods]
impl ClientRegistrationStartData {
    /// Get the registration message to send to server
    fn get_message(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.message).into())
    }

    /// Get the client state (keep private)
    fn get_state(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.state).into())
    }
}

/// Server registration start result
#[pyclass]
struct ServerRegistrationStartData {
    message: Vec<u8>,
}

#[pymethods]
impl ServerRegistrationStartData {
    /// Get the registration response to send to client
    fn get_message(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.message).into())
    }
}

/// Client registration finish result
#[pyclass]
struct ClientRegistrationFinishData {
    message: Vec<u8>,
    export_key: Vec<u8>,
}

#[pymethods]
impl ClientRegistrationFinishData {
    /// Get the final registration message to send to server
    fn get_message(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.message).into())
    }

    /// Get the export key (can be used for additional key derivation)
    fn get_export_key(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.export_key).into())
    }
}

/// Server registration finish result containing the password file
#[pyclass]
struct ServerRegistrationFinishData {
    password_file: Vec<u8>,
}

#[pymethods]
impl ServerRegistrationFinishData {
    /// Get the password file (store this securely on server)
    fn get_password_file(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.password_file).into())
    }
}

/// Client login start result
#[pyclass]
struct ClientLoginStartData {
    message: Vec<u8>,
    state: Vec<u8>,
}

#[pymethods]
impl ClientLoginStartData {
    /// Get the login message to send to server
    fn get_message(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.message).into())
    }

    /// Get the client state (keep private)
    fn get_state(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.state).into())
    }
}

/// Server login start result
#[pyclass]
struct ServerLoginStartData {
    message: Vec<u8>,
    state: Vec<u8>,
}

#[pymethods]
impl ServerLoginStartData {
    /// Get the login response to send to client
    fn get_message(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.message).into())
    }

    /// Get the server state (keep private)
    fn get_state(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.state).into())
    }
}

/// Client login finish result containing session key
#[pyclass]
struct ClientLoginFinishData {
    message: Vec<u8>,
    session_key: Vec<u8>,
    export_key: Vec<u8>,
}

#[pymethods]
impl ClientLoginFinishData {
    /// Get the final login message to send to server
    fn get_message(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.message).into())
    }

    /// Get the session key (use this for encrypting communications)
    fn get_session_key(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.session_key).into())
    }

    /// Get the export key
    fn get_export_key(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.export_key).into())
    }
}

/// Server login finish result containing session key
#[pyclass]
struct ServerLoginFinishData {
    session_key: Vec<u8>,
}

#[pymethods]
impl ServerLoginFinishData {
    /// Get the session key (use this for encrypting communications)
    fn get_session_key(&self, py: Python) -> PyResult<Py<PyBytes>> {
        Ok(PyBytes::new(py, &self.session_key).into())
    }
}

/// Generate server setup (run once per server)
#[pyfunction]
fn server_setup() -> PyResult<ServerSetupData> {
    let mut rng = OsRng;
    let setup = ServerSetup::<DefaultCipherSuite>::new(&mut rng);
    Ok(ServerSetupData { inner: setup })
}

/// Client: Start registration
#[pyfunction]
fn client_registration_start(password: &[u8]) -> PyResult<ClientRegistrationStartData> {
    let mut rng = OsRng;
    let result = ClientRegistration::<DefaultCipherSuite>::start(&mut rng, password)
        .map_err(|e| PyValueError::new_err(format!("Registration start failed: {:?}", e)))?;

    Ok(ClientRegistrationStartData {
        message: result.message.serialize().to_vec(),
        state: result.state.serialize().to_vec(),
    })
}

/// Server: Start registration
#[pyfunction]
fn server_registration_start(
    server_setup: &ServerSetupData,
    registration_request: &[u8],
    username: &[u8],
) -> PyResult<ServerRegistrationStartData> {
    let request = RegistrationRequest::deserialize(registration_request)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize request: {:?}", e)))?;

    let result =
        ServerRegistration::<DefaultCipherSuite>::start(&server_setup.inner, request, username)
            .map_err(|e| {
                PyValueError::new_err(format!("Server registration start failed: {:?}", e))
            })?;

    Ok(ServerRegistrationStartData {
        message: result.message.serialize().to_vec(),
    })
}

/// Client: Finish registration
#[pyfunction]
fn client_registration_finish(
    password: &[u8],
    client_state: &[u8],
    registration_response: &[u8],
) -> PyResult<ClientRegistrationFinishData> {
    let mut rng = OsRng;
    let state = ClientRegistration::<DefaultCipherSuite>::deserialize(client_state)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize state: {:?}", e)))?;

    let response = opaque_ke::RegistrationResponse::deserialize(registration_response)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize response: {:?}", e)))?;

    let result = state
        .finish(
            &mut rng,
            password,
            response,
            ClientRegistrationFinishParameters::default(),
        )
        .map_err(|e| {
            PyValueError::new_err(format!("Client registration finish failed: {:?}", e))
        })?;

    Ok(ClientRegistrationFinishData {
        message: result.message.serialize().to_vec(),
        export_key: result.export_key.to_vec(),
    })
}

/// Server: Finish registration
#[pyfunction]
fn server_registration_finish(
    registration_upload: &[u8],
) -> PyResult<ServerRegistrationFinishData> {
    let upload = RegistrationUpload::<DefaultCipherSuite>::deserialize(registration_upload)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize upload: {:?}", e)))?;

    let password_file = ServerRegistration::<DefaultCipherSuite>::finish(upload);

    Ok(ServerRegistrationFinishData {
        password_file: password_file.serialize().to_vec(),
    })
}

/// Client: Start login
#[pyfunction]
fn client_login_start(password: &[u8]) -> PyResult<ClientLoginStartData> {
    let mut rng = OsRng;
    let result = ClientLogin::<DefaultCipherSuite>::start(&mut rng, password)
        .map_err(|e| PyValueError::new_err(format!("Client login start failed: {:?}", e)))?;

    Ok(ClientLoginStartData {
        message: result.message.serialize().to_vec(),
        state: result.state.serialize().to_vec(),
    })
}

/// Server: Start login
#[pyfunction]
fn server_login_start(
    server_setup: &ServerSetupData,
    password_file: &[u8],
    credential_request: &[u8],
    username: &[u8],
) -> PyResult<ServerLoginStartData> {
    let mut rng = OsRng;
    let password_file = ServerRegistration::<DefaultCipherSuite>::deserialize(password_file)
        .map_err(|e| {
            PyValueError::new_err(format!("Failed to deserialize password file: {:?}", e))
        })?;

    let request = CredentialRequest::deserialize(credential_request)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize request: {:?}", e)))?;

    let result = ServerLogin::start(
        &mut rng,
        &server_setup.inner,
        Some(password_file),
        request,
        username,
        ServerLoginParameters::default(),
    )
    .map_err(|e| PyValueError::new_err(format!("Server login start failed: {:?}", e)))?;

    Ok(ServerLoginStartData {
        message: result.message.serialize().to_vec(),
        state: result.state.serialize().to_vec(),
    })
}

/// Client: Finish login
#[pyfunction]
fn client_login_finish(
    password: &[u8],
    client_state: &[u8],
    credential_response: &[u8],
) -> PyResult<ClientLoginFinishData> {
    let mut rng = OsRng;
    let state = ClientLogin::<DefaultCipherSuite>::deserialize(client_state)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize state: {:?}", e)))?;

    let response = CredentialResponse::deserialize(credential_response)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize response: {:?}", e)))?;

    let result = state
        .finish(
            &mut rng,
            password,
            response,
            ClientLoginFinishParameters::default(),
        )
        .map_err(|e| PyValueError::new_err(format!("Client login finish failed: {:?}", e)))?;

    Ok(ClientLoginFinishData {
        message: result.message.serialize().to_vec(),
        session_key: result.session_key.to_vec(),
        export_key: result.export_key.to_vec(),
    })
}

/// Server: Finish login
#[pyfunction]
fn server_login_finish(
    server_state: &[u8],
    credential_finalization: &[u8],
) -> PyResult<ServerLoginFinishData> {
    let state = ServerLogin::<DefaultCipherSuite>::deserialize(server_state)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize state: {:?}", e)))?;

    let finalization =
        CredentialFinalization::deserialize(credential_finalization).map_err(|e| {
            PyValueError::new_err(format!("Failed to deserialize finalization: {:?}", e))
        })?;

    let result = state
        .finish(finalization, ServerLoginParameters::default())
        .map_err(|e| PyValueError::new_err(format!("Server login finish failed: {:?}", e)))?;

    Ok(ServerLoginFinishData {
        session_key: result.session_key.to_vec(),
    })
}

#[pymodule]
fn opaque_ke_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ServerSetupData>()?;
    m.add_class::<ClientRegistrationStartData>()?;
    m.add_class::<ServerRegistrationStartData>()?;
    m.add_class::<ClientRegistrationFinishData>()?;
    m.add_class::<ServerRegistrationFinishData>()?;
    m.add_class::<ClientLoginStartData>()?;
    m.add_class::<ServerLoginStartData>()?;
    m.add_class::<ClientLoginFinishData>()?;
    m.add_class::<ServerLoginFinishData>()?;

    m.add_function(wrap_pyfunction!(server_setup, m)?)?;
    m.add_function(wrap_pyfunction!(client_registration_start, m)?)?;
    m.add_function(wrap_pyfunction!(server_registration_start, m)?)?;
    m.add_function(wrap_pyfunction!(client_registration_finish, m)?)?;
    m.add_function(wrap_pyfunction!(server_registration_finish, m)?)?;
    m.add_function(wrap_pyfunction!(client_login_start, m)?)?;
    m.add_function(wrap_pyfunction!(server_login_start, m)?)?;
    m.add_function(wrap_pyfunction!(client_login_finish, m)?)?;
    m.add_function(wrap_pyfunction!(server_login_finish, m)?)?;

    Ok(())
}
