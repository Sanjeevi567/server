use aws_apis::{PollyOps, RekognitionOps, S3Ops, SdkConfig, TranscribeOps};
pub struct SharedCredAndOps {
    s3_ops: S3Ops,
    rekognition_ops: RekognitionOps,
    polly_ops: PollyOps,
    transcribe_ops: TranscribeOps,
}
impl SharedCredAndOps {
    pub fn initialize_cred_and_ops(config: SdkConfig) -> Self {
        Self {
            s3_ops: S3Ops::build(config.clone()),
            polly_ops: PollyOps::build(config.clone()),
            rekognition_ops: RekognitionOps::build(config.clone()),
            transcribe_ops: TranscribeOps::build(config),
        }
    }
    pub fn s3_ops(&self) -> &S3Ops {
        &self.s3_ops
    }
    pub fn rekognition_ops(&self) -> &RekognitionOps {
        &self.rekognition_ops
    }
    pub fn polly_ops(&self) -> &PollyOps {
        &self.polly_ops
    }
    pub fn transcribe_ops(&self) -> &TranscribeOps {
        &self.transcribe_ops
    }
}
