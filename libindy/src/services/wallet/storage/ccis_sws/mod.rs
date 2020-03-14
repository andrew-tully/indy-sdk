use super::{WalletStorageType};

use std::sync::Arc;
use services::wallet::storage::{WalletStorage, StorageRecord, Tag, TagName, StorageIterator};
use errors::IndyError;
use grpcio::{ChannelBuilder, EnvBuilder, Environment, Channel};
use grpcio::Error::*;

use services::wallet::wallet::EncryptedValue;
use services::wallet::{language, SearchOptions, RecordOptions};

use errors::prelude::*;

mod secure_wallet_service;
mod secure_wallet_service_grpc;
use services::wallet::storage::ccis_sws::secure_wallet_service::*;
use services::wallet::storage::ccis_sws::secure_wallet_service_grpc::SecureWalletClient;
use std::error::Error;

lazy_static! {
    static ref SWS_CLIENT: SecureWalletClient = {
        let env: Arc<Environment> = Arc::new(EnvBuilder::new().build());
        let ch: Channel = ChannelBuilder::new(env).connect("localhost:50051");
        secure_wallet_service_grpc::SecureWalletClient::new(ch)
    };
}

struct SwsStorage {
    wallet_id : String
}

impl SwsStorage {
    pub fn new(id: &str) -> SwsStorage{
        SwsStorage{ wallet_id: id.to_string()}
    }
}

fn grpc_error_to_indy_error(e: grpcio::Error, default: IndyErrorKind) -> IndyError {
    return match e {
        RpcFailure(failure) => {
            match failure.status {
                grpcio::RpcStatusCode::Unknown => {
                    IndyError::from(default)
                },
                _ => {
                    IndyError::from_msg(IndyErrorKind::IOError, failure.details.unwrap())
                }
            }
        }
        _ => IndyError::from_msg(IndyErrorKind::IOError, format!("Unexpected error: {:?}", e.description()))
    }
}

impl WalletStorage for SwsStorage {
    #[allow(non_snake_case)]
    fn get(&self, type_: &[u8], id: &[u8], options: &str) -> Result<StorageRecord, IndyError> {
        // [Done] Initialize the GetWalletItemRequest
        let mut req: GetWalletItemRequest = secure_wallet_service::GetWalletItemRequest::new();

        // [Done] Set request params 1. walletId 2. field_type 3. id
        req.set_walletId(self.wallet_id.clone());
        req.set_field_type(type_.to_vec());
        req.set_id(id.to_vec());

        let options: RecordOptions = if options == "{}" { // FIXME:
            RecordOptions::default()
        } else {
            serde_json::from_str(options)
                .to_indy(IndyErrorKind::InvalidStructure, "RecordOptions is malformed json")?
        };

        // [Done] match call of SWS client of get_wallet_item, error handling, response. 1. walletItem
        let wallet_item: WalletItemResponse = match SWS_CLIENT.get_wallet_item(&req) {
            Err(e) => {
              return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletItemNotFound))
            },
            Ok(e) => { e.walletItem.unwrap() }
        };

        // [Done] Extract the walletItem's params. Plain text [[ 1. id 2. value 3. key 4. field_type ]] 5. encryptedTags 6. plaintextTags
        let id: &[u8] = wallet_item.get_id();
        let value = if options.retrieve_value {
            Some(EncryptedValue::new(wallet_item.get_value().to_vec(), wallet_item.get_key().to_vec()))
        } else {
            None
        };
        let type_ = if options.retrieve_type {
            Some(wallet_item.get_field_type())
        } else {
            None
        };

        let all_tags = if options.retrieve_tags {
            let mut tags: Vec<Tag> = vec![];

            let encrypted_tags = wallet_item.get_encryptedTags();
            if !encrypted_tags.is_empty() {
                for encrypted_tag in encrypted_tags {
                    match encrypted_tag {
                        EncryptedTagResponse{name, encryptedValue, unknown_fields: _, cached_size: _ } => {
                            let tag: Tag = Tag::Encrypted(name.to_vec(), encryptedValue.to_vec());
                            tags.push(tag);
                        }
                    };
                }
            }

            let plaintext_tags = wallet_item.get_plaintextTags();
            if !plaintext_tags.is_empty() {
                for plaintext_tag in plaintext_tags {
                    match plaintext_tag {
                        PlaintextTagResponse{name, plaintextValue, unknown_fields: _, cached_size: _ } => {
                            let tag: Tag = Tag::PlainText(name.to_vec(), plaintextValue.to_string());
                            tags.push(tag);
                        }
                    };
                }
            }
            Some(tags)
        } else {
            None
        };

        // [Done] Fill an initialized Storage Record params. 1. id: Vu8
        // 2. value: Option<EncryptedValue> a) data: Vu8 b) key: Vu8 3. type_: Option<Vu8>
        // 4. tags: V<Tag> i) Encrypted(vu8,vu8) or ii) PlainText(vu8, String)
        Ok(StorageRecord::new(id.to_vec(), value, type_.map(|val| val.to_vec()), all_tags))
    }

    fn add(&self, type_: &[u8], id: &[u8], value: &EncryptedValue, tags: &[Tag]) -> Result<(), IndyError> {
        // [Done] Initialize the AddWalletItemRequest
        let mut req: AddWalletItemRequest = secure_wallet_service::AddWalletItemRequest::new();

        // [Done] Set the request params 1. walletId: String 2. field_type: vu8 3. id: vu8 4. value: vu8
            // 5. key: vu8 6. encryptedTags: <name: vu8, encryptedValue: vu8>
            // 7. plaintextTags <name: vu8, plaintextValue: String>
        req.set_walletId(self.wallet_id.clone());
        req.set_field_type(type_.to_vec());
        req.set_id(id.to_vec());
        req.set_value(value.data.to_vec());
        req.set_key(value.key.to_vec());

        // [Done] Setting the encryptedTags and plaintextTags from the list of tags
        let mut encrypted_tags: Vec<EncryptedTagResponse> = vec![];
        let mut plaintext_tags: Vec<PlaintextTagResponse> = vec![];
        if !tags.is_empty() {
            for tag in tags {
                match tag {
                    &Tag::Encrypted(ref tag_name, ref tag_data) => {
                        let mut encrypted_tag: EncryptedTagResponse = secure_wallet_service::EncryptedTagResponse::new();
                        encrypted_tag.set_name(tag_name.to_vec());
                        encrypted_tag.set_encryptedValue(tag_data.to_vec());
                        encrypted_tags.push(encrypted_tag.clone());
                    },

                    &Tag::PlainText(ref tag_name, ref tag_data) => {
                        let mut plaintext_tag: PlaintextTagResponse = secure_wallet_service::PlaintextTagResponse::new();
                        plaintext_tag.set_name(tag_name.to_vec());
                        plaintext_tag.set_plaintextValue(tag_data.to_string());
                        plaintext_tags.push(plaintext_tag.clone());
                    }
                };
            }
        }
        req.set_encryptedTags(::protobuf::RepeatedField::from_vec(encrypted_tags));
        req.set_plaintextTags(::protobuf::RepeatedField::from_vec(plaintext_tags));

        // [Done] Match call of SWS client of add_wallet_item, error handling, response -> message: String
        match SWS_CLIENT.add_wallet_item(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletItemAlreadyExists))
            },
            Ok(e) => { e.message }
        };

        Ok(())
    }

    fn update(&self, type_: &[u8], id: &[u8], value: &EncryptedValue) -> Result<(), IndyError> {
        // Initialize the UpdateWalletItemRequest
        let mut req: UpdateWalletItemRequest = secure_wallet_service::UpdateWalletItemRequest::new();

        // Set request params 1. walletId: String 2. field_type: vu8 3. id: vu8 4. value: vu8
            // 5. key: vu8
        req.set_walletId(self.wallet_id.clone());
        req.set_field_type(type_.to_vec());
        req.set_id(id.to_vec());
        req.set_value(value.data.to_vec());
        req.set_key(value.key.to_vec());

        // Match call of SWS client of update_wallet_item, error handling, response. 1. message: String
        match SWS_CLIENT.update_wallet_item(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletItemNotFound))
            },
            Ok(e) => { e.message }
        };

        Ok(())
    }

    fn add_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), IndyError> {
        // Initialize the AddWalletItemTagsRequest
        let mut req: AddWalletItemTagsRequest = secure_wallet_service::AddWalletItemTagsRequest::new();

        // Set request params 1. walletId: String 2. field_type: vu8 3. id: vu8
            // 4. encryptedTags: <name: vu8, encryptedValue: vu8>
            // 5. plaintextTags <name: vu8, plaintextValue: String>
        req.set_walletId(self.wallet_id.clone());
        req.set_field_type(type_.to_vec());
        req.set_id(id.to_vec());

        // [Done] Setting the encryptedTags and plaintextTags from the list of tags
        let mut encrypted_tags: Vec<EncryptedTagResponse> = vec![];
        let mut plaintext_tags: Vec<PlaintextTagResponse> = vec![];
        if !tags.is_empty() {
            for tag in tags {
                match tag {
                    &Tag::Encrypted(ref tag_name, ref tag_data) => {
                        let mut encrypted_tag: EncryptedTagResponse = secure_wallet_service::EncryptedTagResponse::new();
                        encrypted_tag.set_name(tag_name.to_vec());
                        encrypted_tag.set_encryptedValue(tag_data.to_vec());
                        encrypted_tags.push(encrypted_tag.clone());
                    },

                    &Tag::PlainText(ref tag_name, ref tag_data) => {
                        let mut plaintext_tag: PlaintextTagResponse = secure_wallet_service::PlaintextTagResponse::new();
                        plaintext_tag.set_name(tag_name.to_vec());
                        plaintext_tag.set_plaintextValue(tag_data.to_string());
                        plaintext_tags.push(plaintext_tag.clone());
                    }

                };
            }
        }
        req.set_encryptedTags(::protobuf::RepeatedField::from_vec(encrypted_tags));
        req.set_plaintextTags(::protobuf::RepeatedField::from_vec(plaintext_tags));

        // Match call of SWS client of add_wallet_item_tags, error handling, response 1. message: String
        match SWS_CLIENT.add_wallet_item_tags(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletItemNotFound))
            },
            Ok(e) => { e.message }
        };

        Ok(())
    }

    fn update_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), IndyError> {
        // Initialize the UpdateWalletItemTagsRequest
        let mut req: UpdateWalletItemTagsRequest = secure_wallet_service::UpdateWalletItemTagsRequest::new();

        // Set request params 1. walletId: String 2. field_type: vu8 3. id: vu8
            // 4. encryptedTags: <name: vu8, encryptedValue: vu8>
            // 5. plaintextTags <name: vu8, plaintextValue: String>
        req.set_walletId(self.wallet_id.clone());
        req.set_field_type(type_.to_vec());
        req.set_id(id.to_vec());

        // [Done] Setting the encryptedTags and plaintextTags from the list of tags
        let mut encrypted_tags: Vec<EncryptedTagResponse> = vec![];
        let mut plaintext_tags: Vec<PlaintextTagResponse> = vec![];
        if !tags.is_empty() {
            for tag in tags {
                match tag {
                    &Tag::Encrypted(ref tag_name, ref tag_data) => {
                        let mut encrypted_tag: EncryptedTagResponse = secure_wallet_service::EncryptedTagResponse::new();
                        encrypted_tag.set_name(tag_name.to_vec());
                        encrypted_tag.set_encryptedValue(tag_data.to_vec());
                        encrypted_tags.push(encrypted_tag.clone());
                    },

                    &Tag::PlainText(ref tag_name, ref tag_data) => {
                        let mut plaintext_tag: PlaintextTagResponse = secure_wallet_service::PlaintextTagResponse::new();
                        plaintext_tag.set_name(tag_name.to_vec());
                        plaintext_tag.set_plaintextValue(tag_data.to_string());
                        plaintext_tags.push(plaintext_tag.clone());
                    }

                };
            }
        }
        req.set_encryptedTags(::protobuf::RepeatedField::from_vec(encrypted_tags));
        req.set_plaintextTags(::protobuf::RepeatedField::from_vec(plaintext_tags));

        // Match call of SWS client of update_wallet_item_tags, error handling, response 1. message: String
        match SWS_CLIENT.update_wallet_item_tags(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletItemNotFound))
            },
            Ok(e) => { e.message }
        };

        Ok(())
    }
    fn delete_tags(&self, type_: &[u8], id: &[u8], tag_names: &[TagName]) -> Result<(), IndyError> {
        // Initialize the DeleteWalletItemTagsRequest
        let mut req: DeleteWalletItemTagsRequest = secure_wallet_service::DeleteWalletItemTagsRequest::new();

        // Set request params 1. walletId: String 2. field_type: vu8 3. id: vu8
            // 4. tagNames: DeleteWalletItemTagsRequest_TagName
                // <tagType: DeleteWalletItemTagsRequest_TagType <enum E0/P1>, name: vu8>
        req.set_walletId(self.wallet_id.clone());
        req.set_field_type(type_.to_vec());
        req.set_id(id.to_vec());

        let mut tag_names_list: Vec<DeleteWalletItemTagsRequest_TagName> = vec![];
        for tag_name in tag_names {
            match tag_name {
                &TagName::OfEncrypted(ref tag_name) => {
                    let mut encrypted_tag_name: DeleteWalletItemTagsRequest_TagName = secure_wallet_service::DeleteWalletItemTagsRequest_TagName::new();
                    encrypted_tag_name.set_tagType(secure_wallet_service::DeleteWalletItemTagsRequest_TagType::ENCRYPTED);
                    encrypted_tag_name.set_name(tag_name.to_vec());
                    tag_names_list.push(encrypted_tag_name.clone());
                },
                &TagName::OfPlain(ref tag_name) => {
                    let mut plaintext_tag_name: DeleteWalletItemTagsRequest_TagName = secure_wallet_service::DeleteWalletItemTagsRequest_TagName::new();
                    plaintext_tag_name.set_tagType(secure_wallet_service::DeleteWalletItemTagsRequest_TagType::PLAINTEXT);
                    plaintext_tag_name.set_name(tag_name.to_vec());
                    tag_names_list.push(plaintext_tag_name.clone());
                }
            };
        }
        req.set_tagNames(::protobuf::RepeatedField::from_vec(tag_names_list));

        // Match call of SWS client of delete_wallet_item_tags, error handling, response 1. message: String
        match SWS_CLIENT.delete_wallet_item_tags(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletItemNotFound))
            },
            Ok(e) => { e.message }
        };

        Ok(())
    }

    fn delete(&self, type_: &[u8], id: &[u8]) -> Result<(), IndyError> {
        // Initialize the DeleteWalletRequest
        let mut req: DeleteWalletItemRequest = secure_wallet_service::DeleteWalletItemRequest::new();

        // Set request params 1. walletId: String 2. field_type: vu8 3. id: vu8
        req.set_walletId(self.wallet_id.clone());
        req.set_field_type(type_.to_vec());
        req.set_id(id.to_vec());

        // Match call of SWS client of delete_wallet, error handling, response 1. message: String
        match SWS_CLIENT.delete_wallet_item(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletItemNotFound))
            },
            Ok(e) => { e.message }
        };

        // TODO: Missing delete the encrypted and plaintext tags related to credential

        Ok(())
    }
    fn get_storage_metadata(&self) -> Result<Vec<u8>, IndyError> {
        // Initialize the GetWalletMetadataRequest
        let mut req: GetWalletMetadataRequest = secure_wallet_service::GetWalletMetadataRequest::new();

        // Set request params 1. walletId: String
        req.set_walletId(self.wallet_id.clone());

        // Match call of SWS client of get_wallet_metadata, error handling, response 1. metadata: vu8
        let metadata: Vec<u8> = match SWS_CLIENT.get_wallet_metadata(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletNotFound))
            },
            Ok(e) => { e.metadata }
        };

        Ok(metadata)
    }
    fn set_storage_metadata(&self, metadata: &[u8]) -> Result<(), IndyError> {
        // Initialize the SetWalletMetadataRequest
        let mut req: SetWalletMetadataRequest = secure_wallet_service::SetWalletMetadataRequest::new();

        // Set request params 1. walletId: String 2. metadata: vu8
        req.set_walletId(self.wallet_id.clone());
        req.set_metadata(metadata.to_vec());

        // Match call of SWS client of set_wallet_metadata, error handling, response 1. message: String
        match SWS_CLIENT.set_wallet_metadata(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletNotFound))
            },
            Ok(e) => { e.message }
        };

        Ok(())
    }
    fn get_all(&self) -> Result<Box<StorageIterator>, IndyError> {
        // Initialize the GetAllWalletItemsRequest
        let mut req: GetAllWalletItemsRequest = secure_wallet_service::GetAllWalletItemsRequest::new();

        // Set request params 1. walletId: String
        req.set_walletId(self.wallet_id.clone());

        let fetch_options = RecordOptions {
            retrieve_type: true,
            retrieve_value: true,
            retrieve_tags: true,
        };


        // Match call of SWS client of get_all_wallet_items, error handling, response 1. walletItems: Repeated<WalletItemResponse>
        let wallet_items_list: Vec<WalletItemResponse> = match SWS_CLIENT.get_all_wallet_items(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletNotFound))
            },
            Ok(e) => { e.walletItems.into_vec() }
        };

        // Figure out what to do with wallet items and Storage Iterator

        Ok(Box::new(SwsStorageIterator::new(wallet_items_list.to_vec(), wallet_items_list.len(), fetch_options)))
    }
    fn search(&self, type_: &[u8], query: &language::Operator, options: Option<&str>) -> Result<Box<StorageIterator>, IndyError> {
        // Initialize the SearchWalletItemsRequest
        let mut req: SearchWalletItemsRequest = secure_wallet_service::SearchWalletItemsRequest::new();

        // Set request params 1. walletId: String 2. field_type: vu8 3. query: String 4. options: String
        req.set_walletId(self.wallet_id.clone());
        req.set_field_type(type_.to_vec());
        req.set_query(query.to_string());

        let search_options: SearchOptions = match options {
            None => SearchOptions::default(),
            Some(option_str) => serde_json::from_str(option_str)
                .to_indy(IndyErrorKind::InvalidStructure, "Search options is malformed json")?
        };

        let fetch_options = if search_options.retrieve_records {
            RecordOptions {
                retrieve_value: search_options.retrieve_value,
                retrieve_tags: search_options.retrieve_tags,
                retrieve_type: search_options.retrieve_type,
            }
        } else {
            RecordOptions::default()
        };

        let wallet_items_list: Vec<WalletItemResponse> = match SWS_CLIENT.search_wallet_items(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletNotFound))
            },
            Ok(e) => { e.walletItems.into_vec() }
        };

        Ok(Box::new(SwsStorageIterator::new(wallet_items_list.to_vec(), wallet_items_list.len(), fetch_options)))
    }
    fn close(&mut self) -> Result<(), IndyError> {
        // No call made to SWS
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
    fn create_storage(&self, id: &str, _config: Option<&str>, _credentials: Option<&str>, metadata: &[u8]) -> Result<(), IndyError>{
        // Set CreateWalletRequest for the gRPC call to CCIS SWS
        let mut req: CreateWalletRequest = secure_wallet_service::CreateWalletRequest::new();
        req.set_walletId(id.to_string());
        req.set_metadata(metadata.to_vec());

        // Call the CreateWallet function with the request
        match SWS_CLIENT.create_wallet(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletAlreadyExists))
            },
            Ok(resp) => {}
        };

        Ok(())
    }

    fn open_storage(&self, id: &str, _config: Option<&str>, _credentials: Option<&str>) -> Result<Box<WalletStorage>, IndyError>{
        // No call made to CCIS SWS,
        // TODO: can consider checking if wallet exists, or to whatever makes the program work
        Ok(Box::new(SwsStorage::new(id)))
    }

    fn delete_storage(&self, id: &str, _config: Option<&str>, _credentials: Option<&str>) -> Result<(), IndyError> {
        // Set DeleteWalletRequest for the gRPC call to CCIS SWS
        let mut req: DeleteWalletRequest = secure_wallet_service::DeleteWalletRequest::new();
        req.set_walletId(id.to_string());

        // Call the DeleteWallet function with the request
        match SWS_CLIENT.delete_wallet(&req) {
            Err(e) => {
                return Err(grpc_error_to_indy_error(e, IndyErrorKind::WalletNotFound))
            },
            Ok(e) => { e }
        };

        Ok(())
    }
}

struct SwsStorageIterator {
    credentials: Vec<WalletItemResponse>,
    options: RecordOptions,
    cursor_index: usize,
    total_count: usize,
}

impl SwsStorageIterator {
    pub fn new(credentials: Vec<WalletItemResponse>, total_count: usize, options: RecordOptions) -> SwsStorageIterator {
        SwsStorageIterator {
            credentials,
            options,
            cursor_index: 0,
            total_count
        }
    }
}

impl StorageIterator for SwsStorageIterator {
    fn next(&mut self) -> Result<Option<StorageRecord>, IndyError> {
        // If no credentials in the list
        if self.cursor_index >= self.total_count {
            return Ok(None);
        }

        // Grab the next WalletItemResponse according to the cursor_index
        let wallet_item: WalletItemResponse = self.credentials.get(self.cursor_index).unwrap().clone();

        // Convert WalletItemResponse into StorageRecord
        // [Done] Extract the walletItem's params. Plain text [[ 1. id 2. value 3. key 4. field_type ]] 5. encryptedTags 6. plaintextTags
        let id: &[u8] = wallet_item.get_id();

        let value = if self.options.retrieve_value {
            Some(EncryptedValue::new(wallet_item.get_value().to_vec(), wallet_item.get_key().to_vec()))
        } else {
            None
        };

        let type_ = if self.options.retrieve_type {
            Some(wallet_item.get_field_type())
        } else {
            None
        };

        let all_tags = if self.options.retrieve_tags {
            let mut tags: Vec<Tag> = vec![];

            let encrypted_tags = wallet_item.get_encryptedTags();
            if !encrypted_tags.is_empty() {
                for encrypted_tag in encrypted_tags {
                    match encrypted_tag {
                        EncryptedTagResponse{name, encryptedValue, unknown_fields: _, cached_size: _ } => {
                            let tag: Tag = Tag::Encrypted(name.to_vec(), encryptedValue.to_vec());
                            tags.push(tag);
                        }
                    };
                }
            }

            let plaintext_tags = wallet_item.get_plaintextTags();
            if !plaintext_tags.is_empty() {
                for plaintext_tag in plaintext_tags {
                    match plaintext_tag {
                        PlaintextTagResponse{name, plaintextValue, unknown_fields: _, cached_size: _ } => {
                            let tag: Tag = Tag::PlainText(name.to_vec(), plaintextValue.to_string());
                            tags.push(tag);
                        }
                    };
                }
            }
            Some(tags)
        } else {
            None
        };

        // Increment the cursor_index
        self.cursor_index = self.cursor_index + 1;

        // [Done] Fill an initialized Storage Record params. 1. id: Vu8
        // 2. value: Option<EncryptedValue> a) data: Vu8 b) key: Vu8 3. type_: Option<Vu8>
        // 4. tags: V<Tag> i) Encrypted(vu8,vu8) or ii) PlainText(vu8, String)
        Ok(Some(StorageRecord::new(id.to_vec(), value, type_.map(|val| val.to_vec()), all_tags)))
    }
    fn get_total_count(&self) -> Result<Option<usize>, IndyError> {
        Ok(Some(self.total_count))
    }
}

#[cfg(test)]
mod ccis_sws_tests {
    use utils::test;

    use super::*;
    use super::super::Tag;


    #[test]
    // TODO: These tests below to be deleted/moved as these are integration tests. Don't belong.
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
    // TODO: These tests below to be deleted/moved as these are integration tests. Don't belong.
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