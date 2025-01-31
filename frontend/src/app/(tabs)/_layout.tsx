import { IconSymbol } from "@/src/components/IconSymbol";
import { Tabs } from "expo-router";
import { useTranslation } from "react-i18next";

export default function TabLayout() {
  const { t } = useTranslation();

  return (
    <Tabs>
      <Tabs.Screen
        name="index"
        options={{
          title: t("main_menus.home"),
          tabBarIcon: ({ color }) => (
            <IconSymbol name="house.fill" size={32} color={color} />
          ),
        }}
      />
      <Tabs.Screen
        name="more"
        options={{
          title: t("main_menus.more"),
          tabBarIcon: ({ color }) => (
            <IconSymbol name="ellipsis.circle" size={32} color={color} />
          ),
        }}
      />
    </Tabs>
  );
}
