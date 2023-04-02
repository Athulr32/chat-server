import {
  View,
  Text,
  ImageBackground,
  StyleSheet,
  FlatList,
  KeyboardAvoidingView,
} from "react-native";
import bg from "../../assets/images/BG.png";
import { useRoute, useNavigation } from "@react-navigation/native";
import Message from "../components/Message";
import { useEffect } from "react";
import InputBox from "../components/InputBox";
import messages from "../../assets/data/messages.json";

const ChatScreen = () => {
  const route = useRoute();
  const navigation = useNavigation();
  console.log(route);

  useEffect(() => {
    navigation.setOptions({ title: route.params.name });
  }, [route.params.name]);

  return (
    <KeyboardAvoidingView
      behavior={Platform.OS === "ios" ? "padding" : "height"}
      keyboardVerticalOffset={Platform.OS === "ios" ? 60 : 90}
      style={styles.bg}
    >
      <ImageBackground source={bg} style={styles.bg}>
        <FlatList
          data={messages}
          renderItem={({ item }) => <Message message={item} />}
          style={styles.list}
          inverted
        />
        <InputBox />
      </ImageBackground>
    </KeyboardAvoidingView>
  );
};

const styles = StyleSheet.create({
  bg: {
    flex: 1,
  },
});

export default ChatScreen;
