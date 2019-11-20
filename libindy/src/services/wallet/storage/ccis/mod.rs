use errors::prelude::*;
use services::wallet::language;
use super::{EncryptedValue, StorageIterator, StorageRecord, Tag, TagName, WalletStorage, WalletStorageType};
use base64::{encode};

extern crate mongodb;
use mongodb::{Bson, bson, Document, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::results::UpdateResult;
use mongodb::cursor::Cursor;
use mongodb::coll::Collection;
use mongodb::coll::options::IndexOptions;
use mongodb::coll::options::UpdateOptions;

use super::super::{RecordOptions};
use services::wallet::SearchOptions;

use serde_json::{Value, Error};
use std::io::ErrorKind;

#[derive(Debug)]
struct MongoStorage{
    wallet_id: String
}

/*
fn _wallet_id_from_json(json: &str) -> Result<&str, Error>{
    let v: Value = serde_json::from_str(json)?;
    v["id"].as_str().ok_or(Error::from(std::io::Error::new(ErrorKind::InvalidData, "No id")))
}
*/

#[derive(Debug)]
pub struct MongoStorageType {
}

#[derive(Debug)]
struct MongoOp{ }

static mut MONGO_CLIENT:Option<Client> = None;

impl MongoOp {
    fn wallets_collection() -> Result<Collection, IndyError> {
        MongoOp::collection("wallets")
    }

    fn items_collection() -> Result<Collection, IndyError> {
        MongoOp::collection("items")
    }

    fn tags_encrypted_collection() -> Result<Collection, IndyError> {
        MongoOp::collection("tags_encrypted")
    }

    fn tags_plaintext_collection() -> Result<Collection, IndyError> {
        MongoOp::collection("tags_plaintext")
    }

    fn metadata_collection() -> Result<Collection, IndyError> {
        MongoOp::collection("metadata")
    }

    fn collection(coll_name:&str) -> Result<Collection, IndyError> {

        unsafe {

            if MONGO_CLIENT.is_none() {
                let c = Client::connect("127.0.0.1", 27017);
                if c.is_err() {
                    error!("MongoOp::find_tag_docs - {:?}", c.unwrap());
                    return Err(IndyError::from_msg(IndyErrorKind::IOError, "Unable to connect to mongodb server"));
                };
                let mc = c.unwrap();
                MONGO_CLIENT = Some(mc);
            }

            let client = match MONGO_CLIENT {
                Some(ref c) => c,
                None => return Err(IndyError::from_msg(IndyErrorKind::IOError, "Unable to connect to mongodb server"))
            };

            return Ok(client.db("ccis_indy_wallets").collection(coll_name));
        }

    }

    fn set_index(coll:&Collection, idx:&Document) {
        let mut opts = IndexOptions::new();
        opts.unique = Some(true);
        coll.create_index(idx.clone(), Some(opts)).unwrap();
    }

    fn find_metadata_doc(wallet_id:&str) -> Result<Document, IndyError> {
        let coll = match MongoOp::metadata_collection() {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        let filter = doc! {
            "wallet_id": wallet_id,
        };

        let item = match MongoOp::find_doc(&coll, &filter) {
            Err(e) => return Err(e),
            Ok(d) => {
                match d {
                    None => {
                        return Err(IndyError::from_msg(IndyErrorKind::WalletItemNotFound, "Wallet item not found"));
                    },
                    Some(d) => d
                }
            }
        };

        Ok(item)

    }

    fn update_metadata_doc(wallet_id:&str, value:&[u8]) -> Result<Bson, IndyError> {
        let coll = match MongoOp::metadata_collection() {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        let doc = MongoOp::metadata_doc(wallet_id, value);
        MongoOp::insert_doc(&coll, &doc)
    }

    fn insert_item_doc(doc:&Document) -> Result<Bson, IndyError> {
        let coll = match MongoOp::items_collection() {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        MongoOp::set_index(&coll, &doc! { "wallet_id" : 1, "name" : 1, "type" : 1 });
        match MongoOp::insert_doc(&coll, doc){
            Err(e) => {
                error!("MongoOp::insert_item_doc error = {:?}", e);
                return Err(e);
            },
            Ok(b) => {
                trace!("MongoOp::insert_item_doc OK");
                return Ok(b);
            }
        }
    }

    fn update_item_doc(wallet_id:&str, type_:&[u8], item_id:&[u8], doc:&Document) -> Result<(), IndyError> {
        let coll = match MongoOp::items_collection() {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        let filter = doc! {
            "wallet_id": wallet_id,
            "type": MongoOp::bson_binary(type_),
            "name": MongoOp::bson_binary(item_id)
        };

        trace!("MongoOp::update_item_doc filter = {:?}", filter);
        trace!("MongoOp::update_item_doc doc = {:?}", doc);

        match MongoOp::update_doc(&coll, filter,doc.clone()) {
            Err(e) => Err(e),
            Ok(r) => {
                if r.matched_count == 0 {
                    return Err(IndyError::from(IndyErrorKind::WalletItemNotFound));
                }
                Ok(())
            }
        }
    }

    fn delete_storage(wallet_id:&str) -> Result<(), IndyError> {
        let filter = doc! {
            "wallet_id": wallet_id,
        };

        let mut coll = match MongoOp::items_collection() {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        match MongoOp::delete_all(&coll, &filter.clone()){
            Err(e) => return Err(e),
            Ok(()) => {}
        }

        coll = match MongoOp::metadata_collection() {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        match MongoOp::delete_all(&coll, &filter.clone()){
            Err(e) => return Err(e),
            Ok(()) => {}
        }

        coll = match MongoOp::wallets_collection() {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        MongoOp::delete_all(&coll, &filter.clone())
    }

    fn delete_item_doc(wallet_id:&str, type_:&[u8], id: &[u8]) -> Result<(), IndyError> {
        let coll = match MongoOp::items_collection() {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        let filter = doc! {
            "wallet_id": wallet_id,
            "type": MongoOp::bson_binary(type_),
            "name": MongoOp::bson_binary(id)
        };

        MongoOp::delete_doc(&coll, &filter.clone())
    }

    fn insert_tag_doc(doc:&Document, encrypted:bool) -> Result<Bson, IndyError> {
        trace!("insert_tag_doc {:?}", doc);
        let coll_in = if encrypted {
            MongoOp::tags_encrypted_collection()
        }else {
            MongoOp::tags_plaintext_collection()
        };

        let coll = match coll_in {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        //MongoOp::set_index(&coll, & doc!{"name": 1, "item_id": 1, "wallet_id": 1});
        match MongoOp::insert_doc(&coll, doc){
            Err(e) => {
                error!("MongoOp::insert_tag_doc - {:?}", e.kind());
                return Err(e);
            },
            Ok(t) => Ok(t)
        }
    }

    fn delete_tag_doc(wallet_id:&str, name:&[u8], item_id:&[u8], encrypted:bool) -> Result<(), IndyError> {
        let coll_in = if encrypted {
            MongoOp::tags_encrypted_collection()
        }else {
            MongoOp::tags_plaintext_collection()
        };

        let coll = match coll_in {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        let mut doc = Document::new();
        doc.insert("name", MongoOp::bson_binary(name));
        doc.insert("item_id", MongoOp::bson_binary(item_id));
        doc.insert("wallet_id", wallet_id);

        MongoOp::delete_doc(&coll, &doc)
    }

    fn find_tag_docs(wallet_id:&str, item_id:&[u8], encrypted:bool) -> Result<Cursor, IndyError> {
        let coll_in = if encrypted {
            MongoOp::tags_encrypted_collection()
        }else {
            MongoOp::tags_plaintext_collection()
        };

        let coll = match coll_in {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        let filter = doc! {
            "item_id": MongoOp::bson_binary(item_id),
            "wallet_id": wallet_id,
        };

        let cursor = match coll.find(Some(filter), None) {
            Err(e) => {
                error!("MongoOp::find_tag_docs - {:?}", e);
                return Err(IndyError::from_msg(IndyErrorKind::IOError, "Mongodb wallet error"));
            }
            Ok(t) => t,
        };

        Ok(cursor)

    }

    fn update_tag_doc(doc:&Document, encrypted:bool) -> Result<(), IndyError> {
        trace!("MongoOp::update_tag_doc {:?}", doc);
        let coll_in = if encrypted {
            MongoOp::tags_encrypted_collection()
        }else {
            MongoOp::tags_plaintext_collection()
        };

        let coll = match coll_in {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        //MongoOp::set_index(&coll, & doc!{"name": 1, "item_id": 1, "wallet_id": 1});

        let item_id = doc.get_binary_generic("item_id").unwrap().as_slice();
        let wallet_id = doc.get_str("wallet_id").unwrap();

        let filter = doc! {
            "item_id": MongoOp::bson_binary(item_id),
            "wailet_id": wallet_id
        };

        match MongoOp::update_doc(&coll, filter.clone(), doc.clone()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn update_doc(coll:&Collection, filter: Document, update: Document) -> Result<UpdateResult, IndyError> {
        let mut opt = UpdateOptions::new();
        opt.upsert = Some(true);

        let set = doc! {
            "$set": update
        };

        let result = match coll.update_one(filter, set, Some(opt)) {
            Err(e) => {
                error!("MongoOp::update_doc error is {:#?}", e);
                return Err(IndyError::from_msg(IndyErrorKind::IOError,"MongoDB error"));
            },
            Ok(w) => {
                if w.write_exception.is_some() {
                    let ex = w.write_exception.unwrap();
                    error!("MongoOp::update document exception !! {:}", ex);
                    return Err(IndyError::from_msg(IndyErrorKind::IOError, "Unable to update doc"));
                }
                w
            },
        } ;

        Ok(result)
    }

    fn insert_doc(coll:&Collection, doc:&Document) -> Result<Bson, IndyError> {
        let insert_id = match coll.insert_one(doc.clone(), None) {
            Err(e) => {
                error!("MongoOp::insert_docs - {:?}", e);
                return Err(IndyError::from_msg(IndyErrorKind::IOError,"Unable to create wallet"));
            },
            Ok(w) => {
                if w.write_exception.is_some() {
                    let ex = w.write_exception.unwrap();
                    error!("MongoOp::insert_doc exception !! {:}", ex);

                    //Duplicate index
                    let write_error = ex.write_error.unwrap();
                    if write_error.code == 11000 {
                        trace!("MongoOp::insert_doc returning indy error");
                        return Err(IndyError::from(IndyErrorKind::WalletItemAlreadyExists));
                    }

                    return Err(IndyError::from_msg(IndyErrorKind::IOError,"Unable to create wallet"));
                }
                w.inserted_id.unwrap()
            },
        } ;

        trace!("MongoOp::insert_doc - {:?}", doc);
        Ok(insert_id)
    }

    fn delete_doc(coll:&Collection, filter:&Document) -> Result<(), IndyError> {
        match coll.delete_one(filter.clone(), None) {
            Err(e) => {
                trace!("MongoOp::delete_doc doc - {:?}", e);
                return Err(IndyError::from_msg(IndyErrorKind::IOError, "Unable to connect to mongodb server"));
            },

            Ok(w) => {
                if w.write_exception.is_some() {
                    return Err(IndyError::from_msg(IndyErrorKind::IOError,"Unable to delete doc"));
                }

                if w.deleted_count == 0 {
                    return Err(IndyError::from(IndyErrorKind::WalletItemNotFound));
                }
            }
        }

        Ok(())
    }

    fn delete_all(coll:&Collection, filter:&Document) -> Result<(), IndyError> {
        match coll.delete_many(filter.clone(), None) {
            Err(e) => {
                error!("MongoOp::delete_doc - {:?}", e);
                return Err(IndyError::from_msg(IndyErrorKind::IOError, "Unable to connect to mongodb server"));
            },

            Ok(w) => {
                if w.write_exception.is_some() {
                    return Err(IndyError::from_msg(IndyErrorKind::IOError,"Unable to delete documents"));
                }
            }
        }

        Ok(())
    }

    fn find_doc(coll:&Collection, filter:&Document) -> Result<Option<Document>, IndyError> {
        match coll.find_one(Some(filter.clone()), None) {
            Err(e) => {
                error!("MongoOp::find_doc - {:?}", e);
                return Err(IndyError::from_msg(IndyErrorKind::IOError, "Unable to connect to mongodb server"));
            },

            Ok(results) => {
               return Ok(results);
            }
        }
    }

    fn wallet_exist_t(id:&str) -> Result<bool, IndyError> {
        let coll = match MongoOp::wallets_collection() {
            Err(e) => return Err(e),
            Ok(e) => { e }
        };

        return MongoOp::wallet_exist(id, &coll);
    }

    fn wallet_exist( id:&str, coll: &Collection ) -> Result<bool, IndyError> {
        MongoOp::doc_exist(doc! { "wallet_id" : id }, coll)
    }

    fn item_exist(wallet_id:&str, type_:&[u8], item_id:&[u8]) -> Result<bool, IndyError> {
        let coll = match MongoOp::items_collection() {
            Err(e) => return Err(e),
            Ok(e) => { e }
        };

        let filter = doc! {
            "wallet_id": wallet_id,
            "type": MongoOp::bson_binary(type_),
            "name": MongoOp::bson_binary(item_id)
        };

        MongoOp::doc_exist(filter, &coll)
    }


    fn item_find(wallet_id:&str, type_:&[u8], item_id:&[u8]) -> Result<Document, IndyError> {
        let coll = match MongoOp::items_collection() {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        let filter = doc! {
            "wallet_id": wallet_id,
            "type": MongoOp::bson_binary(type_),
            "name": MongoOp::bson_binary(item_id)
        };

        let item = match MongoOp::find_doc(&coll, &filter) {
            Err(e) => return Err(e),
            Ok(d) => {
                match d {
                    None => {
                        return Err(IndyError::from_msg(IndyErrorKind::WalletItemNotFound, "Wallet item not found"));
                    },
                    Some(d) => d
                }
            }
        };

        Ok(item)

    }

    fn doc_exist(doc:Document, coll: &Collection) -> Result<bool, IndyError> {
        match coll.find_one(Some(doc), None) {
            Err(e) => {
                error!("MongoOp::doc_exist - {:?}", e);
                return Err(IndyError::from_msg(IndyErrorKind::IOError, "Unable query mongodb"));
            },
            Ok(one) => {
                match one {
                    None => Ok(false),
                    Some(_) => Ok(true),
                }
            }
        }
    }

/*
    fn as_bson_vec(bson:&Bson) -> Option<&Vec<u8>> {
        match bson {
            Bson::Binary(_, ref vec) => return Some(vec),
            _ => return None,
        }
    }
*/

    fn bson_binary(data: &[u8]) -> Bson{
        Bson::Binary(bson::spec::BinarySubtype::Generic, data.to_vec())
    }

    /*
    fn bson_oid(oid: &str) -> Bson{
        let o = mongodb::oid::ObjectId::with_string(oid).unwrap();
        Bson::ObjectId(o)
    }*/

    fn wallet_doc(id:&str, metadata: &[u8]) -> Document {
        let mut doc = Document::new();
        doc.insert("wallet_id".to_owned(), id.to_owned());
        doc.insert("metadata".to_owned(), MongoOp::bson_binary(metadata));

        doc
    }

    fn item_doc(wallet_id:&str, type_: &[u8], id: &[u8], value: &EncryptedValue) -> Document {
        let mut doc = Document::new();
        doc.insert("wallet_id".to_owned(), wallet_id.to_owned());
        doc.insert("name".to_owned(), MongoOp::bson_binary(id));
        doc.insert("type".to_owned(), MongoOp::bson_binary(type_));
        doc.insert("value".to_owned(), MongoOp::bson_binary(&value.data));
        doc.insert("key".to_owned(), MongoOp::bson_binary(&value.key));

        doc
    }

    fn tags_doc(wallet_id:&str, name:&[u8], item_id:&[u8], value_encrypted:&[u8], value_decrypted:&str ) -> Document {
        let mut doc = Document::new();
        doc.insert("wallet_id".to_owned(), wallet_id.to_owned());
        doc.insert("name", MongoOp::bson_binary(name));
        doc.insert("item_id", MongoOp::bson_binary(item_id));

        if value_decrypted == "" {
            doc.insert("value", MongoOp::bson_binary(value_encrypted));
        }else{
            doc.insert("value", value_decrypted.to_owned());
        }

        doc
    }

    fn metadata_doc(wallet_id:&str, value:&[u8] ) -> Document {
        let mut doc = Document::new();
        doc.insert("wallet_id".to_owned(), wallet_id.to_owned());
        doc.insert("value", MongoOp::bson_binary(value));

        doc
    }
}

impl MongoStorage {
    pub fn new(id: &str) -> MongoStorage{
        MongoStorage{ wallet_id: id.to_string()}
    }

    fn _add_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag], update:bool) -> Result<(), IndyError> {
        //println!("update_tags type = {:?} id = {:?}", type_, id);

        match MongoOp::item_exist(&self.wallet_id.clone(), type_, id) {
            Err(e) => return Err(e),
            Ok(exists) => {
                if !exists {
                    return Err(IndyError::from(IndyErrorKind::WalletItemNotFound));
                }
            },
        }

        if !tags.is_empty() {
            for tag in tags {
                match tag {
                    &Tag::Encrypted(ref tag_name, ref tag_data) => {
                        trace!("add / update_tag tag_name is {:?} value is {:?}", tag_name.as_slice(), tag_data.as_slice());
                        let doc = MongoOp::tags_doc(&self.wallet_id.clone(), tag_name.as_slice(), id, &tag_data.as_slice(),"");

                        if update {
                            match MongoOp::update_tag_doc(&doc, true){
                                Err(e) => return Err(e),
                                Ok(_) => {}
                            }
                        }else {
                            match MongoOp::insert_tag_doc(&doc, true) {
                                Err(e) => return Err(e),
                                Ok(_) => {}
                            }
                        }
                    },

                    &Tag::PlainText(ref tag_name, ref tag_data) => {
                        trace!("update_tag tag_name is {:?}, tag_data = {:?}", tag_name.as_slice(), tag_data);
                        let doc = MongoOp::tags_doc( &self.wallet_id.clone(),tag_name.as_slice(), id, &[], tag_data);

                        if update {
                            match MongoOp::update_tag_doc(&doc, false) {
                                Err(e) => return Err(e),
                                Ok(_) => {}
                            }
                        }else {
                            match MongoOp::insert_tag_doc(&doc, false) {
                                Err(e) => return Err(e),
                                Ok(_) => {}
                            }
                        }
                    }
                };
            }
        }

        Ok(())
    }

    fn _delete_all_tags(&self, item_id:&[u8]) -> Result<(), IndyError>{
        let coll_plaintext = match MongoOp::tags_plaintext_collection() {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        let coll_encrypted = match MongoOp::tags_encrypted_collection() {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        //Delete all
        let query = doc! {
            "wallet_id" : self.wallet_id.clone(),
            "item_id" : MongoOp::bson_binary(item_id)
        };

        match coll_plaintext.delete_many(query.clone(), None) {
            Err(e) => {
                println!("ERROR _delete_all_tags - {:?}", e);
                return Err(IndyError::from_msg(IndyErrorKind::IOError, "Delete many fails"))
            },
            Ok(_) => {}
        }

        match coll_encrypted.delete_many(query.clone(), None) {
            Err(e) => {
                println!("ERROR delete_many - {:?}", e);
                return Err(IndyError::from_msg(IndyErrorKind::IOError, "Delete many fails"))
            },
            Ok(_) => Ok(())
        }
    }

    fn _retrieve_tags(wallet_id:&str, item_id:&[u8]) -> Result<Vec<Tag>, IndyError>{
        let mut tags = Vec::new();

        //get all encrypted
        let mut cursor_encrypted = match MongoOp::find_tag_docs(wallet_id,item_id, true) {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        while cursor_encrypted.has_next().unwrap() {
            let row = match cursor_encrypted.next() {
                Some(Ok(doc)) => doc,
                Some(Err(e)) => {
                    println!("ERROR _retrieve_tags - {:?}", e);
                    return Err(IndyError::from_msg(IndyErrorKind::IOError, "Cursor io error"));
                },
                None => {
                    return Err(IndyError::from_msg(IndyErrorKind::IOError, "Cursor io error"));
                }
            };

            println!("row is {:?}", row);

            let name = row.get_binary_generic("name").unwrap();
            let value = row.get_binary_generic("value").unwrap();
            let tag = Tag::Encrypted(name.clone(), value.clone());
            tags.push(tag);

            println!("tag = {:?}", Tag::Encrypted(name.clone(), value.clone()));
        }

        //get all plaintext
        let mut cursor_plaintext = match MongoOp::find_tag_docs(wallet_id, item_id, false) {
            Err(e) => return Err(e),
            Ok(c) => c
        };

        while cursor_plaintext.has_next().unwrap() {
            let row = match cursor_plaintext.next() {
                Some(Ok(doc)) => doc,
                Some(Err(e)) => {
                    println!("ERROR _retrieve_tags - {:?}", e);
                    return Err(IndyError::from_msg(IndyErrorKind::IOError, "Cursor io error"));
                },
                None => {
                    return Err(IndyError::from_msg(IndyErrorKind::IOError, "Cursor io error"));
                }
            };

            println!("row is {:?}", row);

            let name = row.get_binary_generic("name").unwrap();
            let value = row.get_str("value").unwrap().to_string();
            tags.push(Tag::PlainText(name.clone(), value));
        }

        Ok(tags)
    }

}

impl WalletStorage for MongoStorage {
    fn get(&self, type_: &[u8], id: &[u8], options: &str) -> Result<StorageRecord, IndyError>{
        let options: RecordOptions = if options == "{}" { // FIXME:
            RecordOptions::default()
        } else {
            serde_json::from_str(options)
                .to_indy(IndyErrorKind::InvalidStructure, "RecordOptions is malformed json")?
        };

        let item = match MongoOp::item_find(&self.wallet_id, type_, id) {
            Err(e) => return Err(e),
            Ok(i) => i
        };

        let value = if options.retrieve_value {
            let key = item.get_binary_generic("key").unwrap().clone();
            let value = item.get_binary_generic("value").unwrap().clone();
            Some(EncryptedValue::new(value, key))
        } else {
            None
        };

        let type_ = if options.retrieve_type { Some(type_.clone()) } else { None };
        let tags = if options.retrieve_tags {
            match MongoStorage::_retrieve_tags(&self.wallet_id, id) {
                Err(e) => return Err(e),
                Ok(t) => Some(t)
            }
        } else { None };

        Ok(StorageRecord::new(id.to_vec(), value, type_.map(|val| val.to_vec()), tags))

    }

    fn add(&self, type_: &[u8], id: &[u8], value: &EncryptedValue, tags: &[Tag]) -> Result<(), IndyError>{

        let doc = MongoOp::item_doc(&self.wallet_id.clone(), type_, id, value);
        let insert_id = match MongoOp::insert_item_doc(&doc) {
            Err(e) => {
                error!("MongoOp::add() insert_item_doc error {:}", e.kind());
                return Err(e)
            },
            Ok(t) => t
        };

        trace!("MongoOp::add() insert id is {:}", insert_id);

        if !tags.is_empty() {
            for tag in tags {
                match tag {
                    &Tag::Encrypted(ref tag_name, ref tag_data) => {
                        //println!("add tag_name is {:?}", tag_name.as_slice());
                        let doc = MongoOp::tags_doc(&self.wallet_id.clone(),tag_name.as_slice(), id, &tag_data.as_slice(),"");
                        MongoOp::insert_tag_doc(&doc, true).unwrap();
                    },

                    &Tag::PlainText(ref tag_name, ref tag_data) => {
                        //println!("add tag_name is {:?}", tag_name.as_slice());
                        let doc = MongoOp::tags_doc(&self.wallet_id.clone(),tag_name.as_slice(), id, &[], tag_data);
                        MongoOp::insert_tag_doc(&doc, false).unwrap();
                    }
                };
            }
        }

        Ok(())
    }

    fn update(&self, type_: &[u8], id: &[u8], value: &EncryptedValue) -> Result<(), IndyError>{
        let doc = MongoOp::item_doc(&self.wallet_id.clone(), type_, id, value);

        MongoOp::update_item_doc(&self.wallet_id.clone(), type_, id, &doc)
    }

    fn add_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), IndyError>{
       self._add_tags(type_, id, tags, true)
    }

    fn update_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), IndyError> {
        match self._delete_all_tags(id) {
            Err(e) => return Err(e),
            Ok(_) => {}
        };

        self._add_tags(type_, id, tags, false)
    }

    fn delete_tags(&self, type_: &[u8], id: &[u8], tag_names: &[TagName]) -> Result<(), IndyError>{
        match MongoOp::item_exist(&self.wallet_id.clone(), type_, id) {
            Err(e) => return Err(e),
            Ok(exists) => {
                if !exists {
                    return Err(IndyError::from(IndyErrorKind::WalletItemNotFound));
                }
            },
        }

        for tag_name in tag_names {
            match tag_name {
                &TagName::OfEncrypted(ref tag_name) => {
                    MongoOp::delete_tag_doc(&self.wallet_id,tag_name.as_slice(), id, true).unwrap();
                },
                &TagName::OfPlain(ref tag_name) => {
                    MongoOp::delete_tag_doc(&self.wallet_id, tag_name.as_slice(), id, false).unwrap();
                },
            };
        }

        Ok(())
    }

    fn delete(&self, type_: &[u8], id: &[u8]) -> Result<(), IndyError>{
        match MongoOp::delete_item_doc(&self.wallet_id.clone(), type_, id) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }

        self._delete_all_tags(id)
    }

    fn get_storage_metadata(&self) -> Result<Vec<u8>, IndyError>{
        let metadata = match MongoOp::find_metadata_doc(&self.wallet_id.clone()) {
            Err(e) => return Err(e),
            Ok(m) => m
        };

        let value = metadata.get_binary_generic("value").unwrap();
        Ok(value.to_vec().clone())
    }

    fn set_storage_metadata(&self, metadata: &[u8]) -> Result<(), IndyError>{
        match MongoOp::update_metadata_doc(&self.wallet_id.clone(), metadata) {
            Err(e) => return Err(e),
            Ok(_) => Ok(())
        }
    }

    fn get_all(&self) -> Result<Box<StorageIterator>, IndyError>{
        let fetch_options = RecordOptions {
            retrieve_type: true,
            retrieve_value: true,
            retrieve_tags: true,
        };

        let iter = MongoStorageIterator::new(self.wallet_id.clone(), Vec::new(), fetch_options, IteratorOp::All);

        Ok(Box::new(iter))
    }

    fn search(&self, _type_: &[u8], _query: &language::Operator, options: Option<&str>) -> Result<Box<StorageIterator>, IndyError>{
        let type_ = _type_.to_vec(); // FIXME

        let search_options = match options {
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

        Ok(Box::new(MongoStorageIterator::new(self.wallet_id.clone(), type_, fetch_options, IteratorOp::Search)))
    }

    fn close(&mut self) -> Result<(), IndyError>{ Ok(()) }
}

#[derive(Clone, Copy)]
enum IteratorOp {
    All,
    Search
}

struct MongoStorageIterator {
    options: RecordOptions,
    total_count: Option<i32>,
    operation: IteratorOp,
    cursor: Option<Cursor>,
    wallet_id: String,
    _type_: Vec<u8>
}

impl MongoStorageIterator {
    pub fn new(wallet_id:String, _type_: Vec<u8>, options: RecordOptions, operation:IteratorOp, ) -> MongoStorageIterator {
        MongoStorageIterator {
            options: options,
            total_count: None,
            operation: operation,
            cursor: None,
            wallet_id: wallet_id,
            _type_: _type_
        }
    }
}

impl StorageIterator for MongoStorageIterator {
    fn next(&mut self) -> Result<Option<StorageRecord>, IndyError>{

        match self.cursor {
            Some(_) => {},
            None => {
               self.cursor = match self.operation {
                   //TODO - Search operators
                    _ => {
                        let coll = match MongoOp::items_collection() {
                            Err(e) => return Err(e),
                            Ok(c) => c
                        };

                        let filter = doc! {
                            "wallet_id": self.wallet_id.clone(),
                            "type": MongoOp::bson_binary(&self._type_)
                        };

                        match coll.find(Some(filter), None) {
                            Err(e) => {
                                error!("MongoStorageIterator::next() - {:?}", e);
                                return Err(IndyError::from_msg(IndyErrorKind::IOError, "Cursor io error"))
                            },
                            Ok(c) => Some(c)
                        }
                    }
                };
            }
        };

        let  cursor = self.cursor.as_mut().unwrap();

        let row = match cursor.next() {
            Some(Ok(r)) => r,
            Some(Err(e)) => {
                error!("MongoStorageIterator::next() - {:?}", e);
                return Err(IndyError::from_msg(IndyErrorKind::IOError, "Cursor io error"))
            },
            None => return Ok(None)
        };

        let value = if self.options.retrieve_value {
            let key = row.get_binary_generic("key").unwrap().clone();
            let value = row.get_binary_generic("value").unwrap().clone();
            Some(EncryptedValue::new(value, key))
        } else {
            None
        };

        let id = row.get_binary_generic("name").unwrap();
        let type_ = row.get_binary_generic("type").unwrap().to_vec();
        let type_ = if self.options.retrieve_type { Some(type_.clone()) } else { None };
        let tags = if self.options.retrieve_tags {

            match MongoStorage::_retrieve_tags(&self.wallet_id, id.to_vec().as_slice()) {
                Err(e) => return Err(e),
                Ok(t) => Some(t)
            }
        } else { None };

        let rec = Some(StorageRecord::new(id.clone().to_vec(), value, type_.map(|val| val.to_vec()), tags));

        Ok(rec)
    }

    fn get_total_count(&self) -> Result<Option<usize>, IndyError>{
        match self.total_count {
            None => {
                //query count
                let coll = match MongoOp::items_collection() {
                    Err(e) => return Err(e),
                    Ok(c) => c
                };

                let filter = doc!{};
                let count = match coll.count(Some(filter), None) {
                    Err(e) => {
                        error!("MongoStorageIterator::get_total_count() - {:?}", e);
                        return Err(IndyError::from_msg(IndyErrorKind::IOError, "Count error"))
                    },
                    Ok(c) => c
                };

                Ok(Some(count as usize))
            },
            Some(count) => Ok(Some(count as usize))
        }
    }
}

impl MongoStorageType {
    pub fn new() -> MongoStorageType {
        MongoStorageType{}
    }
}

impl WalletStorageType for MongoStorageType {
    fn create_storage(&self, id: &str, _config: Option<&str>, _credentials: Option<&str>, metadata: &[u8]) -> Result<(), IndyError>{
        let coll = match MongoOp::wallets_collection() {
            Err(e) => return Err(e),
            Ok(e) => { e }
        };

        let doc = MongoOp::wallet_doc(id, metadata);

        match MongoOp::wallet_exist(id, &coll) {
            Ok(exists) => {
                if !exists {
                    //Create one
                    let r = coll.insert_one(doc.clone(), None);
                    match r {
                        Err(e) => {
                            error!("MongoStorageType::create_storage - {:?}", e);
                            return Err(IndyError::from_msg(IndyErrorKind::IOError,"Unable to create wallet"));
                        },
                        Ok(w) => {
                            if w.write_exception.is_some() {
                                return Err(IndyError::from_msg(IndyErrorKind::IOError,"Unable to create wallet"));
                            }
                        },
                    };

                    //Create metadata
                    let storage = MongoStorage::new(id);
                    match storage.set_storage_metadata(metadata){
                        Err(e) => return Err(e),
                        Ok(_) => {}
                    }
                }else{
                    return Err(IndyError::from(IndyErrorKind::WalletAlreadyExists));
                }
            },

            Err(e) => return Err(e)
        }

        Ok(())

    }

    fn open_storage(&self, id: &str, _config: Option<&str>, _credentials: Option<&str>) -> Result<Box<WalletStorage>, IndyError>{
        print!("mongo open_storage");

        match MongoOp::wallet_exist_t(id) {
            Ok(exist) => {
                if exist {
                    Ok(Box::new(MongoStorage::new(id)))
                }else{
                    return Err(IndyError::from(IndyError::from(IndyErrorKind::WalletNotFound)));
                }
            }
            Err(e) => return Err(e)
        }
    }

    fn delete_storage(&self, id: &str, _config: Option<&str>, _credentials: Option<&str>) -> Result<(), IndyError>{
        print!("mongo delete_storage");

        match MongoOp::wallet_exist_t(id) {
            Ok(exist) => {
                if exist {
                    return MongoOp::delete_storage(id)
                }else{
                    return Err(IndyError::from(IndyErrorKind::WalletNotFound));
                }
            }
            Err(e) => return Err(e)
        }
    }

}

#[cfg(test)]
mod ccis_tests {

    use super::*;
    use super::super::Tag;
    use utils::environment;

    static mut TEST_CLIENT:Option<Client> = None;

    #[test]
    fn ccis_storage_type_create_works() {
        _cleanup();

        let storage_type = MongoStorageType::new();
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();
    }

    #[test]
    fn ccis_storage_type_create_works_for_twice() {
        _cleanup();

        let storage_type = MongoStorageType::new();
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();

        let res = storage_type.create_storage(_wallet_id(), None, None, &_metadata());
        assert_kind!(IndyErrorKind::WalletAlreadyExists, res);
    }

    #[test]
    fn ccis_storage_get_storage_metadata_works() {
        _cleanup();

        let storage = _storage();
        let metadata = storage.get_storage_metadata().unwrap();

        assert_eq!(metadata, _metadata());
    }

    #[test]
    fn ccis_storage_type_delete_works() {
        _cleanup();

        let storage_type = MongoStorageType::new();
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();

        storage_type.delete_storage(_wallet_id(), None, None).unwrap();
    }

    #[test]
    fn ccis_storage_type_delete_works_for_non_existing() {
        _cleanup();

        let storage_type = MongoStorageType::new();
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();

        let res = storage_type.delete_storage("unknown", None, None);
        assert_kind!(IndyErrorKind::WalletNotFound, res);

        storage_type.delete_storage(_wallet_id(), None, None).unwrap();
    }

    #[test]
    fn ccis_storage_type_open_works() {
        _cleanup();
        _storage();
    }

    #[test]
    fn ccis_storage_type_open_works_for_not_created() {
        _cleanup();

        let storage_type = MongoStorageType::new();

        let res = storage_type.open_storage("unknown", Some("{}"), Some("{}"));
        assert_kind!(IndyErrorKind::WalletNotFound, res);
    }

    #[test]
    fn ccis_storage_add_works_for_is_802() {
        _cleanup();

        let storage = _storage();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.add(&_type1(), &_id1(), &_value1(), &_tags());
        assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);

        let res = storage.add(&_type1(), &_id1(), &_value1(), &_tags());
        assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);
    }

    #[test]
    fn ccis_storage_set_get_works() {
        _cleanup();

        let storage = _storage();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();

        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));
    }

    #[test]
    fn ccis_storage_set_get_works_for_twice() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.add(&_type1(), &_id1(), &_value2(), &_tags());
        assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);
    }

    #[test]
    fn ccis_storage_set_get_works_for_reopen() {
        _cleanup();

        {
            _storage().add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        }

        let storage_type = MongoStorageType::new();
        let storage = storage_type.open_storage(_wallet_id(), Some("{}"), Some("{}")).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();

        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));
    }

    #[test]
    fn ccis_storage_get_works_for_wrong_key() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.get(&_type1(), &_id2(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##);
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    #[test]
    fn ccis_storage_delete_works() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));

        storage.delete(&_type1(), &_id1()).unwrap();
        let res = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##);
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    #[test]
    fn ccis_storage_delete_works_for_non_existing() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.delete(&_type1(), &_id2());
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    #[test]
    fn ccis_storage_delete_returns_error_item_not_found_if_no_such_type() {
        _cleanup();

        let storage = _storage();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        let res = storage.delete(&_type2(), &_id2());
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    #[test]
    fn ccis_storage_get_all_works() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        storage.add(&_type2(), &_id2(), &_value2(), &_tags()).unwrap();

        let mut storage_iterator = storage.get_all().unwrap();

        let record = storage_iterator.next().unwrap().unwrap();
        assert_eq!(record.type_.unwrap(), _type1());
        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));

        let record = storage_iterator.next().unwrap().unwrap();
        assert_eq!(record.type_.unwrap(), _type2());
        assert_eq!(record.value.unwrap(), _value2());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));

        let record = storage_iterator.next().unwrap();
        assert!(record.is_none());
    }

    #[test]
    fn ccis_storage_get_all_works_for_empty() {
        _cleanup();

        let storage = _storage();
        let mut storage_iterator = storage.get_all().unwrap();

        let record = storage_iterator.next().unwrap();
        assert!(record.is_none());
    }

    #[test]
    fn ccis_storage_update_works() {
        _cleanup();

        let storage = _storage();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        storage.update(&_type1(), &_id1(), &_value2()).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value2());
    }

    #[test]
    fn ccis_storage_update_works_for_non_existing_id() {
        _cleanup();

        let storage = _storage();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        let res = storage.update(&_type1(), &_id2(), &_value2());
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    #[test]
    fn ccis_storage_update_works_for_non_existing_type() {
        _cleanup();

        let storage = _storage();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        let res = storage.update(&_type2(), &_id1(), &_value2());
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    #[test]
    fn ccis_storage_add_tags_works() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        storage.add_tags(&_type1(), &_id1(), &_new_tags()).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        let expected_tags = {
            let mut tags = _tags();
            tags.extend(_new_tags());
            _sort(tags)
        };

        assert_eq!(_sort(record.tags.unwrap()), expected_tags);
    }

    #[test]
    fn ccis_storage_add_tags_works_for_non_existing_id() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.add_tags(&_type1(), &_id2(), &_new_tags());
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    #[test]
    fn ccis_storage_add_tags_works_for_non_existing_type() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.add_tags(&_type2(), &_id1(), &_new_tags());
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    #[test]
    fn ccis_storage_add_tags_works_for_already_existing() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let tags_with_existing = {
            let mut tags = _tags();
            tags.extend(_new_tags());
            tags
        };

        storage.add_tags(&_type1(), &_id1(), &tags_with_existing).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        let expected_tags = {
            let mut tags = _tags();
            tags.extend(_new_tags());
            _sort(tags)
        };

        assert_eq!(_sort(record.tags.unwrap()), expected_tags);
    }

    #[test]
    fn ccis_storage_update_tags_works() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        storage.update_tags(&_type1(), &_id1(), &_new_tags()).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_new_tags()));
    }

    #[test]
    fn ccis_storage_update_tags_works_for_non_existing_id() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.update_tags(&_type1(), &_id2(), &_new_tags());
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    #[test]
    fn ccis_storage_update_tags_works_for_non_existing_type() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.update_tags(&_type1(), &_id2(), &_new_tags());
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    #[test]
    fn ccis_storage_update_tags_works_for_already_existing() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let tags_with_existing = {
            let mut tags = _tags();
            tags.extend(_new_tags());
            tags
        };

        storage.update_tags(&_type1(), &_id1(), &tags_with_existing).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        let expected_tags = {
            let mut tags = _tags();
            tags.extend(_new_tags());
            _sort(tags)
        };

        assert_eq!(_sort(record.tags.unwrap()), expected_tags);
    }

    #[test]
    fn ccis_storage_delete_tags_works() {
        _cleanup();

        let storage = _storage();

        let tag_name1 = vec![0, 0, 0];
        let tag_name2 = vec![1, 1, 1];
        let tag_name3 = vec![2, 2, 2];
        let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
        let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
        let tags = vec![tag1.clone(), tag2.clone(), tag3.clone()];

        storage.add(&_type1(), &_id1(), &_value1(), &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name1.clone()), TagName::OfPlain(tag_name2.clone())];
        storage.delete_tags(&_type1(), &_id1(), &tag_names).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.tags.unwrap(), vec![tag3]);
    }

    #[test]
    fn ccis_storage_delete_tags_works_for_non_existing_type() {
        _cleanup();

        let storage = _storage();

        let tag_name1 = vec![0, 0, 0];
        let tag_name2 = vec![1, 1, 1];
        let tag_name3 = vec![2, 2, 2];
        let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
        let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
        let tags = vec![tag1.clone(), tag2.clone(), tag3.clone()];

        storage.add(&_type1(), &_id1(), &_value1(), &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name1.clone()), TagName::OfPlain(tag_name2.clone())];
        let res = storage.delete_tags(&_type2(), &_id1(), &tag_names);
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    #[test]
    fn ccis_storage_delete_tags_works_for_non_existing_id() {
        _cleanup();

        let storage = _storage();

        let tag_name1 = vec![0, 0, 0];
        let tag_name2 = vec![1, 1, 1];
        let tag_name3 = vec![2, 2, 2];
        let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
        let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
        let tags = vec![tag1.clone(), tag2.clone(), tag3.clone()];

        storage.add(&_type1(), &_id1(), &_value1(), &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name1.clone()), TagName::OfPlain(tag_name2.clone())];
        let res = storage.delete_tags(&_type1(), &_id2(), &tag_names);
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    fn _storage() -> Box<WalletStorage> {
        let storage_type = MongoStorageType::new();
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();
        storage_type.open_storage(_wallet_id(), None, None).unwrap()
    }

    fn _cleanup() {

        unsafe {
            if TEST_CLIENT.is_none() {
                let c = Client::connect("127.0.0.1", 27017);
                if c.is_err() {
                    panic!("Unable to connect to mongodb server");
                };
                let mc = c.unwrap();
                TEST_CLIENT = Some(mc);
            }

            let client = match TEST_CLIENT {
                Some(ref c) => c,
                None => { panic!("No MongoDB test client!!") }
            };

            match client.db("ccis_indy_wallets").drop_database(){
                Err(_) => {}
                Ok(_) => {}
            }

        }
    }

    fn _wallet_id() -> &'static str {
        "w1"
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

    fn _type(i: u8) -> Vec<u8> {
        vec![i, 1 + i, 2 + i]
    }

    fn _type1() -> Vec<u8> {
        _type(1)
    }

    fn _type2() -> Vec<u8> {
        _type(2)
    }

    fn _id(i: u8) -> Vec<u8> {
        vec![3 + i, 4 + i, 5 + i]
    }

    fn _id1() -> Vec<u8> {
        _id(1)
    }

    fn _id2() -> Vec<u8> {
        _id(2)
    }

    fn _value(i: u8) -> EncryptedValue {
        EncryptedValue { data: vec![6 + i, 7 + i, 8 + i], key: vec![9 + i, 10 + i, 11 + i] }
    }

    fn _value1() -> EncryptedValue {
        _value(1)
    }

    fn _value2() -> EncryptedValue {
        _value(2)
    }

    fn _tags() -> Vec<Tag> {
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![1, 5, 8], vec![3, 5, 6]));
        tags.push(Tag::PlainText(vec![1, 5, 8, 1], "Plain value".to_string()));
        tags
    }

    fn _new_tags() -> Vec<Tag> {
        vec![
            Tag::Encrypted(vec![1, 1, 1], vec![2, 2, 2]),
            Tag::PlainText(vec![1, 1, 1], String::from("tag_value_3"))
        ]
    }

    fn _sort(mut v: Vec<Tag>) -> Vec<Tag> {
        v.sort();
        v
    }

    fn _custom_path() -> String {
        let mut path = environment::tmp_path();
        path.push("custom_wallet_path");
        path.to_str().unwrap().to_owned()
    }

}
