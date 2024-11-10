use chrono::{DateTime, Duration as ChronoDuration, Utc};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    pub title: String,
    pub body: String,
    pub toast_type: ToastType,
    pub timeout: Option<ChronoDuration>,
    pub id: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ToastType {
    Info,
    Success,
    Warning,
    Error,
}

impl Default for ToastType {
    fn default() -> Self {
        ToastType::Info
    }
}

#[derive(Debug, Clone, Default)]
pub struct ToastManager {
    pub toasts: HashMap<usize, Toast>,
    pub timeouts: BinaryHeap<Reverse<DateTime<Utc>>>,
    next_id: usize,
}

// TODO: Improve API
impl ToastManager {
    pub fn add_toast(
        &mut self,
        title: String,
        body: String,
        toast_type: ToastType,
        timeout: Option<ChronoDuration>,
    ) -> &mut Self {
        let id = self.next_id;
        self.next_id += 1;

        let expiration = timeout.map(|d| Utc::now() + d);
        if let Some(expiration_time) = expiration {
            self.timeouts.push(Reverse(expiration_time));
        }

        self.toasts.insert(
            id,
            Toast {
                title,
                body,
                toast_type,
                timeout,
                id,
            },
        );
        self
    }

    pub fn remove_toast(&mut self, id: usize) {
        self.toasts.remove(&id);
    }

    pub fn cleanup_expired(&mut self) {
        let now = Utc::now();
        let mut ids_to_remove = Vec::new();

        while let Some(&Reverse(expiration)) = self.timeouts.peek() {
            if expiration <= now {
                self.timeouts.pop();

                for (id, toast) in self.toasts.iter() {
                    if let Some(timeout) = toast.timeout {
                        if expiration.signed_duration_since(now) <= timeout {
                            ids_to_remove.push(id.clone());
                        }
                    }
                }
            } else {
                break;
            }
        }

        for id in ids_to_remove {
            self.remove_toast(id);
        }
    }
}
