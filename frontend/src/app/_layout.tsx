import { Stack } from "expo-router";
import "@/assets/global.css";
import "@/locales";

export default function RootLayout() {
  return (
    <Stack>
      <Stack.Screen name="(tabs)" options={{ headerShown: false }} />
    </Stack>
  );
}
