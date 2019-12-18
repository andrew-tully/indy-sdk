use super::{WalletStorageType};

use std::sync::Arc;
use services::wallet::storage::{WalletStorage, StorageRecord, Tag, TagName, StorageIterator};
use errors::IndyError;
use grpcio::{ChannelBuilder, EnvBuilder, Environment, Channel};
use services::wallet::wallet::EncryptedValue;
use services::wallet::language;

use errors::prelude::*;

mod secure_wallet_server;
mod secure_wallet_server_grpc;
use services::wallet::storage::ccis_sws::secure_wallet_server::*;
use services::wallet::storage::ccis_sws::secure_wallet_server_grpc::SecureWalletClient;

lazy_static! {
    static ref SWS_CLIENT: SecureWalletClient = {
        SwsGrpcClient::new().client
    };
}

struct SwsGrpcClient {
    client : SecureWalletClient
}

impl SwsGrpcClient {
    pub fn new() -> SwsGrpcClient {
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect("localhost:50051");
        let sws_client = secure_wallet_server_grpc::SecureWalletClient::new(ch);
        SwsGrpcClient{ client: sws_client }
    }
}

struct SwsStorage {
    wallet_id : String
}

impl SwsStorage {
    pub fn new(id: &str) -> SwsStorage{
        SwsStorage{ wallet_id: id.to_string()}
    }
}

// TODO: Implement these later!
impl WalletStorage for SwsStorage {
    fn get(&self, type_: &[u8], id: &[u8], options: &str) -> Result<StorageRecord, IndyError> {
        Ok(StorageRecord::new(id.to_vec(), None, None, None))
    }
    fn add(&self, type_: &[u8], id: &[u8], value: &EncryptedValue, tags: &[Tag]) -> Result<(), IndyError> {
        Ok(())
    }
    fn update(&self, type_: &[u8], id: &[u8], value: &EncryptedValue) -> Result<(), IndyError> {
        Ok(())
    }
    fn add_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), IndyError> {
        Ok(())
    }
    fn update_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), IndyError> {
        Ok(())
    }
    fn delete_tags(&self, type_: &[u8], id: &[u8], tag_names: &[TagName]) -> Result<(), IndyError> {
        Ok(())
    }
    fn delete(&self, type_: &[u8], id: &[u8]) -> Result<(), IndyError> {
        Ok(())
    }
    fn get_storage_metadata(&self) -> Result<Vec<u8>, IndyError> {
        Ok(vec![1,2,3])
    }
    fn set_storage_metadata(&self, metadata: &[u8]) -> Result<(), IndyError> {
        Ok(())
    }
    fn get_all(&self) -> Result<Box<StorageIterator>, IndyError> {
        Ok(Box::new(SwsStorageIterator::new()))
    }
    fn search(&self, type_: &[u8], query: &language::Operator, options: Option<&str>) -> Result<Box<StorageIterator>, IndyError> {
        Ok(Box::new(SwsStorageIterator::new()))
    }
    fn close(&mut self) -> Result<(), IndyError> {
        Ok(())
    }
}

pub struct SwsStorageType {
}

impl SwsStorageType {
    pub fn new() -> SwsStorageType {
        SwsStorageType{}
    }
}

impl WalletStorageType for SwsStorageType {
    // TODO: Discuss about handling SK1, SK2, SK3. Current return type is None.
    fn create_storage(&self, id: &str, _config: Option<&str>, _credentials: Option<&str>, metadata: &[u8]) -> Result<(), IndyError>{
        print!("ccis-sws create_storage");

        // Set CreateWalletRequest for the gRPC call to CCIS SWS
        let mut req = secure_wallet_server::CreateWalletRequest::new();
        req.set_walletId(id.to_string());

        // Call the CreateWallet function with the request
        let mut reply = match SWS_CLIENT.create_wallet(&req) {
            Err(e) => {
                // TODO: Determine what happened to the other Errors such as IndyErrorKind::IOError
                return Err(IndyError::from(IndyErrorKind::WalletAlreadyExists))
            },
            Ok(e) => { e }
        };
        println!("Create wallet received: {:?}", reply);

        Ok(())
    }

    fn open_storage(&self, id: &str, _config: Option<&str>, _credentials: Option<&str>) -> Result<Box<WalletStorage>, IndyError>{
        print!("ccis-sws open_storage");
        // No call made to CCIS SWS
        Ok(Box::new((SwsStorage::new(id))))
    }

    fn delete_storage(&self, id: &str, _config: Option<&str>, _credentials: Option<&str>) -> Result<(), IndyError> {
        print!("ccis-sws delete_storage");

        // Set DeleteWalletRequest for the gRPC call to CCIS SWS
        let mut req = secure_wallet_server::DeleteWalletRequest::new();
        req.set_walletId(id.to_string());

        // Call the DeleteWallet function with the request
        let mut reply = match SWS_CLIENT.delete_wallet(&req) {
            Err(e) => {
                return Err(IndyError::from(IndyErrorKind::WalletNotFound))
            },
            Ok(e) => { e }
        };
        println!("Delete wallet received: {:?}", reply);

        Ok(())
    }
}

struct SwsStorageIterator {
    total_count: Option<i32>,
}

impl SwsStorageIterator {
    pub fn new() -> SwsStorageIterator {
        SwsStorageIterator {
            total_count: None
        }
    }
}

impl StorageIterator for SwsStorageIterator {
    fn next(&mut self) -> Result<Option<StorageRecord>, IndyError> {
        Ok(None)
    }
    fn get_total_count(&self) -> Result<Option<usize>, IndyError> {
        Ok(None)
    }
}

#[cfg(test)]
mod ccis_sws_tests {
    use utils::test;

    use super::*;
    use super::super::Tag;

    #[test]
    fn ccis_sws_storage_type_create_and_delete_works() {
        let storage_type = SwsStorageType::new();
        match storage_type.delete_storage(_wallet_id(), None, None) {
            Err(e) => (),
            Ok(e) => ()
        }
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();
        storage_type.delete_storage(_wallet_id(), None, None).unwrap();

        // Creating wallet with same ID twice should error on second time
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();

        let res = storage_type.create_storage(_wallet_id(), None, None, &_metadata());
        assert_kind!(IndyErrorKind::WalletAlreadyExists, res);

        storage_type.delete_storage(_wallet_id(), None, None).unwrap();

        let res = storage_type.delete_storage(_wallet_id(), None, None);
        assert_kind!(IndyErrorKind::WalletNotFound, res);
    }

    #[test]
    fn ccis_sws_storage_type_open_works() {
        let storage_type = SwsStorageType::new();
        storage_type.open_storage(_wallet_id(), None, None).unwrap();
    }

    fn _wallet_id() -> &'static str {
        "test_wallet"
    }

    fn _metadata() -> Vec<u8> {
        return vec![
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8
        ];
    }
}