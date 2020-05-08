use crate::error::*;
use crate::handles::*;
use crate::version::Version;
use crate::CryptoCtx;

impl CryptoCtx {
    pub fn key_manager_open(&self, _options: Option<Handle>) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn key_manager_close(&self, _key_manager_handle: Handle) -> Result<(), CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn key_manager_invalidate(
        &self,
        _key_manager_handle: Handle,
        _key_id: &[u8],
        _key_version: Version,
    ) -> Result<(), CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }
}
