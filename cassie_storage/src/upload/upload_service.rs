use crate::upload::oss_service::OssService;
use async_trait::async_trait;
use axum::body::Bytes;
use cassie_common::error::Result;
use cassie_config::config::{ApplicationConfig, UploadType};

/**
 * @description:  IUploadService  upload base trait
 * @author String
 * @date 2022/3/25 15:54
 * @email 348040933@qq.com
 */
#[async_trait]
pub trait IUploadService: Sync + Send {
    async fn upload(&self, data: Bytes, file_name: String, content_type: String) -> Result<String>;
}

pub struct UploadService {
    pub inner: Box<dyn IUploadService>,
}

impl UploadService {
    //创建上传服务默认实现oss
    pub fn new(config: &ApplicationConfig) -> cassie_common::error::Result<Self> {
        match config.upload_type() {
            UploadType::OSS => {
                config.oss().validate();
                Ok(Self {
                    inner: Box::new(OssService::new(
                        config.oss().key_id().clone(),
                        config.oss().key_secret().clone(),
                        config.oss().endpoint().clone(),
                        config.oss().bucket().clone(),
                        config.oss().access_endpoint().clone(),
                    )),
                })
            }
            _ => {
                panic!("unknown of upload_type:,current support 'oss' ");
            }
        }
    }
    pub async fn upload(
        &self,
        data: Bytes,
        file_name: String,
        content_type: String,
    ) -> Result<String> {
        self.inner.upload(data, file_name, content_type).await
    }
}
