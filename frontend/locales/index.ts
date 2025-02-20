import translationEN from "@/locales/en/translation.json";
import translationKO from "@/locales/ko/translation.json";
import languageDetector from "@/locales/languageDetector";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";

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
