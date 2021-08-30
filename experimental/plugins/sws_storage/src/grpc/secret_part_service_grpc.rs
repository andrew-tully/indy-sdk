// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_SECRET_PART_PUT_SECRET_PART: ::grpcio::Method<super::secret_part_service::PutSecretPartRequest, super::secret_part_service::PutSecretPartResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecretPart/PutSecretPart",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECRET_PART_SET_WALLET_ENCRYPTION: ::grpcio::Method<super::secret_part_service::SetWalletEncryptionRequest, super::secret_part_service::SetWalletEncryptionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecretPart/SetWalletEncryption",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECRET_PART_ISSUE_WALLET_KEYS: ::grpcio::Method<super::secret_part_service::IssueWalletKeysRequest, super::secret_part_service::IssueWalletKeysResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecretPart/IssueWalletKeys",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECRET_PART_REISSUE_SECRET_PARTS: ::grpcio::Method<super::secret_part_service::ReissueSecretPartsRequest, super::secret_part_service::ReissueSecretPartsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecretPart/ReissueSecretParts",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECRET_PART_CREATE_WALLET_RECOVERY_KEYS: ::grpcio::Method<super::secret_part_service::CreateWalletRecoveryKeysRequest, super::secret_part_service::CreateWalletRecoveryKeysResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.SecretPart/CreateWalletRecoveryKeys",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SecretPartClient {
    client: ::grpcio::Client,
}

impl SecretPartClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SecretPartClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn put_secret_part_opt(&self, req: &super::secret_part_service::PutSecretPartRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secret_part_service::PutSecretPartResponse> {
        self.client.unary_call(&METHOD_SECRET_PART_PUT_SECRET_PART, req, opt)
    }

    pub fn put_secret_part(&self, req: &super::secret_part_service::PutSecretPartRequest) -> ::grpcio::Result<super::secret_part_service::PutSecretPartResponse> {
        self.put_secret_part_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_secret_part_async_opt(&self, req: &super::secret_part_service::PutSecretPartRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_part_service::PutSecretPartResponse>> {
        self.client.unary_call_async(&METHOD_SECRET_PART_PUT_SECRET_PART, req, opt)
    }

    pub fn put_secret_part_async(&self, req: &super::secret_part_service::PutSecretPartRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_part_service::PutSecretPartResponse>> {
        self.put_secret_part_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_wallet_encryption_opt(&self, req: &super::secret_part_service::SetWalletEncryptionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secret_part_service::SetWalletEncryptionResponse> {
        self.client.unary_call(&METHOD_SECRET_PART_SET_WALLET_ENCRYPTION, req, opt)
    }

    pub fn set_wallet_encryption(&self, req: &super::secret_part_service::SetWalletEncryptionRequest) -> ::grpcio::Result<super::secret_part_service::SetWalletEncryptionResponse> {
        self.set_wallet_encryption_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_wallet_encryption_async_opt(&self, req: &super::secret_part_service::SetWalletEncryptionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_part_service::SetWalletEncryptionResponse>> {
        self.client.unary_call_async(&METHOD_SECRET_PART_SET_WALLET_ENCRYPTION, req, opt)
    }

    pub fn set_wallet_encryption_async(&self, req: &super::secret_part_service::SetWalletEncryptionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_part_service::SetWalletEncryptionResponse>> {
        self.set_wallet_encryption_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn issue_wallet_keys_opt(&self, req: &super::secret_part_service::IssueWalletKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secret_part_service::IssueWalletKeysResponse> {
        self.client.unary_call(&METHOD_SECRET_PART_ISSUE_WALLET_KEYS, req, opt)
    }

    pub fn issue_wallet_keys(&self, req: &super::secret_part_service::IssueWalletKeysRequest) -> ::grpcio::Result<super::secret_part_service::IssueWalletKeysResponse> {
        self.issue_wallet_keys_opt(req, ::grpcio::CallOption::default())
    }

    pub fn issue_wallet_keys_async_opt(&self, req: &super::secret_part_service::IssueWalletKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_part_service::IssueWalletKeysResponse>> {
        self.client.unary_call_async(&METHOD_SECRET_PART_ISSUE_WALLET_KEYS, req, opt)
    }

    pub fn issue_wallet_keys_async(&self, req: &super::secret_part_service::IssueWalletKeysRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_part_service::IssueWalletKeysResponse>> {
        self.issue_wallet_keys_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reissue_secret_parts_opt(&self, req: &super::secret_part_service::ReissueSecretPartsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secret_part_service::ReissueSecretPartsResponse> {
        self.client.unary_call(&METHOD_SECRET_PART_REISSUE_SECRET_PARTS, req, opt)
    }

    pub fn reissue_secret_parts(&self, req: &super::secret_part_service::ReissueSecretPartsRequest) -> ::grpcio::Result<super::secret_part_service::ReissueSecretPartsResponse> {
        self.reissue_secret_parts_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reissue_secret_parts_async_opt(&self, req: &super::secret_part_service::ReissueSecretPartsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_part_service::ReissueSecretPartsResponse>> {
        self.client.unary_call_async(&METHOD_SECRET_PART_REISSUE_SECRET_PARTS, req, opt)
    }

    pub fn reissue_secret_parts_async(&self, req: &super::secret_part_service::ReissueSecretPartsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_part_service::ReissueSecretPartsResponse>> {
        self.reissue_secret_parts_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_wallet_recovery_keys_opt(&self, req: &super::secret_part_service::CreateWalletRecoveryKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secret_part_service::CreateWalletRecoveryKeysResponse> {
        self.client.unary_call(&METHOD_SECRET_PART_CREATE_WALLET_RECOVERY_KEYS, req, opt)
    }

    pub fn create_wallet_recovery_keys(&self, req: &super::secret_part_service::CreateWalletRecoveryKeysRequest) -> ::grpcio::Result<super::secret_part_service::CreateWalletRecoveryKeysResponse> {
        self.create_wallet_recovery_keys_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_wallet_recovery_keys_async_opt(&self, req: &super::secret_part_service::CreateWalletRecoveryKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_part_service::CreateWalletRecoveryKeysResponse>> {
        self.client.unary_call_async(&METHOD_SECRET_PART_CREATE_WALLET_RECOVERY_KEYS, req, opt)
    }

    pub fn create_wallet_recovery_keys_async(&self, req: &super::secret_part_service::CreateWalletRecoveryKeysRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_part_service::CreateWalletRecoveryKeysResponse>> {
        self.create_wallet_recovery_keys_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SecretPart {
    fn put_secret_part(&mut self, ctx: ::grpcio::RpcContext, req: super::secret_part_service::PutSecretPartRequest, sink: ::grpcio::UnarySink<super::secret_part_service::PutSecretPartResponse>);
    fn set_wallet_encryption(&mut self, ctx: ::grpcio::RpcContext, req: super::secret_part_service::SetWalletEncryptionRequest, sink: ::grpcio::UnarySink<super::secret_part_service::SetWalletEncryptionResponse>);
    fn issue_wallet_keys(&mut self, ctx: ::grpcio::RpcContext, req: super::secret_part_service::IssueWalletKeysRequest, sink: ::grpcio::UnarySink<super::secret_part_service::IssueWalletKeysResponse>);
    fn reissue_secret_parts(&mut self, ctx: ::grpcio::RpcContext, req: super::secret_part_service::ReissueSecretPartsRequest, sink: ::grpcio::UnarySink<super::secret_part_service::ReissueSecretPartsResponse>);
    fn create_wallet_recovery_keys(&mut self, ctx: ::grpcio::RpcContext, req: super::secret_part_service::CreateWalletRecoveryKeysRequest, sink: ::grpcio::UnarySink<super::secret_part_service::CreateWalletRecoveryKeysResponse>);
}

pub fn create_secret_part<S: SecretPart + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECRET_PART_PUT_SECRET_PART, move |ctx, req, resp| {
        instance.put_secret_part(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECRET_PART_SET_WALLET_ENCRYPTION, move |ctx, req, resp| {
        instance.set_wallet_encryption(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECRET_PART_ISSUE_WALLET_KEYS, move |ctx, req, resp| {
        instance.issue_wallet_keys(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECRET_PART_REISSUE_SECRET_PARTS, move |ctx, req, resp| {
        instance.reissue_secret_parts(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_SECRET_PART_CREATE_WALLET_RECOVERY_KEYS, move |ctx, req, resp| {
        instance.create_wallet_recovery_keys(ctx, req, resp)
    });
    builder.build()
}
