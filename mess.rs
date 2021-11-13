////////////////////////////////////////////////////////////////////////////////
// Adding new messages

impl MessagingUI {
    fn add_timestamp(&mut self, ts: Timestamp) {
        ts.stamp(&mut self.msg_area);
        self.last_activity_ts = Some(ts);
    }

    pub(crate) fn show_topic(&mut self, topic: &str, ts: Timestamp) {
        self.add_timestamp(ts);

        self.msg_area.add_text(topic, SegStyle::Topic);

        self.msg_area.flush_line();
    }

    pub(crate) fn add_client_err_msg(&mut self, msg: &str) {
        self.reset_activity_line();

        self.msg_area.add_text(msg, SegStyle::ErrMsg);
        self.msg_area.flush_line();
    }
}
