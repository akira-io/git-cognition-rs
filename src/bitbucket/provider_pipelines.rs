use crate::ManagedPipelineProvider;

use super::{BitbucketPipeline, BitbucketPipelineCollection, BitbucketProvider};

impl ManagedPipelineProvider for BitbucketProvider {
    fn pipeline_url(&self, pipeline: &crate::Pipeline) -> crate::RequestUrl {
        BitbucketPipeline::make(self.api_base_url(), pipeline.clone()).url()
    }

    fn pipeline_list_url(&self, query: &crate::PipelineListQuery) -> crate::RequestUrl {
        BitbucketPipelineCollection::make(self.api_base_url()).list(query)
    }

    fn pipeline_rerun_request(
        &self,
        _pipeline: &crate::Pipeline,
    ) -> crate::CognitionResult<crate::Request> {
        Err(crate::error().invalid_input(
            "bitbucket pipeline rerun is not exposed by a validated pipeline endpoint",
        ))
    }

    fn pipeline_cancel_request(
        &self,
        pipeline: &crate::Pipeline,
    ) -> crate::CognitionResult<crate::Request> {
        Ok(BitbucketPipeline::make(self.api_base_url(), pipeline.clone()).cancel())
    }
}
