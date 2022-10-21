use std::collections::VecDeque;

use common::{Message, SendMessage};
use parking_lot::Mutex;
use uuid::Uuid;

pub struct Messaging {
    queue: Mutex<VecDeque<Message>>,
}

impl Messaging {
    pub fn new() -> Messaging {
        Messaging {
            queue: Default::default(),
        }
    }

    pub fn put(&self, msg: SendMessage) {
        let msg = Message {
            id: Uuid::new_v4(),
            topic: msg.topic,
            msg_type: msg.msg_type,
            payload: msg.payload,
        };

        let mut queue = self.queue.lock();
        queue.push_back(msg);
    }

    pub fn query(&self, topics: Option<Vec<String>>, from_id: Option<Uuid>) -> Vec<Message> {
        let queue = self.queue.lock();
        let queue_iter = queue.iter();

        // If from_id is set start search after the id.
        // If form_id is not found assume already deleted and give everything
        let id_index = from_id
            .and_then(|id| queue.iter().position(|m| m.id == id)) // Try to find the index of from_id only if its set.
            .map(|index| index + 1) // We want to start on the index after the one we found.
            .unwrap_or(0); // If from_id was not set or from_id was not found lest go back to the beginning
        let id_iter = queue_iter.skip(id_index); // Skip the items ahead of the ID.

        // If topics is set filter out messages where topic is not in topics.
        let topic_iter = id_iter.filter(|m| {
            topics
                .as_ref()
                .map(|list| list.contains(&m.topic)) //Test if topics contains message.topic. But only if topic is set.
                .unwrap_or(true) // If topics was not sett we want to keep every message.
        });

        topic_iter.cloned().collect()
    }
}

impl Default for Messaging {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_message(topic: &str, payload: &str) -> SendMessage {
        SendMessage {
            topic: topic.to_string(),
            msg_type: "test".to_string(),
            payload: payload.to_owned(),
        }
    }

    fn assert_message_payload(msgs: &[Message], payloads: &[&str]) {
        let msg_payload: Vec<_> = msgs.iter().map(|m| m.payload.as_str()).collect();
        assert_eq!(msg_payload, payloads)
    }

    fn topics(t: &[&str]) -> Option<Vec<String>> {
        Some(t.iter().map(|&s| s.into()).collect())
    }

    #[test]
    fn query() {
        let service = Messaging::new();
        //Populate messages
        service.put(create_message("test_topic", "1"));
        service.put(create_message("test_topic", "2"));
        service.put(create_message("test_topic", "3"));

        let messages = service.query(None, None);

        assert_message_payload(&messages, &["1", "2", "3"])
    }

    #[test]
    fn query_topic() {
        let service = Messaging::new();
        //Populate messages
        service.put(create_message("topic_1", "1"));
        service.put(create_message("topic_2", "2"));
        service.put(create_message("topic_1", "3"));

        let messages = service.query(topics(&["topic_1"]), None);

        assert_message_payload(&messages, &["1", "3"])
    }

    #[test]
    fn query_topics() {
        let service = Messaging::new();
        //Populate messages
        service.put(create_message("topic_1", "1"));
        service.put(create_message("topic_2", "2"));
        service.put(create_message("topic_3", "3"));

        let messages = service.query(topics(&["topic_2", "topic_3"]), None);

        assert_message_payload(&messages, &["2", "3"])
    }

    #[test]
    fn query_id() {
        let service = Messaging::new();
        //Populate messages
        service.put(create_message("topic_1", "1"));
        service.put(create_message("topic_2", "2"));
        service.put(create_message("topic_3", "3"));
        let id_first = service.query(None, None)[0].id;

        let messages = service.query(None, Some(id_first));

        assert_message_payload(&messages, &["2", "3"])
    }

    #[test]
    fn query_id_not_found() {
        let service = Messaging::new();
        //Populate messages
        service.put(create_message("topic_1", "1"));
        service.put(create_message("topic_2", "2"));
        service.put(create_message("topic_3", "3"));
        let id_first = Uuid::new_v4();

        let messages = service.query(None, Some(id_first));

        assert_message_payload(&messages, &["1", "2", "3"])
    }

    #[test]
    fn query_topic_and_id() {
        let service = Messaging::new();
        //Populate messages
        service.put(create_message("topic_1", "1"));
        service.put(create_message("topic_2", "2"));
        service.put(create_message("topic_1", "3"));
        let id_first = service.query(None, None)[0].id;

        let messages = service.query(topics(&["topic_1"]), Some(id_first));

        assert_message_payload(&messages, &["3"])
    }
}
