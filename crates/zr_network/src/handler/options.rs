use flate2::Compression;

const DEFAULT_BITS: i32 = 1024;

#[derive(Debug, Default)]
pub struct CompressionOption {
    compression_enabled: bool,
    compression: Option<Compression>,
    threshold: Option<usize>,
}

#[derive(Debug)]
pub struct CipherOption {
    shared_key: Option<Vec<u8>>,
    offline_mode: bool,
    bits: Option<i32>,
}

#[derive(Debug, Default)]
pub struct HandlerOptions {
    cipher_option: CipherOption,
    compression_option: CompressionOption,
}

impl Default for CipherOption {
    fn default() -> Self {
        Self {
            shared_key: None,
            offline_mode: true,
            bits: None,
        }
    }
}

impl CompressionOption {
    pub fn enable_compression(&mut self) {
        self.compression_enabled = self.compression.is_some();
    }

    pub fn compression(&self) -> Option<Compression> {
        self.compression
    }

    pub fn threshold(&self) -> Option<usize> {
        self.threshold
    }
}

impl HandlerOptions {
    /// enable the compression if is set
    pub fn enable_compression(&mut self) {
        println!("Compression enable");
        self.compression_option.enable_compression();
    }

    pub fn set_offline(&mut self, offline: bool) -> &mut Self {
        self.cipher_option.offline_mode = offline;
        self
    }

    /// return true if crack account are allowed
    pub fn is_offline(&self) -> bool {
        dbg!(self.cipher_option.offline_mode)
    }

    /// return true if the compression is `enabled`
    pub fn is_compression_enabled(&self) -> bool {
        self.compression_option.compression_enabled
    }

    /// return true if the compression is `set`
    pub fn is_compression_set(&self) -> bool {
        self.compression_option.compression.is_some()
    }

    /// set the type of compression or None to disable it
    pub fn set_compression(&mut self, compression: Option<Compression>) -> &mut Self {
        self.compression_option.compression = compression;
        self
    }

    pub fn set_compression_threshold(&mut self, threshold: Option<usize>) -> &mut Self {
        self.compression_option.threshold = threshold;
        self
    }

    pub fn threshold(&self) -> Option<usize> {
        self.compression_option.threshold()
    }

    pub fn compression(&self) -> Option<Compression> {
        self.compression_option.compression
    }

    /// set key bit size for RSA
    pub fn set_cipher_key_bits(&mut self, bits: Option<i32>) -> &mut Self {
        self.cipher_option.bits = if self.cipher_option.offline_mode {
            None
        } else {
            bits.or(Some(DEFAULT_BITS))
        };
        self
    }

    pub fn shared_key(&self) -> &Option<Vec<u8>> {
        &self.cipher_option.shared_key
    }

    pub fn set_shared_key(&mut self, shared_key: Option<Vec<u8>>) {
        self.cipher_option.shared_key = shared_key;
    }
}
