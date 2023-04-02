import { StatusBar } from "expo-status-bar";
// import ChatListItem from "./src/components/ChatListItem";
import { StyleSheet, Text, View } from "react-native";
import Navigator from './src/navigation';

// const chat = {
//   id: "1",
//   user: {
//     image:
//       "https://notjustdev-dummy.s3.us-east-2.amazonaws.com/avatars/lukas.jpeg",
//     name: "Aaron Mathew",
//   },
//   lastMessage: {
//     text: "Oke",
//     createdAt: "07:30",
//   },
// };


export default function App() {
  
  return (
    <View style={styles.container}>
      <Navigator/>
      {/* <ChatListItem chat={chat} /> */}
      <StatusBar style='auto' />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "whitesmoke",
    // alignItems: "center",
    justifyContent: "center",
    // paddingVertical:50,
  },
});
