import { View, Text } from 'react-native'
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs'
import About from '../screens/About';
import ChatsScreens from '../screens/ChatsScreens';
import {Ionicons} from '@expo/vector-icons';

const Tab= createBottomTabNavigator();

const MainTabNavigator = () => {
  return (
    <Tab.Navigator initialRouteName='Chats' 
    screenOptions={{
      tabBarStyle: { backgroundColor: "whitesmoke" },
      headerStyle: { backgroundColor: "whitesmoke" },
    }}
  >
      <Tab.Screen name="About" component={About} options={{tabBarIcon:({color,size})=>(<Ionicons name="logo-whatsapp" size={size} color={color} />),}}/>
      <Tab.Screen name="Chats" component={ChatsScreens} options={{tabBarIcon:({color,size})=>(<Ionicons name="ios-chatbubbles-sharp" size={size} color={color} />),}}/>
      <Tab.Screen name="Settings" component={About} options={{tabBarIcon:({color,size})=>(<Ionicons name="settings-outline" size={size} color={color} />),}}/>
    </Tab.Navigator>
  )
}

export default MainTabNavigator