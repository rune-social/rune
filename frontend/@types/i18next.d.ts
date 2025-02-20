import type translation from "@/locales/en-US/translation.json";
import "i18next";

declare module "i18next" {
  interface CustomTypeOptions {
    resources: {
      translation: typeof translation;
    };
  }
}
