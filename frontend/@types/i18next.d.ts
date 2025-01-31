import "i18next";
import type translation from "@/locales/en-US/translation.json";

declare module "i18next" {
  interface CustomTypeOptions {
    resources: {
      translation: typeof translation;
    };
  }
}
