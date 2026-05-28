use crate::ManagedPipelineProvider;

use super::{GitLabPipeline, GitLabPipelineCollection, GitLabProvider};

impl ManagedPipelineProvider for GitLabProvider {
    fn pipeline_url(&self, pipeline: &crate::Pipeline) -> crate::RequestUrl {
        GitLabPipeline::make(self.api_base_url(), pipeline.clone()).url()
    }

    fn pipeline_list_url(&self, query: &crate::PipelineListQuery) -> crate::RequestUrl {
        GitLabPipelineCollection::make(self.api_base_url()).list(query)
    }

    fn pipeline_rerun_request(
        &self,
        pipeline: &crate::Pipeline,
    ) -> crate::CognitionResult<crate::Request> {
        Ok(GitLabPipeline::make(self.api_base_url(), pipeline.clone()).rerun())
    }

    fn pipeline_cancel_request(
        &self,
        pipeline: &crate::Pipeline,
    ) -> crate::CognitionResult<crate::Request> {
        Ok(GitLabPipeline::make(self.api_base_url(), pipeline.clone()).cancel())
    }
}
