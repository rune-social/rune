import i18n from "i18next";
import translationEN from "@/locales/en/translation.json";
import translationKO from "@/locales/ko/translation.json";
import { initReactI18next } from "react-i18next";
import languageDetector from "@/locales/languageDetector";

const resources = {
  en: { translation: translationEN },
  ko: { translation: translationKO },
};

async function initI18n() {
  i18n
    .use(languageDetector)
    .use(initReactI18next)
    .init({
      resources,
      fallbackLng: "en",
      interpolation: {
        escapeValue: false,
      },
    });
}

initI18n();

export default i18n;
