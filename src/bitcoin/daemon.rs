use std::{process::Command, thread::sleep, time::Duration};

use serde_json::Value;

struct BlockchainInfo {
    pub headers: u64,
    pub blocks: u64,
}

pub struct BitcoinDaemon<'a> {
    path: &'a str,
}

impl<'a> BitcoinDaemon<'a> {
    pub fn new(bitcoin_dir_path: &'a str) -> Self {
        Self {
            path: bitcoin_dir_path,
        }
    }

    pub fn start(&self) {
        sleep(Duration::from_secs(1));

        println!("Starting node...");

        // bitcoind -datadir=/Users/k/Developer/bitcoin -blocksonly -txindex=1 -v2transport -daemon
        let _ = Command::new("bitcoind")
            .arg(self.datadir())
            .arg("-blocksonly")
            .arg("-txindex=1")
            .arg("-v2transport")
            .arg("-daemon")
            .output()
            .expect("bitcoind to be able to properly start");

        sleep(Duration::from_secs(15));

        println!("Node started successfully !");
    }

    pub fn stop(&self) {
        // bitcoin-cli -datadir=/Users/k/Developer/bitcoin stop
        let status = Command::new("bitcoin-cli")
            .arg(self.datadir())
            .arg("stop")
            .output()
            .unwrap()
            .status;

        if status.success() {
            println!("Stopping node...");
            sleep(Duration::from_secs(15));
            println!("bitcoind stopped successfully !");
        }
    }

    pub fn wait_sync(&self) -> color_eyre::Result<()> {
        while !self.check_if_fully_synced()? {
            sleep(Duration::from_secs(5))
        }

        Ok(())
    }

    pub fn wait_for_new_block(&self, last_block_height: usize) -> color_eyre::Result<()> {
        println!("Waiting for new block...");

        while self.get_blockchain_info()?.headers as usize == last_block_height {
            sleep(Duration::from_secs(5))
        }

        Ok(())
    }

    pub fn check_if_fully_synced(&self) -> color_eyre::Result<bool> {
        let BlockchainInfo { blocks, headers } = self.get_blockchain_info()?;

        let synced = blocks == headers;

        if synced {
            println!("Synced ! ({blocks} blocks)");
        } else {
            println!("Syncing... ({} remaining)", headers - blocks)
        }

        Ok(synced)
    }

    fn get_blockchain_info(&self) -> color_eyre::Result<BlockchainInfo> {
        // bitcoin-cli -datadir=/Users/k/Developer/bitcoin getblockchaininfo
        let output = Command::new("bitcoin-cli")
            .arg(self.datadir())
            .arg("getblockchaininfo")
            .output()
            .unwrap();

        let json: Value = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?;
        let json = json.as_object().unwrap();

        let blocks = json.get("blocks").unwrap().as_u64().unwrap();
        let headers = json.get("headers").unwrap().as_u64().unwrap();

        Ok(BlockchainInfo { headers, blocks })
    }

    fn datadir(&self) -> String {
        format!("-datadir={}", self.path)
    }
}
