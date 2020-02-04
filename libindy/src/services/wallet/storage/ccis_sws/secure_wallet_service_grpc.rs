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

const METHOD_SECURE_WALLET_CREATE_WALLET: ::grpcio::Method<super::secure_wallet_service::CreateWalletRequest, super::secure_wallet_service::CreateWalletResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/CreateWallet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_DELETE_WALLET: ::grpcio::Method<super::secure_wallet_service::DeleteWalletRequest, super::secure_wallet_service::DeleteWalletResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/DeleteWallet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_WALLET_EXISTS: ::grpcio::Method<super::secure_wallet_service::WalletExistsRequest, super::secure_wallet_service::WalletExistsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/WalletExists",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_GET_WALLET_METADATA: ::grpcio::Method<super::secure_wallet_service::GetWalletMetadataRequest, super::secure_wallet_service::GetWalletMetadataResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/GetWalletMetadata",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_SET_WALLET_METADATA: ::grpcio::Method<super::secure_wallet_service::SetWalletMetadataRequest, super::secure_wallet_service::SetWalletMetadataResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/SetWalletMetadata",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_ADD_WALLET_ITEM: ::grpcio::Method<super::secure_wallet_service::AddWalletItemRequest, super::secure_wallet_service::AddWalletItemResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/AddWalletItem",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_GET_WALLET_ITEM: ::grpcio::Method<super::secure_wallet_service::GetWalletItemRequest, super::secure_wallet_service::GetWalletItemResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/GetWalletItem",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_GET_ALL_WALLET_ITEMS: ::grpcio::Method<super::secure_wallet_service::GetAllWalletItemsRequest, super::secure_wallet_service::GetAllWalletItemsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/GetAllWalletItems",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_SEARCH_WALLET_ITEMS: ::grpcio::Method<super::secure_wallet_service::SearchWalletItemsRequest, super::secure_wallet_service::SearchWalletItemsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/SearchWalletItems",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM: ::grpcio::Method<super::secure_wallet_service::UpdateWalletItemRequest, super::secure_wallet_service::UpdateWalletItemResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/UpdateWalletItem",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_DELETE_WALLET_ITEM: ::grpcio::Method<super::secure_wallet_service::DeleteWalletItemRequest, super::secure_wallet_service::DeleteWalletItemResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/DeleteWalletItem",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_ADD_WALLET_ITEM_TAGS: ::grpcio::Method<super::secure_wallet_service::AddWalletItemTagsRequest, super::secure_wallet_service::AddWalletItemTagsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/AddWalletItemTags",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM_TAGS: ::grpcio::Method<super::secure_wallet_service::UpdateWalletItemTagsRequest, super::secure_wallet_service::UpdateWalletItemTagsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecureWallet/UpdateWalletItemTags",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURE_WALLET_DELETE_WALLET_ITEM_TAGS: ::grpcio::Method<super::secure_wallet_service::DeleteWalletItemTagsRequest, super::secure_wallet_service::DeleteWalletItemTagsResponse> = ::grpcio::Method {
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

    pub fn create_wallet_opt(&self, req: &super::secure_wallet_service::CreateWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::CreateWalletResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_CREATE_WALLET, req, opt)
    }

    pub fn create_wallet(&self, req: &super::secure_wallet_service::CreateWalletRequest) -> ::grpcio::Result<super::secure_wallet_service::CreateWalletResponse> {
        self.create_wallet_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_wallet_async_opt(&self, req: &super::secure_wallet_service::CreateWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::CreateWalletResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_CREATE_WALLET, req, opt)
    }

    pub fn create_wallet_async(&self, req: &super::secure_wallet_service::CreateWalletRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::CreateWalletResponse>> {
        self.create_wallet_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_opt(&self, req: &super::secure_wallet_service::DeleteWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::DeleteWalletResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_DELETE_WALLET, req, opt)
    }

    pub fn delete_wallet(&self, req: &super::secure_wallet_service::DeleteWalletRequest) -> ::grpcio::Result<super::secure_wallet_service::DeleteWalletResponse> {
        self.delete_wallet_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_async_opt(&self, req: &super::secure_wallet_service::DeleteWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::DeleteWalletResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_DELETE_WALLET, req, opt)
    }

    pub fn delete_wallet_async(&self, req: &super::secure_wallet_service::DeleteWalletRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::DeleteWalletResponse>> {
        self.delete_wallet_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wallet_exists_opt(&self, req: &super::secure_wallet_service::WalletExistsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::WalletExistsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_WALLET_EXISTS, req, opt)
    }

    pub fn wallet_exists(&self, req: &super::secure_wallet_service::WalletExistsRequest) -> ::grpcio::Result<super::secure_wallet_service::WalletExistsResponse> {
        self.wallet_exists_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wallet_exists_async_opt(&self, req: &super::secure_wallet_service::WalletExistsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::WalletExistsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_WALLET_EXISTS, req, opt)
    }

    pub fn wallet_exists_async(&self, req: &super::secure_wallet_service::WalletExistsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::WalletExistsResponse>> {
        self.wallet_exists_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_wallet_metadata_opt(&self, req: &super::secure_wallet_service::GetWalletMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::GetWalletMetadataResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_GET_WALLET_METADATA, req, opt)
    }

    pub fn get_wallet_metadata(&self, req: &super::secure_wallet_service::GetWalletMetadataRequest) -> ::grpcio::Result<super::secure_wallet_service::GetWalletMetadataResponse> {
        self.get_wallet_metadata_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_wallet_metadata_async_opt(&self, req: &super::secure_wallet_service::GetWalletMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::GetWalletMetadataResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_GET_WALLET_METADATA, req, opt)
    }

    pub fn get_wallet_metadata_async(&self, req: &super::secure_wallet_service::GetWalletMetadataRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::GetWalletMetadataResponse>> {
        self.get_wallet_metadata_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_wallet_metadata_opt(&self, req: &super::secure_wallet_service::SetWalletMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::SetWalletMetadataResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_SET_WALLET_METADATA, req, opt)
    }

    pub fn set_wallet_metadata(&self, req: &super::secure_wallet_service::SetWalletMetadataRequest) -> ::grpcio::Result<super::secure_wallet_service::SetWalletMetadataResponse> {
        self.set_wallet_metadata_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_wallet_metadata_async_opt(&self, req: &super::secure_wallet_service::SetWalletMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::SetWalletMetadataResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_SET_WALLET_METADATA, req, opt)
    }

    pub fn set_wallet_metadata_async(&self, req: &super::secure_wallet_service::SetWalletMetadataRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::SetWalletMetadataResponse>> {
        self.set_wallet_metadata_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_wallet_item_opt(&self, req: &super::secure_wallet_service::AddWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::AddWalletItemResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_ADD_WALLET_ITEM, req, opt)
    }

    pub fn add_wallet_item(&self, req: &super::secure_wallet_service::AddWalletItemRequest) -> ::grpcio::Result<super::secure_wallet_service::AddWalletItemResponse> {
        self.add_wallet_item_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_wallet_item_async_opt(&self, req: &super::secure_wallet_service::AddWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::AddWalletItemResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_ADD_WALLET_ITEM, req, opt)
    }

    pub fn add_wallet_item_async(&self, req: &super::secure_wallet_service::AddWalletItemRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::AddWalletItemResponse>> {
        self.add_wallet_item_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_wallet_item_opt(&self, req: &super::secure_wallet_service::GetWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::GetWalletItemResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_GET_WALLET_ITEM, req, opt)
    }

    pub fn get_wallet_item(&self, req: &super::secure_wallet_service::GetWalletItemRequest) -> ::grpcio::Result<super::secure_wallet_service::GetWalletItemResponse> {
        self.get_wallet_item_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_wallet_item_async_opt(&self, req: &super::secure_wallet_service::GetWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::GetWalletItemResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_GET_WALLET_ITEM, req, opt)
    }

    pub fn get_wallet_item_async(&self, req: &super::secure_wallet_service::GetWalletItemRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::GetWalletItemResponse>> {
        self.get_wallet_item_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_all_wallet_items_opt(&self, req: &super::secure_wallet_service::GetAllWalletItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::GetAllWalletItemsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_GET_ALL_WALLET_ITEMS, req, opt)
    }

    pub fn get_all_wallet_items(&self, req: &super::secure_wallet_service::GetAllWalletItemsRequest) -> ::grpcio::Result<super::secure_wallet_service::GetAllWalletItemsResponse> {
        self.get_all_wallet_items_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_all_wallet_items_async_opt(&self, req: &super::secure_wallet_service::GetAllWalletItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::GetAllWalletItemsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_GET_ALL_WALLET_ITEMS, req, opt)
    }

    pub fn get_all_wallet_items_async(&self, req: &super::secure_wallet_service::GetAllWalletItemsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::GetAllWalletItemsResponse>> {
        self.get_all_wallet_items_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_wallet_items_opt(&self, req: &super::secure_wallet_service::SearchWalletItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::SearchWalletItemsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_SEARCH_WALLET_ITEMS, req, opt)
    }

    pub fn search_wallet_items(&self, req: &super::secure_wallet_service::SearchWalletItemsRequest) -> ::grpcio::Result<super::secure_wallet_service::SearchWalletItemsResponse> {
        self.search_wallet_items_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_wallet_items_async_opt(&self, req: &super::secure_wallet_service::SearchWalletItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::SearchWalletItemsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_SEARCH_WALLET_ITEMS, req, opt)
    }

    pub fn search_wallet_items_async(&self, req: &super::secure_wallet_service::SearchWalletItemsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::SearchWalletItemsResponse>> {
        self.search_wallet_items_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_wallet_item_opt(&self, req: &super::secure_wallet_service::UpdateWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::UpdateWalletItemResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM, req, opt)
    }

    pub fn update_wallet_item(&self, req: &super::secure_wallet_service::UpdateWalletItemRequest) -> ::grpcio::Result<super::secure_wallet_service::UpdateWalletItemResponse> {
        self.update_wallet_item_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_wallet_item_async_opt(&self, req: &super::secure_wallet_service::UpdateWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::UpdateWalletItemResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM, req, opt)
    }

    pub fn update_wallet_item_async(&self, req: &super::secure_wallet_service::UpdateWalletItemRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::UpdateWalletItemResponse>> {
        self.update_wallet_item_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_item_opt(&self, req: &super::secure_wallet_service::DeleteWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::DeleteWalletItemResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_DELETE_WALLET_ITEM, req, opt)
    }

    pub fn delete_wallet_item(&self, req: &super::secure_wallet_service::DeleteWalletItemRequest) -> ::grpcio::Result<super::secure_wallet_service::DeleteWalletItemResponse> {
        self.delete_wallet_item_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_item_async_opt(&self, req: &super::secure_wallet_service::DeleteWalletItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::DeleteWalletItemResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_DELETE_WALLET_ITEM, req, opt)
    }

    pub fn delete_wallet_item_async(&self, req: &super::secure_wallet_service::DeleteWalletItemRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::DeleteWalletItemResponse>> {
        self.delete_wallet_item_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_wallet_item_tags_opt(&self, req: &super::secure_wallet_service::AddWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::AddWalletItemTagsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_ADD_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn add_wallet_item_tags(&self, req: &super::secure_wallet_service::AddWalletItemTagsRequest) -> ::grpcio::Result<super::secure_wallet_service::AddWalletItemTagsResponse> {
        self.add_wallet_item_tags_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_wallet_item_tags_async_opt(&self, req: &super::secure_wallet_service::AddWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::AddWalletItemTagsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_ADD_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn add_wallet_item_tags_async(&self, req: &super::secure_wallet_service::AddWalletItemTagsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::AddWalletItemTagsResponse>> {
        self.add_wallet_item_tags_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_wallet_item_tags_opt(&self, req: &super::secure_wallet_service::UpdateWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::UpdateWalletItemTagsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn update_wallet_item_tags(&self, req: &super::secure_wallet_service::UpdateWalletItemTagsRequest) -> ::grpcio::Result<super::secure_wallet_service::UpdateWalletItemTagsResponse> {
        self.update_wallet_item_tags_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_wallet_item_tags_async_opt(&self, req: &super::secure_wallet_service::UpdateWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::UpdateWalletItemTagsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_UPDATE_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn update_wallet_item_tags_async(&self, req: &super::secure_wallet_service::UpdateWalletItemTagsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::UpdateWalletItemTagsResponse>> {
        self.update_wallet_item_tags_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_item_tags_opt(&self, req: &super::secure_wallet_service::DeleteWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secure_wallet_service::DeleteWalletItemTagsResponse> {
        self.client.unary_call(&METHOD_SECURE_WALLET_DELETE_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn delete_wallet_item_tags(&self, req: &super::secure_wallet_service::DeleteWalletItemTagsRequest) -> ::grpcio::Result<super::secure_wallet_service::DeleteWalletItemTagsResponse> {
        self.delete_wallet_item_tags_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_wallet_item_tags_async_opt(&self, req: &super::secure_wallet_service::DeleteWalletItemTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::DeleteWalletItemTagsResponse>> {
        self.client.unary_call_async(&METHOD_SECURE_WALLET_DELETE_WALLET_ITEM_TAGS, req, opt)
    }

    pub fn delete_wallet_item_tags_async(&self, req: &super::secure_wallet_service::DeleteWalletItemTagsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secure_wallet_service::DeleteWalletItemTagsResponse>> {
        self.delete_wallet_item_tags_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SecureWallet {
    fn create_wallet(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::CreateWalletRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::CreateWalletResponse>);
    fn delete_wallet(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::DeleteWalletRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::DeleteWalletResponse>);
    fn wallet_exists(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::WalletExistsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::WalletExistsResponse>);
    fn get_wallet_metadata(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::GetWalletMetadataRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::GetWalletMetadataResponse>);
    fn set_wallet_metadata(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::SetWalletMetadataRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::SetWalletMetadataResponse>);
    fn add_wallet_item(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::AddWalletItemRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::AddWalletItemResponse>);
    fn get_wallet_item(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::GetWalletItemRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::GetWalletItemResponse>);
    fn get_all_wallet_items(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::GetAllWalletItemsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::GetAllWalletItemsResponse>);
    fn search_wallet_items(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::SearchWalletItemsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::SearchWalletItemsResponse>);
    fn update_wallet_item(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::UpdateWalletItemRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::UpdateWalletItemResponse>);
    fn delete_wallet_item(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::DeleteWalletItemRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::DeleteWalletItemResponse>);
    fn add_wallet_item_tags(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::AddWalletItemTagsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::AddWalletItemTagsResponse>);
    fn update_wallet_item_tags(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::UpdateWalletItemTagsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::UpdateWalletItemTagsResponse>);
    fn delete_wallet_item_tags(&mut self, ctx: ::grpcio::RpcContext, req: super::secure_wallet_service::DeleteWalletItemTagsRequest, sink: ::grpcio::UnarySink<super::secure_wallet_service::DeleteWalletItemTagsResponse>);
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
