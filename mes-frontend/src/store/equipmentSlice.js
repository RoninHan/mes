import { createSlice } from "@reduxjs/toolkit";

const initialState = {
  list: [],
  current: null,
  statusLog: [],
  mqttConnected: false,
  realtimeStatus: {}
};

const equipmentSlice = createSlice({
  name: "equipment",
  initialState,
  reducers: {
    setEquipmentList(state, action) {
      state.list = action.payload;
    },
    setCurrentEquipment(state, action) {
      state.current = action.payload;
    },
    setStatusLog(state, action) {
      state.statusLog = action.payload;
    },
    setMqttConnected(state, action) {
      state.mqttConnected = action.payload;
    },
    upsertRealtimeStatus(state, action) {
      const { equipmentId, data } = action.payload;
      state.realtimeStatus[equipmentId] = data;
    }
  }
});

export const {
  setEquipmentList,
  setCurrentEquipment,
  setStatusLog,
  setMqttConnected,
  upsertRealtimeStatus
} = equipmentSlice.actions;

export default equipmentSlice.reducer;


