import i18n from "i18next";
import { initReactI18next } from "react-i18next";

const resources = {
  zh: {
    translation: {
      "equipment.list.title": "设备台账",
      "equipment.detail.title": "设备详情",
      "equipment.control.title": "设备控制"
    }
  },
  en: {
    translation: {
      "equipment.list.title": "Equipment Ledger",
      "equipment.detail.title": "Equipment Detail",
      "equipment.control.title": "Equipment Control"
    }
  }
};

i18n.use(initReactI18next).init({
  resources,
  lng: "zh",
  interpolation: {
    escapeValue: false
  }
});

export default i18n;


