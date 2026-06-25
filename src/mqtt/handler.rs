use rumqttc::Publish;

pub trait MqttPublishHandler {
    fn handle(&mut self, publish: Publish);
}
