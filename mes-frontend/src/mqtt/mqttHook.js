import { useEffect } from "react";
import { useDispatch } from "react-redux";
import { mqttClient } from "./mqttClient";
import { setMqttConnected, upsertRealtimeStatus } from "../store/equipmentSlice";

export function useMqtt(equipmentId, topic) {
  const dispatch = useDispatch();

  useEffect(() => {
    const url = import.meta.env.VITE_MQTT_BROKER || "ws://localhost:8083/mqtt";
    mqttClient.connect(url);
    mqttClient.subscribe(topic);
    dispatch(setMqttConnected(true));

    const key = `eq-${equipmentId}`;
    mqttClient.addHandler(key, (recvTopic, msg) => {
      if (recvTopic === topic) {
        try {
          const data = JSON.parse(msg);
          dispatch(upsertRealtimeStatus({ equipmentId, data }));
        } catch {
          // ignore parse error
        }
      }
    });

    return () => {
      mqttClient.removeHandler(key);
    };
  }, [equipmentId, topic, dispatch]);
}


