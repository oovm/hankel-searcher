use std::time::{Duration, Instant};

/// Wall-clock and step limits for one improve run.
#[derive(Debug, Clone)]
pub struct SearchBudget {
    pub max_steps: usize,
    pub deadline: Option<Instant>,
}

impl SearchBudget {
    pub fn new(max_steps: usize) -> Self {
        Self { max_steps, deadline: None }
    }

    pub fn with_deadline(max_steps: usize, deadline: Instant) -> Self {
        Self { max_steps, deadline: Some(deadline) }
    }

    pub fn from_duration(max_steps: usize, time_limit: Duration) -> Self {
        Self::with_deadline(max_steps, Instant::now() + time_limit)
    }

    pub fn allows_more(&self, completed_steps: usize) -> bool {
        if completed_steps >= self.max_steps {
            return false;
        }
        if let Some(deadline) = self.deadline {
            if Instant::now() >= deadline {
                return false;
            }
        }
        true
    }
}
