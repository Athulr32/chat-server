import { View, Text } from "react-native";
import { createNativeStackNavigator } from "@react-navigation/native-stack";
import { NavigationContainer } from "@react-navigation/native";
import ChatScreen from "../screens/ChatScreen";
import ChatsScreens from "../screens/ChatsScreens";
import { useState, useEffect } from "react";
import AsyncStorage from "@react-native-async-storage/async-storage";
import OnboardingScreens from "../screens/OnboardingScreens";
import MainTabNavigator from "./MainTabNavigator";

const Stack = createNativeStackNavigator();

const index = () => {
  const [isFirstLaunch, setIsFirstLaunch] = useState(false);

  useEffect(() => {
    AsyncStorage.getItem("alreadyLaunched").then((value) => {
      if (value === null) {
        AsyncStorage.setItem("alreadyLaunched", "true");
        setIsFirstLaunch(true);
      } else {
        setIsFirstLaunch(false);
      }
    });
  }, []);

  return (
    <NavigationContainer>
      <Stack.Navigator
      // screenOptions={{ headerStyle: { backgroundColor: "whitesmoke" } }}
      >
        {!isFirstLaunch && (
          <Stack.Screen
            options={{ headerShown: false }}
            name='Onboarding'
            component={OnboardingScreens}
          />
        )}
        <Stack.Screen
          name='Home'
          component={MainTabNavigator}
          options={{ headerShown: false }}
        />
        <Stack.Screen name='Chat' component={ChatScreen} />
      </Stack.Navigator>
    </NavigationContainer>
  );
};

export default index;
