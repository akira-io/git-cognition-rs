use crate::{ManagedCodeReviewProvider, ManagedIssueProvider, ManagedReleaseProvider};

use super::{
    GitLabCodeReview, GitLabCodeReviewCollection, GitLabIssue, GitLabIssueCollection,
    GitLabProvider, GitLabRelease, GitLabReleaseCollection,
};

impl ManagedIssueProvider for GitLabProvider {
    fn issue_url(&self, issue: &crate::Issue) -> crate::RequestUrl {
        GitLabIssue::make(self.api_base_url(), issue.clone()).url()
    }

    fn issue_list_url(&self, query: &crate::IssueListQuery) -> crate::RequestUrl {
        GitLabIssueCollection::make(self.api_base_url()).list(query)
    }

    fn issue_create_request(&self, draft: &crate::IssueDraft) -> crate::Request {
        GitLabIssueCollection::make(self.api_base_url()).create(draft)
    }

    fn issue_update_request(&self, patch: &crate::IssuePatch) -> crate::Request {
        GitLabIssue::make(self.api_base_url(), patch.issue().clone()).update(patch)
    }

    fn issue_delete_request(&self, issue: &crate::Issue) -> crate::CognitionResult<crate::Request> {
        Ok(GitLabIssue::make(self.api_base_url(), issue.clone()).delete())
    }
}

impl ManagedCodeReviewProvider for GitLabProvider {
    fn code_review_url(&self, code_review: &crate::CodeReview) -> crate::RequestUrl {
        GitLabCodeReview::make(self.api_base_url(), code_review.clone()).url()
    }

    fn code_review_list_url(&self, query: &crate::CodeReviewListQuery) -> crate::RequestUrl {
        GitLabCodeReviewCollection::make(self.api_base_url()).list(query)
    }

    fn code_review_create_request(&self, draft: &crate::CodeReviewDraft) -> crate::Request {
        GitLabCodeReviewCollection::make(self.api_base_url()).create(draft)
    }

    fn code_review_update_request(&self, patch: &crate::CodeReviewPatch) -> crate::Request {
        GitLabCodeReview::make(self.api_base_url(), patch.code_review().clone()).update(patch)
    }

    fn code_review_merge_request(&self, code_review: &crate::CodeReview) -> crate::Request {
        GitLabCodeReview::make(self.api_base_url(), code_review.clone()).merge()
    }

    fn code_review_close_request(&self, code_review: &crate::CodeReview) -> crate::Request {
        let close_patch = code_review.patch().closed().get();

        GitLabCodeReview::make(self.api_base_url(), code_review.clone()).update(&close_patch)
    }

    fn code_review_delete_request(
        &self,
        code_review: &crate::CodeReview,
    ) -> crate::CognitionResult<crate::Request> {
        Ok(GitLabCodeReview::make(self.api_base_url(), code_review.clone()).delete())
    }
}

impl ManagedReleaseProvider for GitLabProvider {
    fn release_url(&self, release: &crate::Release) -> crate::RequestUrl {
        GitLabRelease::make(self.api_base_url(), release.clone()).url()
    }

    fn release_list_url(&self, query: &crate::ReleaseListQuery) -> crate::RequestUrl {
        GitLabReleaseCollection::make(self.api_base_url()).list(query)
    }

    fn release_create_request(&self, draft: &crate::ReleaseDraft) -> crate::Request {
        GitLabReleaseCollection::make(self.api_base_url()).create(draft)
    }

    fn release_update_request(&self, patch: &crate::ReleasePatch) -> crate::Request {
        GitLabRelease::make(self.api_base_url(), patch.release().clone()).update(patch)
    }

    fn release_delete_request(&self, release: &crate::Release) -> crate::Request {
        GitLabRelease::make(self.api_base_url(), release.clone()).delete()
    }
}
