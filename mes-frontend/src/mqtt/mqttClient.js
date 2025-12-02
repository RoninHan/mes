import mqtt from "mqtt";

class MqttClient {
  constructor() {
    this.client = null;
    this.handlers = {};
  }

  connect(url) {
    if (this.client) return;
    this.client = mqtt.connect(url);
    this.client.on("connect", () => {
      console.log("MQTT connected");
    });
    this.client.on("message", (topic, message) => {
      Object.values(this.handlers).forEach((h) => h(topic, message.toString()));
    });
  }

  subscribe(topic) {
    if (!this.client) return;
    this.client.subscribe(topic);
  }

  publish(topic, payload) {
    if (!this.client) return;
    this.client.publish(topic, JSON.stringify(payload));
  }

  addHandler(key, handler) {
    this.handlers[key] = handler;
  }

  removeHandler(key) {
    delete this.handlers[key];
  }
}

export const mqttClient = new MqttClient();


