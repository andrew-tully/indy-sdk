#![cfg_attr(feature = "fatal_warnings", deny(warnings))]

extern crate base64;

#[macro_use]
extern crate log;

extern crate serde;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

// Note that to use macroses from indy_common::util inside of other modules it must me loaded first!
extern crate libc;
extern crate time;
extern crate rand;
extern crate grpcio;
extern crate protobuf;
extern crate futures;

pub mod libindy;

// Note that to use macroses from util inside of other modules it must me loaded first!
#[macro_use]
pub mod utils;
pub mod errors;
pub mod sws_storage;
pub mod wql;
pub mod grpc;

use libindy::ErrorCode;
use utils::sequence::SequenceUtils;
use utils::crypto::base64 as util_base64;
use utils::ctypes;
use wql::storage::{WalletStorage, StorageRecord, StorageIterator, Tag, TagName, EncryptedValue};
use wql::language;
use errors::wallet::WalletStorageError;
use sws_storage::{WalletStorageType, SwsStorageType};

use self::libc::c_char;

use std::collections::HashMap;
use std::ffi::CString;
use std::sync::Mutex;
use std::str;

pub static SWS_STORAGE_NAME: &str = "sws_storage";


#[no_mangle]
pub extern fn sws_storage_init() -> libindy::ErrorCode {
    debug!("Initializing sws storage plugin");
    // if let Err(err) = utils::logger::PostgressStorageLogger::init() {
    //     return err;
    // }

    let sws_storage_name = CString::new(SWS_STORAGE_NAME).unwrap();

    libindy::wallet::register_wallet_storage(
        sws_storage_name.as_ptr(),
        SwsWallet::create,
        SwsWallet::open,
        SwsWallet::close,
        SwsWallet::delete,
        SwsWallet::add_record,
        SwsWallet::update_record_value,
        SwsWallet::update_record_tags,
        SwsWallet::add_record_tags,
        SwsWallet::delete_record_tags,
        SwsWallet::delete_record,
        SwsWallet::get_record,
        SwsWallet::get_record_id,
        SwsWallet::get_record_type,
        SwsWallet::get_record_value,
        SwsWallet::get_record_tags,
        SwsWallet::free_record,
        SwsWallet::get_storage_metadata,
        SwsWallet::set_storage_metadata,
        SwsWallet::free_storage_metadata,
        SwsWallet::search_records,
        SwsWallet::search_all_records,
        SwsWallet::get_search_total_count,
        SwsWallet::fetch_search_next_record,
        SwsWallet::free_search,
    )
}

// #[no_mangle]
// pub extern fn init_storagetype(config: *const c_char, credentials: *const c_char) -> libindy::ErrorCode {
//     return SwsWallet::init(config, credentials);
// }

struct SwsStorageContext {
    _xhandle: i32,        // reference returned to client to track open wallet connection
    id: String,          // wallet name
    _config: String,      // wallet config
    _credentials: String, // wallet credentials
    sws_handle: Box<::sws_storage::SwsStorage>  // reference to a SWS storage obj
}

#[derive(Debug, Clone)]
struct SwsWalletRecord {
    id: CString,
    type_: CString,
    value: Vec<u8>,
    tags: CString
}

#[derive(Debug, Clone)]
struct SwsWalletRecordSet {
    idx: usize,
    records: Vec<SwsWalletRecord>,
    count: usize
}

lazy_static! {
    // store handle -> id / storage obj
    static ref SWS_OPEN_WALLETS: Mutex<HashMap<i32, SwsStorageContext>> = Default::default();
}

lazy_static! {
    // metadata for active wallets
    static ref SWS_ACTIVE_METADATAS: Mutex<HashMap<i32, CString>> = Default::default();
}

lazy_static! {
    // cache of Postgres fetched records
    static ref SWS_ACTIVE_RECORDS: Mutex<HashMap<i32, SwsWalletRecord>> = Default::default();
}

lazy_static! {
    // cache of active Postgres searches
    // TODO figure out a thread-safe iterator
    // static ref SWS_ACTIVE_SEARCHES: Mutex<HashMap<i32, Box<::postgres_storage::PostgresStorageIterator>>> = Default::default();
    static ref SWS_ACTIVE_SEARCHES: Mutex<HashMap<i32, SwsWalletRecordSet>> = Default::default();
}

pub struct SwsWallet{}

impl SwsWallet {

    // pub extern fn init(config: *const c_char, credentials: *const c_char) -> ErrorCode {
    //     check_useful_c_str!(config, ErrorCode::CommonInvalidState);
    //     check_useful_c_str!(credentials, ErrorCode::CommonInvalidState);
    //
    //     // create Postgres database, and create schema
    //     let storage_type = ::sws_storage::SwsStorageType::new();
    //     let res = storage_type.init_storage(Some(&config), Some(&credentials));
    //
    //     match res {
    //         Ok(_) => ErrorCode::Success,
    //         Err(err) => {
    //             match err {
    //                 WalletStorageError::AlreadyExists => ErrorCode::WalletAlreadyExistsError,
    //                 _ => {
    //                     error!("Init storage failed: {:?}", err);
    //                     ErrorCode::WalletStorageError
    //                 }
    //             }
    //         }
    //     }
    // }

    pub extern fn create(id: *const c_char,
                             config: *const c_char,
                             credentials: *const c_char,
                             metadata: *const c_char) -> ErrorCode {
        check_useful_c_str!(id, ErrorCode::CommonInvalidParam1);
        check_useful_c_str!(config, ErrorCode::CommonInvalidParam2);
        check_useful_c_str!(credentials, ErrorCode::CommonInvalidParam3);
        check_useful_c_str!(metadata, ErrorCode::CommonInvalidParam4);

        let storage_type = ::sws_storage::SwsStorageType::new();
        let res = storage_type.create_storage(&id, Some(&config), Some(&credentials), &metadata.as_bytes()[..]);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::AlreadyExists => ErrorCode::WalletAlreadyExistsError,
                    _ => {
                        error!("Create storage failed: {:?}", err);
                        ErrorCode::WalletStorageError
                    }
                }
            }
        }
    }


    pub extern fn open(id: *const c_char,
                           config: *const c_char,
                           credentials: *const c_char,
                           handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(id, ErrorCode::CommonInvalidParam1);
        check_useful_c_str!(config, ErrorCode::CommonInvalidParam2);
        check_useful_c_str!(credentials, ErrorCode::CommonInvalidParam3);

        // open wallet and return handle
        // PostgresStorageType::open_storage(), returns a PostgresStorage that goes into the handle
        let mut handles = SWS_OPEN_WALLETS.lock().unwrap();

        // check if we have opened this wallet already
        for (_key, value) in &*handles {
            if value.id == id {
                return ErrorCode::WalletAlreadyOpenedError;
            }
        }

        // open the wallet
        let storage_type = ::sws_storage::SwsStorageType::new();
        let sws_handle = match storage_type.open_storage(&id, Some(&config), Some(&credentials))  {
            Ok(sws_handle) => sws_handle,
            Err(_err) => {
                return ErrorCode::WalletNotFoundError;
            }
        };

        // get a handle (to use to identify wallet for subsequent calls)
        let xhandle = SequenceUtils::get_next_id();

        // create a storage context (keep all info in case we need to recycle wallet connection)
        let context = SwsStorageContext {
            _xhandle: xhandle,      // reference returned to client to track open wallet connection
            id,           // wallet name
            _config: config,       // wallet config
            _credentials: credentials,  // wallet credentials
            sws_handle // reference to sws storage
        };

        // add to our open wallet list
        handles.insert(xhandle, context);

        // return handle = index into our collection of open wallets
        unsafe { *handle = xhandle };
        ErrorCode::Success
    }


    pub extern fn add_record(xhandle: i32,
                                 type_: *const c_char,
                                 id: *const c_char,
                                 value: *const u8,
                                 value_len: usize,
                                 tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_byte_array!(value, value_len, ErrorCode::CommonInvalidState, ErrorCode::CommonInvalidState);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidState);

        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let value = EncryptedValue::from_bytes(&value).unwrap();
        let tags = _tags_from_json(&tags_json).unwrap();

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.sws_handle;
        let storage = &*wallet_box;

        let res = storage.add(&type_.as_bytes(), &id.as_bytes(), &value, &tags);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
                    _ => {
                        error!("Error adding a record. Error details: {:?}", err);
                        ErrorCode::WalletStorageError
                    }
                }
            }
        }
    }


    pub extern fn update_record_value(xhandle: i32,
                                          type_: *const c_char,
                                          id: *const c_char,
                                          joined_value: *const u8,
                                          joined_value_len: usize) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_byte_array!(joined_value, joined_value_len, ErrorCode::CommonInvalidState, ErrorCode::CommonInvalidState);

        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let value = EncryptedValue::from_bytes(&joined_value).unwrap();

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.sws_handle;
        let storage = &*wallet_box;

        let res = storage.update(&type_.as_bytes(), &id.as_bytes(), &value);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemNotFound => ErrorCode::WalletItemNotFound,
                    _ => {
                        error!("Error updating a record. Error details: {:?}", err);
                        ErrorCode::WalletStorageError
                    }
                }
            }
        }
    }


    pub extern fn get_record(xhandle: i32,
                                 type_: *const c_char,
                                 id: *const c_char,
                                 options_json: *const c_char,
                                 handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(options_json, ErrorCode::CommonInvalidState);

        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.sws_handle;
        let storage = &*wallet_box;

        let res = storage.get(&type_.as_bytes(), &id.as_bytes(), &options_json);

        match res {
            Ok(record) => {
                let record_handle = SequenceUtils::get_next_id();
                let p_rec = _storagerecord_to_swsrecord(&record).unwrap();

                let mut handles = SWS_ACTIVE_RECORDS.lock().unwrap();
                handles.insert(record_handle, p_rec);

                unsafe { *handle = record_handle };
                ErrorCode::Success
            },
            Err(err) => {
                match err {
                    WalletStorageError::ItemNotFound => ErrorCode::WalletItemNotFound,
                    _ => {
                        error!("Error getting a record. Error details: {:?}", err);
                        ErrorCode::WalletStorageError
                    }
                }
            }
        }
    }


    pub extern fn get_record_id(xhandle: i32,
                                    record_handle: i32,
                                    id_ptr: *mut *const c_char) -> ErrorCode {
        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = SWS_ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *id_ptr = record.id.as_ptr() as *const i8; }

        ErrorCode::Success
    }


    pub extern fn get_record_type(xhandle: i32,
                                      record_handle: i32,
                                      type_ptr: *mut *const c_char) -> ErrorCode {
        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = SWS_ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *type_ptr = record.type_.as_ptr() as *const i8; }

        ErrorCode::Success
    }


    pub extern fn get_record_value(xhandle: i32,
                                       record_handle: i32,
                                       value_ptr: *mut *const u8,
                                       value_len: *mut usize) -> ErrorCode {
        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = SWS_ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *value_ptr = record.value.as_ptr() as *const u8; }
        unsafe { *value_len = record.value.len() as usize; }

        ErrorCode::Success
    }


    pub extern fn get_record_tags(xhandle: i32,
                                      record_handle: i32,
                                      tags_json_ptr: *mut *const c_char) -> ErrorCode {
        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = SWS_ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *tags_json_ptr = record.tags.as_ptr() as *const i8; }

        ErrorCode::Success
    }



    pub extern fn free_record(xhandle: i32, record_handle: i32) -> ErrorCode {
        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut handles = SWS_ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }
        handles.remove(&record_handle);

        ErrorCode::Success
    }


    pub extern fn add_record_tags(xhandle: i32,
                                      type_: *const c_char,
                                      id: *const c_char,
                                      tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidState);

        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let tags = _tags_from_json(&tags_json).unwrap();

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.sws_handle;
        let storage = &*wallet_box;

        let res = storage.add_tags(&type_.as_bytes(), &id.as_bytes(), &tags);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
                    _ => {
                        error!("Error adding record tags. Error details: {:?}", err);
                        ErrorCode::WalletStorageError
                    }
                }
            }
        }
    }


    pub extern fn update_record_tags(xhandle: i32,
                                         type_: *const c_char,
                                         id: *const c_char,
                                         tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidState);

        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let tags = _tags_from_json(&tags_json).unwrap();

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.sws_handle;
        let storage = &*wallet_box;

        let res = storage.update_tags(&type_.as_bytes(), &id.as_bytes(), &tags);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
                    _ => {
                        error!("Error updating record tags. Error details: {:?}", err);
                        ErrorCode::WalletStorageError
                    }
                }
            }
        }
    }


    pub extern fn delete_record_tags(xhandle: i32,
                                         type_: *const c_char,
                                         id: *const c_char,
                                         tag_names_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(tag_names_json, ErrorCode::CommonInvalidState);

        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        // convert to [TagName]
        let tag_names = _tag_names_from_json(&tag_names_json).unwrap();

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.sws_handle;
        let storage = &*wallet_box;

        let res = storage.delete_tags(&type_.as_bytes(), &id.as_bytes(), &tag_names);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
                    _ => {
                        error!("Error deleting record tags. Error details: {:?}", err);
                        ErrorCode::WalletStorageError
                    }
                }
            }
        }
    }


    pub extern fn delete_record(xhandle: i32,
                                    type_: *const c_char,
                                    id: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);

        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.sws_handle;
        let storage = &*wallet_box;

        let res = storage.delete(&type_.as_bytes(), &id.as_bytes());

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemNotFound => ErrorCode::WalletItemNotFound,
                    _ => {
                        error!("Error deleting record. Error details: {:?}", err);
                        ErrorCode::WalletStorageError
                    }
                }
            }
        }
    }


    pub extern fn get_storage_metadata(xhandle: i32, metadata_ptr: *mut *const c_char, metadata_handle: *mut i32) -> ErrorCode {
        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.sws_handle;
        let storage = &*wallet_box;

        let res = storage.get_storage_metadata();

        match res {
            Ok(metadata) => {
                let metadata = CString::new(metadata.clone()).unwrap();
                let metadata_pointer = metadata.as_ptr();

                let handle = SequenceUtils::get_next_id();

                let mut metadatas = SWS_ACTIVE_METADATAS.lock().unwrap();
                metadatas.insert(handle, metadata);

                unsafe { *metadata_ptr = metadata_pointer; }
                unsafe { *metadata_handle = handle };

                ErrorCode::Success
            },
            Err(err) => {
                error!("Error getting storage metadata. Error details: {:?}", err);
                ErrorCode::CommonInvalidState
            }
        }
    }


    pub extern fn set_storage_metadata(xhandle: i32, metadata: *const c_char) -> ErrorCode {
        check_useful_c_str!(metadata, ErrorCode::CommonInvalidState);

        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.sws_handle;
        let storage = &*wallet_box;

        let res = storage.set_storage_metadata(&metadata.as_bytes());

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
                    _ => {
                        error!("Error setting storage metadata. Error details: {:?}", err);
                        ErrorCode::WalletStorageError
                    }
                }
            }
        }
    }


    pub extern fn free_storage_metadata(xhandle: i32, metadata_handler: i32) -> ErrorCode {
        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut handles = SWS_ACTIVE_METADATAS.lock().unwrap();

        if !handles.contains_key(&metadata_handler) {
            return ErrorCode::CommonInvalidState;
        }
        handles.remove(&metadata_handler);

        ErrorCode::Success
    }


    pub extern fn search_records(xhandle: i32, type_: *const c_char, query_json: *const c_char, options_json: *const c_char, handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(query_json, ErrorCode::CommonInvalidState);
        check_useful_c_str!(options_json, ErrorCode::CommonInvalidState);

        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let query = language::parse_from_json_encrypted(&query_json).unwrap();
        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.sws_handle;
        let storage = &*wallet_box;

        let res = storage.search(&type_.as_bytes(), &query, Some(&options_json));

        match res {
            Ok(iter) => {
                // iter: Box<StorageIterator>
                let total_count = iter.get_total_count();
                let search_records = _iterator_to_record_set(iter).unwrap();
                let total_count = match total_count {
                    Ok(count) => {
                        match count {
                            Some(ct) => ct,
                            None => 0
                        }
                    },
                    _ => search_records.len()
                };
                let search_set = SwsWalletRecordSet {
                    idx: 0,
                    records: search_records,
                    count: total_count
                };

                let search_handle = SequenceUtils::get_next_id();

                let mut searches = SWS_ACTIVE_SEARCHES.lock().unwrap();

                // TODO store the iterator rather than the fetched records
                // searches.insert(search_handle, iter);
                searches.insert(search_handle, search_set);

                unsafe { *handle = search_handle };
                return ErrorCode::Success
            },
            Err(_err) => {
                // err: WalletStorageError
                return ErrorCode::WalletStorageError
            }
        }
    }


    pub extern fn search_all_records(xhandle: i32, handle: *mut i32) -> ErrorCode {
        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.sws_handle;
        let storage = &*wallet_box;

        let res = storage.get_all();

        match res {
            Ok(iter) => {
                // iter: Box<StorageIterator>
                let total_count = iter.get_total_count();
                let search_records = _iterator_to_record_set(iter).unwrap();
                let total_count = match total_count {
                    Ok(count) => {
                        match count {
                            Some(ct) => ct,
                            None => 0
                        }
                    },
                    _ => search_records.len()
                };
                let search_set = SwsWalletRecordSet {
                    idx: 0,
                    records: search_records,
                    count: total_count
                };

                let search_handle = SequenceUtils::get_next_id();

                let mut searches = SWS_ACTIVE_SEARCHES.lock().unwrap();

                // TODO store the iterator rather than the fetched records
                // searches.insert(search_handle, iter);
                searches.insert(search_handle, search_set);

                unsafe { *handle = search_handle };
                return ErrorCode::Success
            },
            Err(_err) => {
                // err: WalletStorageError
                return ErrorCode::WalletStorageError
            }
        }
    }


    pub extern fn get_search_total_count(xhandle: i32, search_handle: i32, count: *mut usize) -> ErrorCode {
        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let searches = SWS_ACTIVE_SEARCHES.lock().unwrap();

        match searches.get(&search_handle) {
            Some(records) => {
                unsafe { *count = records.count };
            }
            None => return ErrorCode::CommonInvalidState
        }

        ErrorCode::Success
    }


    pub extern fn fetch_search_next_record(xhandle: i32, search_handle: i32, record_handle: *mut i32) -> ErrorCode {
        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut searches = SWS_ACTIVE_SEARCHES.lock().unwrap();

        match searches.get_mut(&search_handle) {
            Some(records) => {
                if records.idx < records.records.len() {
                    let handle = SequenceUtils::get_next_id();

                    let mut handles = SWS_ACTIVE_RECORDS.lock().unwrap();
                    handles.insert(handle, records.records.get(records.idx).unwrap().clone());
                    records.idx = records.idx + 1;

                    unsafe { *record_handle = handle };
                    ErrorCode::Success
                } else {
                    ErrorCode::WalletItemNotFound
                }
            },
            None => ErrorCode::CommonInvalidState
        }
    }


    pub extern fn free_search(xhandle: i32, search_handle: i32) -> ErrorCode {
        let handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut handles = SWS_ACTIVE_SEARCHES.lock().unwrap();

        if !handles.contains_key(&search_handle) {
            return ErrorCode::CommonInvalidState;
        }
        handles.remove(&search_handle);

        ErrorCode::Success
    }


    pub extern fn close(xhandle: i32) -> ErrorCode {
        let mut handles = SWS_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.remove(&xhandle).unwrap();

        let mut storage = *wallet_context.sws_handle;

        let res = storage.close();

        match res {
            Ok(_) => ErrorCode::Success,
            Err(_err) => ErrorCode::WalletStorageError
        }
    }


    pub extern fn delete(id: *const c_char,
                             config: *const c_char,
                             credentials: *const c_char) -> ErrorCode {
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(config, ErrorCode::CommonInvalidState);
        check_useful_c_str!(credentials, ErrorCode::CommonInvalidState);

        let storage_type = ::sws_storage::SwsStorageType::new();
        match storage_type.delete_storage(&id, Some(&config), Some(&credentials)) {
            Ok(_) => ErrorCode::Success,
            Err(_err) => ErrorCode::WalletStorageError
        }
    }
}

fn _storagerecord_to_swsrecord(in_rec: &StorageRecord) -> Result<SwsWalletRecord, WalletStorageError> {
    let out_id = CString::new(in_rec.id.clone()).unwrap();
    let out_type = match in_rec.type_ {
        Some(ref val) => CString::new(val.clone()).unwrap(),
        None => CString::new("").unwrap()
    };
    let out_val = match in_rec.value {
        Some(ref val) => val.to_bytes(),
        None => Vec::<u8>::new()
    };
    let out_tags = match in_rec.tags {
        Some(ref val) => CString::new(_tags_to_json(&val).unwrap()).unwrap(),
        None => CString::new("").unwrap()
    };
    let out_rec = SwsWalletRecord {
        id: out_id,
        type_: out_type,
        value: out_val,
        tags: out_tags
    };
    Ok(out_rec)
}

fn _iterator_to_record_set(mut iter: Box<dyn StorageIterator>) -> Result<Vec<SwsWalletRecord>, ErrorCode> {
    let mut search_continue: bool = true;
    let mut search_records = Vec::new();
    while search_continue {
        let rec = iter.next();
        match rec {
            Ok(record) => {
                match record {
                    Some(record) => {
                        // record: StorageRecord
                        search_records.push(_storagerecord_to_swsrecord(&record).unwrap());
                    },
                    None => {
                        search_continue = false;
                    }
                };
            },
            Err(_err) => {
                return Err(ErrorCode::WalletStorageError);
            }
        };
    }
    Ok(search_records)
}

fn _tags_to_json(tags: &[Tag]) -> Result<String, WalletStorageError> {
    let mut string_tags = HashMap::new();
    for tag in tags {
        match tag {
            &Tag::Encrypted(ref name, ref value) => string_tags.insert(util_base64::encode(&name), util_base64::encode(&value)),
            &Tag::PlainText(ref name, ref value) => string_tags.insert(format!("~{}", &util_base64::encode(&name)), value.to_string()),
        };
    }
    serde_json::to_string(&string_tags).map_err(|err| WalletStorageError::IOError(err.to_string()))
}

fn _tags_from_json(json: &str) -> Result<Vec<Tag>, WalletStorageError> {
    let string_tags: HashMap<String, String> = serde_json::from_str(json).map_err(|err| WalletStorageError::IOError(err.to_string()))?;
    let mut tags = Vec::new();

    for (k, v) in string_tags {
        if k.chars().next() == Some('~') {
            let mut key = k;
            key.remove(0);
            tags.push(
                Tag::PlainText(
                    util_base64::decode(&key).map_err(|err| WalletStorageError::IOError(err.to_string()))?,
                    v
                )
            );
        } else {
            tags.push(
                Tag::Encrypted(
                    util_base64::decode(&k).map_err(|err| WalletStorageError::IOError(err.to_string()))?,
                    util_base64::decode(&v).map_err(|err| WalletStorageError::IOError(err.to_string()))?
                )
            );
        }
    }
    Ok(tags)
}

fn _tag_names_to_json(tag_names: &[TagName]) -> Result<String, WalletStorageError> {
    let mut tags: Vec<String> = Vec::new();

    for tag_name in tag_names {
        tags.push(
            match tag_name {
                &TagName::OfEncrypted(ref tag_name) => util_base64::encode(tag_name),
                &TagName::OfPlain(ref tag_name) => format!("~{}", util_base64::encode(tag_name))
            }
        )
    }
    serde_json::to_string(&tags).map_err(|err| WalletStorageError::IOError(err.to_string()))
}

fn _tag_names_from_json(json: &str) -> Result<Vec<TagName>, WalletStorageError> {
    let string_tag_names: Vec<String> = serde_json::from_str(json).map_err(|err| WalletStorageError::IOError(err.to_string()))?;
    let mut tag_names = Vec::new();

    for k in string_tag_names {
        if k.chars().next() == Some('~') {
            let mut key = k;
            key.remove(0);
            tag_names.push(
                TagName::OfPlain(
                    util_base64::decode(&key).map_err(|err| WalletStorageError::IOError(err.to_string()))?
                )
            );
        } else {
            tag_names.push(
                TagName::OfEncrypted(
                    util_base64::decode(&k).map_err(|err| WalletStorageError::IOError(err.to_string()))?
                )
            );
        }
    }
    Ok(tag_names)
}

#[cfg(test)]
mod tests {}
