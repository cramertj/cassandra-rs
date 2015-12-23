use cql_bindgen::cass_retry_policy_default_new;
use cql_bindgen::cass_retry_policy_downgrading_consistency_new;
use cql_bindgen::cass_retry_policy_fallthrough_new;
use cql_bindgen::cass_retry_policy_logging_new;
use cql_bindgen::cass_retry_policy_free;

use cql_bindgen::CassRetryPolicy as _CassRetryPolicy;

pub struct RetryPolicy(pub *mut _CassRetryPolicy);

impl RetryPolicy {
    pub fn default_new() -> Self { unsafe { RetryPolicy(cass_retry_policy_default_new()) } }

    pub fn downgrading_consistency_new() -> Self {
        unsafe { RetryPolicy(cass_retry_policy_downgrading_consistency_new()) }
    }

    pub fn fallthrough_new() -> Self { unsafe { RetryPolicy(cass_retry_policy_fallthrough_new()) } }

    pub fn logging_new(child_retry_policy: RetryPolicy) -> Self {
        unsafe { RetryPolicy(cass_retry_policy_logging_new(child_retry_policy.0)) }
    }
}

impl Drop for RetryPolicy {
    fn drop(&mut self) {
        unsafe {
            cass_retry_policy_free(self.0);
        }
    }
}
