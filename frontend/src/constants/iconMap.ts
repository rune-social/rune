import type Ionicons from "@expo/vector-icons/Ionicons";
import { type SymbolViewProps } from "expo-symbols";

export default {
  // See Ionicons here: https://icons.expo.fyi
  // See SF Symbols in the SF Symbols app on Mac.
  "house.fill": "home",
} as Partial<
  Record<SymbolViewProps["name"], React.ComponentProps<typeof Ionicons>["name"]>
>;
