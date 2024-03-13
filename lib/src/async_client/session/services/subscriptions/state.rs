use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::client::prelude::{
    encoding::DecodingOptions, NotificationMessage, SubscriptionAcknowledgement,
};

use super::{CreateMonitoredItem, ModifyMonitoredItem, Subscription};

pub struct SubscriptionState {
    subscriptions: HashMap<u32, Subscription>,
    last_publish: Instant,
    min_publish_interval: Duration,
    acknowledgements: Vec<SubscriptionAcknowledgement>,
}

impl SubscriptionState {
    pub fn new(min_publish_interval: Duration) -> Self {
        Self {
            subscriptions: HashMap::new(),
            last_publish: Instant::now() - min_publish_interval,
            min_publish_interval,
            acknowledgements: Vec::new(),
        }
    }

    pub fn next_publish_time(&mut self) -> Option<Instant> {
        let r = self
            .subscriptions
            .values()
            .map(|s| s.next_publish)
            .min()
            .map(|i| i.max(self.last_publish + self.min_publish_interval));
        for sub in self.subscriptions.values_mut() {
            sub.notify_is_publishing_now();
        }
        r
    }

    pub fn take_acknowledgements(&mut self) -> Vec<SubscriptionAcknowledgement> {
        std::mem::take(&mut self.acknowledgements)
    }

    pub fn subscription_ids(&self) -> Option<Vec<u32>> {
        if self.subscriptions.is_empty() {
            None
        } else {
            Some(self.subscriptions.keys().cloned().collect())
        }
    }

    pub fn subscription_exists(&self, subscription_id: u32) -> bool {
        self.subscriptions.contains_key(&subscription_id)
    }

    pub fn get(&self, subscription_id: u32) -> Option<&Subscription> {
        self.subscriptions.get(&subscription_id)
    }

    pub(crate) fn add_subscription(&mut self, subscription: Subscription) {
        self.subscriptions
            .insert(subscription.subscription_id(), subscription);
    }

    pub(crate) fn modify_subscription(
        &mut self,
        subscription_id: u32,
        publishing_interval: Duration,
        lifetime_count: u32,
        max_keep_alive_count: u32,
        max_notifications_per_publish: u32,
        priority: u8,
    ) {
        if let Some(ref mut subscription) = self.subscriptions.get_mut(&subscription_id) {
            subscription.set_publishing_interval(publishing_interval);
            subscription.set_lifetime_count(lifetime_count);
            subscription.set_max_keep_alive_count(max_keep_alive_count);
            subscription.set_max_notifications_per_publish(max_notifications_per_publish);
            subscription.set_priority(priority);
        }
    }

    pub(crate) fn delete_subscription(&mut self, subscription_id: u32) -> Option<Subscription> {
        let subscription = self.subscriptions.remove(&subscription_id);
        subscription
    }

    pub(crate) fn set_publishing_mode(
        &mut self,
        subscription_ids: &[u32],
        publishing_enabled: bool,
    ) {
        subscription_ids.iter().for_each(|subscription_id| {
            if let Some(ref mut subscription) = self.subscriptions.get_mut(subscription_id) {
                subscription.set_publishing_enabled(publishing_enabled);
            }
        });
    }

    pub(crate) fn insert_monitored_items(
        &mut self,
        subscription_id: u32,
        items_to_create: Vec<CreateMonitoredItem>,
    ) {
        if let Some(ref mut subscription) = self.subscriptions.get_mut(&subscription_id) {
            subscription.insert_monitored_items(items_to_create);
        }
    }

    pub(crate) fn modify_monitored_items(
        &mut self,
        subscription_id: u32,
        items_to_modify: &[ModifyMonitoredItem],
    ) {
        if let Some(ref mut subscription) = self.subscriptions.get_mut(&subscription_id) {
            subscription.modify_monitored_items(items_to_modify);
        }
    }

    pub(crate) fn delete_monitored_items(&mut self, subscription_id: u32, items_to_delete: &[u32]) {
        if let Some(ref mut subscription) = self.subscriptions.get_mut(&subscription_id) {
            subscription.delete_monitored_items(items_to_delete);
        }
    }

    pub(crate) fn set_triggering(
        &mut self,
        subscription_id: u32,
        triggering_item_id: u32,
        links_to_add: &[u32],
        links_to_remove: &[u32],
    ) {
        if let Some(ref mut subscription) = self.subscriptions.get_mut(&subscription_id) {
            subscription.set_triggering(triggering_item_id, links_to_add, links_to_remove);
        }
    }

    pub(crate) fn handle_notification(
        &mut self,
        subscription_id: u32,
        notification: NotificationMessage,
        decoding_options: &DecodingOptions,
    ) {
        if let Some(sub) = self.subscriptions.get_mut(&subscription_id) {
            sub.on_notification(notification, decoding_options);
        }
    }
}
