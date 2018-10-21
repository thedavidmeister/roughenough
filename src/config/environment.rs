// Copyright 2017-2018 int08h LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate hex;

use std::env;
use std::time::Duration;

use config::ServerConfig;
use config::{DEFAULT_BATCH_SIZE, DEFAULT_STATUS_INTERVAL};
use key::KeyProtection;
use Error;

///
/// Obtain a Roughenough server configuration ([ServerConfig](trait.ServerConfig.html))
/// from environment variables.
///
///   Config parameter | Environment Variable
///   ---------------- | --------------------
///   port             | `ROUGHENOUGH_PORT`
///   interface        | `ROUGHENOUGH_INTERFACE`
///   seed             | `ROUGHENOUGH_SEED`
///   batch_size       | `ROUGHENOUGH_BATCH_SIZE`
///   status_interval  | `ROUGHENOUGH_STATUS_INTERVAL`
///   key_protection   | `ROUGHENOUGH_KEY_PROTECTION`
///
pub struct EnvironmentConfig {
    port: u16,
    interface: String,
    seed: Vec<u8>,
    batch_size: u8,
    status_interval: Duration,
    key_protection: KeyProtection,
}

const ROUGHENOUGH_PORT: &str = "ROUGHENOUGH_PORT";
const ROUGHENOUGH_INTERFACE: &str = "ROUGHENOUGH_INTERFACE";
const ROUGHENOUGH_SEED: &str = "ROUGHENOUGH_SEED";
const ROUGHENOUGH_BATCH_SIZE: &str = "ROUGHENOUGH_BATCH_SIZE";
const ROUGHENOUGH_STATUS_INTERVAL: &str = "ROUGHENOUGH_STATUS_INTERVAL";
const ROUGHENOUGH_KEY_PROTECTION: &str = "ROUGHENOUGH_KEY_PROTECTION";

impl EnvironmentConfig {
    pub fn new() -> Result<Self, Error> {
        let mut cfg = EnvironmentConfig {
            port: 0,
            interface: "".to_string(),
            seed: Vec::new(),
            batch_size: DEFAULT_BATCH_SIZE,
            status_interval: DEFAULT_STATUS_INTERVAL,
            key_protection: KeyProtection::Plaintext,
        };

        if let Ok(port) = env::var(ROUGHENOUGH_PORT) {
            cfg.port = port
                .parse()
                .unwrap_or_else(|_| panic!("invalid port: {}", port));
        };

        if let Ok(interface) = env::var(ROUGHENOUGH_INTERFACE) {
            cfg.interface = interface.to_string();
        };

        if let Ok(seed) = env::var(ROUGHENOUGH_SEED) {
            cfg.seed =
                hex::decode(&seed).expect("invalid seed value; 'seed' should be a hex value");
        };

        if let Ok(batch_size) = env::var(ROUGHENOUGH_BATCH_SIZE) {
            cfg.batch_size = batch_size
                .parse()
                .unwrap_or_else(|_| panic!("invalid batch_size: {}", batch_size));
        };

        if let Ok(status_interval) = env::var(ROUGHENOUGH_STATUS_INTERVAL) {
            let val: u16 = status_interval
                .parse()
                .unwrap_or_else(|_| panic!("invalid status_interval: {}", status_interval));

            cfg.status_interval = Duration::from_secs(u64::from(val));
        };

        if let Ok(key_protection) = env::var(ROUGHENOUGH_KEY_PROTECTION) {
            cfg.key_protection = key_protection
                .parse()
                .unwrap_or_else(|_| panic!("invalid key_protection value: {}", key_protection));
        }

        Ok(cfg)
    }
}

impl ServerConfig for EnvironmentConfig {
    fn interface(&self) -> &str {
        self.interface.as_ref()
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn seed(&self) -> Vec<u8> {
        self.seed.clone()
    }

    fn batch_size(&self) -> u8 {
        self.batch_size
    }

    fn status_interval(&self) -> Duration {
        self.status_interval
    }

    fn key_protection(&self) -> &KeyProtection {
        &self.key_protection
    }
}
