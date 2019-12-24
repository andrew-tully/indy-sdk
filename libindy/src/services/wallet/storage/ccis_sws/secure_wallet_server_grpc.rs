// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_SECURE_WALLET_CREATE_WALLET: ::grpcio::Method<super::secure_wallet_server::CreateWalletRequest, super::secure_wallet_server::CreateWalletResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/CreateWallet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_DELETE_WALLET: ::grpcio::Method<super::secure_wallet_server::DeleteWalletRequest, super::secure_wallet_server::DeleteWalletResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/DeleteWallet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_WALLET_EXISTS: ::grpcio::Method<super::secure_wallet_server::WalletExistsRequest, super::secure_wallet_server::WalletExistsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/WalletExists",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_GET_WALLET_METADATA: ::grpcio::Method<super::secure_wallet_server::GetWalletMetadataRequest, super::secure_wallet_server::GetWalletMetadataResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/GetWalletMetadata",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_SET_WALLET_METADATA: ::grpcio::Method<super::secure_wallet_server::SetWalletMetadataRequest, super::secure_wallet_server::SetWalletMetadataResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/SetWalletMetadata",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_ADD_WALLET_ITEM: ::grpcio::Method<super::secure_wallet_server::AddWalletItemRequest, super::secure_wallet_server::AddWalletItemResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/AddWalletItem",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_GET_WALLET_ITEM: ::grpcio::Method<super::secure_wallet_server::GetWalletItemRequest, super::secure_wallet_server::GetWalletItemResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/GetWalletItem",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_GET_ALL_WALLET_ITEMS: ::grpcio::Method<super::secure_wallet_server::GetAllWalletItemsRequest, super::secure_wallet_server::GetAllWalletItemsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/GetAllWalletItems",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_SEARCH_WALLET_ITEMS: ::grpcio::Method<super::secure_wallet_server::SearchWalletItemsRequest, super::secure_wallet_server::SearchWalletItemsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/SearchWalletItems",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM: ::grpcio::Method<super::secure_wallet_server::UpdateWalletItemRequest, super::secure_wallet_server::UpdateWalletItemResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/UpdateWalletItem",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_DELETE_WALLET_ITEM: ::grpcio::Method<super::secure_wallet_server::DeleteWalletItemRequest, super::secure_wallet_server::DeleteWalletItemResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/DeleteWalletItem",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_ADD_WALLET_ITEM_TAGS: ::grpcio::Method<super::secure_wallet_server::AddWalletItemTagsRequest, super::secure_wallet_server::AddWalletItemTagsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/AddWalletItemTags",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM_TAGS: ::grpcio::Method<super::secure_wallet_server::UpdateWalletItemTagsRequest, super::secure_wallet_server::UpdateWalletItemTagsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/UpdateWalletItemTags",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_DELETE_WALLET_ITEM_TAGS: ::grpcio::Method<super::secure_wallet_server::DeleteWalletItemTagsRequest, super::secure_wallet_server::DeleteWalletItemTagsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/DeleteWalletItemTags",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SecureWalletClient {
    client: ::grpcio::Client,
}

impl SecureWalletClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SecureWalletClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_wallet_opt(&self, req: &super::secure_wallet_server::CreateWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::CreateWalletResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_CREATE_WALLET, req, opt)
    }

    pub fn create_wallet(&self, req: &super::secure_wallet_server::CreateWalletRequest) -> ::grpcio::Result<super::secure_wallet_server::CreateWalletResponse> {
        self.create_wallet_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_wallet_async_opt(&self, req: &super::secure_wallet_server::CreateWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::CreateWalletResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_CREATE_WALLET, req, opt)
    }

    pub fn create_wallet_async(&self, req: &super::secure_wallet_server::CreateWalletRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::CreateWalletResponse>> {
        self.create_wallet_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_opt(&self, req: &super::secure_wallet_server::DeleteWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::DeleteWalletResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_DELETE_WALLET, req, opt)
    }

    pub fn delete_wallet(&self, req: &super::secure_wallet_server::DeleteWalletRequest) -> ::grpcio::Result<super::secure_wallet_server::DeleteWalletResponse> {
        self.delete_wallet_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_async_opt(&self, req: &super::secure_wallet_server::DeleteWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::DeleteWalletResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_DELETE_WALLET, req, opt)
    }

    pub fn delete_wallet_async(&self, req: &super::secure_wallet_server::DeleteWalletRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::DeleteWalletResponse>> {
        self.delete_wallet_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wallet_exists_opt(&self, req: &super::secure_wallet_server::WalletExistsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::WalletExistsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_WALLET_EXISTS, req, opt)
    }

    pub fn wallet_exists(&self, req: &super::secure_wallet_server::WalletExistsRequest) -> ::grpcio::Result<super::secure_wallet_server::WalletExistsResponse> {
        self.wallet_exists_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wallet_exists_async_opt(&self, req: &super::secure_wallet_server::WalletExistsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::WalletExistsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_WALLET_EXISTS, req, opt)
    }

    pub fn wallet_exists_async(&self, req: &super::secure_wallet_server::WalletExistsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::WalletExistsResponse>> {
        self.wallet_exists_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_wallet_metadata_opt(&self, req: &super::secure_wallet_server::GetWalletMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::GetWalletMetadataResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_GET_WALLET_METADATA, req, opt)
    }

    pub fn get_wallet_metadata(&self, req: &super::secure_wallet_server::GetWalletMetadataRequest) -> ::grpcio::Result<super::secure_wallet_server::GetWalletMetadataResponse> {
        self.get_wallet_metadata_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_wallet_metadata_async_opt(&self, req: &super::secure_wallet_server::GetWalletMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::GetWalletMetadataResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_GET_WALLET_METADATA, req, opt)
    }

    pub fn get_wallet_metadata_async(&self, req: &super::secure_wallet_server::GetWalletMetadataRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::GetWalletMetadataResponse>> {
        self.get_wallet_metadata_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_wallet_metadata_opt(&self, req: &super::secure_wallet_server::SetWalletMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::SetWalletMetadataResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_SET_WALLET_METADATA, req, opt)
    }

    pub fn set_wallet_metadata(&self, req: &super::secure_wallet_server::SetWalletMetadataRequest) -> ::grpcio::Result<super::secure_wallet_server::SetWalletMetadataResponse> {
        self.set_wallet_metadata_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_wallet_metadata_async_opt(&self, req: &super::secure_wallet_server::SetWalletMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::SetWalletMetadataResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_SET_WALLET_METADATA, req, opt)
    }

    pub fn set_wallet_metadata_async(&self, req: &super::secure_wallet_server::SetWalletMetadataRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::SetWalletMetadataResponse>> {
        self.set_wallet_metadata_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_wallet_item_opt(&self, req: &super::secure_wallet_server::AddWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::AddWalletItemResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_ADD_WALLET_ITEM, req, opt)
    }

    pub fn add_wallet_item(&self, req: &super::secure_wallet_server::AddWalletItemRequest) -> ::grpcio::Result<super::secure_wallet_server::AddWalletItemResponse> {
        self.add_wallet_item_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_wallet_item_async_opt(&self, req: &super::secure_wallet_server::AddWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::AddWalletItemResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_ADD_WALLET_ITEM, req, opt)
    }

    pub fn add_wallet_item_async(&self, req: &super::secure_wallet_server::AddWalletItemRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::AddWalletItemResponse>> {
        self.add_wallet_item_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_wallet_item_opt(&self, req: &super::secure_wallet_server::GetWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::GetWalletItemResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_GET_WALLET_ITEM, req, opt)
    }

    pub fn get_wallet_item(&self, req: &super::secure_wallet_server::GetWalletItemRequest) -> ::grpcio::Result<super::secure_wallet_server::GetWalletItemResponse> {
        self.get_wallet_item_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_wallet_item_async_opt(&self, req: &super::secure_wallet_server::GetWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::GetWalletItemResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_GET_WALLET_ITEM, req, opt)
    }

    pub fn get_wallet_item_async(&self, req: &super::secure_wallet_server::GetWalletItemRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::GetWalletItemResponse>> {
        self.get_wallet_item_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_all_wallet_items_opt(&self, req: &super::secure_wallet_server::GetAllWalletItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::GetAllWalletItemsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_GET_ALL_WALLET_ITEMS, req, opt)
    }

    pub fn get_all_wallet_items(&self, req: &super::secure_wallet_server::GetAllWalletItemsRequest) -> ::grpcio::Result<super::secure_wallet_server::GetAllWalletItemsResponse> {
        self.get_all_wallet_items_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_all_wallet_items_async_opt(&self, req: &super::secure_wallet_server::GetAllWalletItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::GetAllWalletItemsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_GET_ALL_WALLET_ITEMS, req, opt)
    }

    pub fn get_all_wallet_items_async(&self, req: &super::secure_wallet_server::GetAllWalletItemsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::GetAllWalletItemsResponse>> {
        self.get_all_wallet_items_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_wallet_items_opt(&self, req: &super::secure_wallet_server::SearchWalletItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::SearchWalletItemsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_SEARCH_WALLET_ITEMS, req, opt)
    }

    pub fn search_wallet_items(&self, req: &super::secure_wallet_server::SearchWalletItemsRequest) -> ::grpcio::Result<super::secure_wallet_server::SearchWalletItemsResponse> {
        self.search_wallet_items_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_wallet_items_async_opt(&self, req: &super::secure_wallet_server::SearchWalletItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::SearchWalletItemsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_SEARCH_WALLET_ITEMS, req, opt)
    }

    pub fn search_wallet_items_async(&self, req: &super::secure_wallet_server::SearchWalletItemsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::SearchWalletItemsResponse>> {
        self.search_wallet_items_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_wallet_item_opt(&self, req: &super::secure_wallet_server::UpdateWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::UpdateWalletItemResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM, req, opt)
    }

    pub fn update_wallet_item(&self, req: &super::secure_wallet_server::UpdateWalletItemRequest) -> ::grpcio::Result<super::secure_wallet_server::UpdateWalletItemResponse> {
        self.update_wallet_item_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_wallet_item_async_opt(&self, req: &super::secure_wallet_server::UpdateWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::UpdateWalletItemResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM, req, opt)
    }

    pub fn update_wallet_item_async(&self, req: &super::secure_wallet_server::UpdateWalletItemRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::UpdateWalletItemResponse>> {
        self.update_wallet_item_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_item_opt(&self, req: &super::secure_wallet_server::DeleteWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::DeleteWalletItemResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_DELETE_WALLET_ITEM, req, opt)
    }

    pub fn delete_wallet_item(&self, req: &super::secure_wallet_server::DeleteWalletItemRequest) -> ::grpcio::Result<super::secure_wallet_server::DeleteWalletItemResponse> {
        self.delete_wallet_item_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_item_async_opt(&self, req: &super::secure_wallet_server::DeleteWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::DeleteWalletItemResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_DELETE_WALLET_ITEM, req, opt)
    }

    pub fn delete_wallet_item_async(&self, req: &super::secure_wallet_server::DeleteWalletItemRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::DeleteWalletItemResponse>> {
        self.delete_wallet_item_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_wallet_item_tags_opt(&self, req: &super::secure_wallet_server::AddWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::AddWalletItemTagsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_ADD_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn add_wallet_item_tags(&self, req: &super::secure_wallet_server::AddWalletItemTagsRequest) -> ::grpcio::Result<super::secure_wallet_server::AddWalletItemTagsResponse> {
        self.add_wallet_item_tags_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_wallet_item_tags_async_opt(&self, req: &super::secure_wallet_server::AddWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::AddWalletItemTagsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_ADD_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn add_wallet_item_tags_async(&self, req: &super::secure_wallet_server::AddWalletItemTagsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::AddWalletItemTagsResponse>> {
        self.add_wallet_item_tags_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_wallet_item_tags_opt(&self, req: &super::secure_wallet_server::UpdateWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::UpdateWalletItemTagsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn update_wallet_item_tags(&self, req: &super::secure_wallet_server::UpdateWalletItemTagsRequest) -> ::grpcio::Result<super::secure_wallet_server::UpdateWalletItemTagsResponse> {
        self.update_wallet_item_tags_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_wallet_item_tags_async_opt(&self, req: &super::secure_wallet_server::UpdateWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::UpdateWalletItemTagsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn update_wallet_item_tags_async(&self, req: &super::secure_wallet_server::UpdateWalletItemTagsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::UpdateWalletItemTagsResponse>> {
        self.update_wallet_item_tags_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_item_tags_opt(&self, req: &super::secure_wallet_server::DeleteWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_server::DeleteWalletItemTagsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_DELETE_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn delete_wallet_item_tags(&self, req: &super::secure_wallet_server::DeleteWalletItemTagsRequest) -> ::grpcio::Result<super::secure_wallet_server::DeleteWalletItemTagsResponse> {
        self.delete_wallet_item_tags_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_item_tags_async_opt(&self, req: &super::secure_wallet_server::DeleteWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::DeleteWalletItemTagsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_DELETE_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn delete_wallet_item_tags_async(&self, req: &super::secure_wallet_server::DeleteWalletItemTagsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_server::DeleteWalletItemTagsResponse>> {
        self.delete_wallet_item_tags_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SecureWallet {
    fn create_wallet(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::CreateWalletRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::CreateWalletResponse>);
    fn delete_wallet(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::DeleteWalletRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::DeleteWalletResponse>);
    fn wallet_exists(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::WalletExistsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::WalletExistsResponse>);
    fn get_wallet_metadata(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::GetWalletMetadataRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::GetWalletMetadataResponse>);
    fn set_wallet_metadata(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::SetWalletMetadataRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::SetWalletMetadataResponse>);
    fn add_wallet_item(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::AddWalletItemRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::AddWalletItemResponse>);
    fn get_wallet_item(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::GetWalletItemRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::GetWalletItemResponse>);
    fn get_all_wallet_items(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::GetAllWalletItemsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::GetAllWalletItemsResponse>);
    fn search_wallet_items(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::SearchWalletItemsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::SearchWalletItemsResponse>);
    fn update_wallet_item(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::UpdateWalletItemRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::UpdateWalletItemResponse>);
    fn delete_wallet_item(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::DeleteWalletItemRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::DeleteWalletItemResponse>);
    fn add_wallet_item_tags(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::AddWalletItemTagsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::AddWalletItemTagsResponse>);
    fn update_wallet_item_tags(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::UpdateWalletItemTagsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::UpdateWalletItemTagsResponse>);
    fn delete_wallet_item_tags(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_server::DeleteWalletItemTagsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_server::DeleteWalletItemTagsResponse>);
}

pub fn create_secure_wallet<S: SecureWallet + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_CREATE_WALLET, move |ctx, req, resp| {
        instance.create_wallet(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_DELETE_WALLET, move |ctx, req, resp| {
        instance.delete_wallet(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_WALLET_EXISTS, move |ctx, req, resp| {
        instance.wallet_exists(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_GET_WALLET_METADATA, move |ctx, req, resp| {
        instance.get_wallet_metadata(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_SET_WALLET_METADATA, move |ctx, req, resp| {
        instance.set_wallet_metadata(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_ADD_WALLET_ITEM, move |ctx, req, resp| {
        instance.add_wallet_item(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_GET_WALLET_ITEM, move |ctx, req, resp| {
        instance.get_wallet_item(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_GET_ALL_WALLET_ITEMS, move |ctx, req, resp| {
        instance.get_all_wallet_items(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_SEARCH_WALLET_ITEMS, move |ctx, req, resp| {
        instance.search_wallet_items(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM, move |ctx, req, resp| {
        instance.update_wallet_item(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_DELETE_WALLET_ITEM, move |ctx, req, resp| {
        instance.delete_wallet_item(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_ADD_WALLET_ITEM_TAGS, move |ctx, req, resp| {
        instance.add_wallet_item_tags(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM_TAGS, move |ctx, req, resp| {
        instance.update_wallet_item_tags(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURE_WALLET_DELETE_WALLET_ITEM_TAGS, move |ctx, req, resp| {
        instance.delete_wallet_item_tags(ctx, req, resp)
    });
    builder.build()
}
