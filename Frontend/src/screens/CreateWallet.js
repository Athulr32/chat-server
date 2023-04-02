import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  TextInput,
} from "react-native";
import { Route, useRoute } from "@react-navigation/native";
import React from "react";
import { useNavigation } from "@react-navigation/native";
import { useEffect } from "react";

const CreateWallet = () => {
  const route = useRoute();
  const navigation = useNavigation();

  const [text, onChangeText] = React.useState("");
    const bip39 = require('bip39')

  useEffect(() => {
    navigation.setOptions({ title: route.params.name });
  }, [route.params.name]);

//   const mnemonic = bip39.generateMnemonic()
//   console.log(mnemonic)
  const val1 = 0;
  const val2 = 0;
  const val3 = 0;
  const val4 = 0;
  const val5 = 0;
  const val6 = 0;
  const val7 = 0;
  const val8 = 0;
  const val9 = 0;
  const val10 = 0;
  const val11 = 0;
  const val12 = 0;

  return (
    <View style={{ height: "100%" }}>
      {route.params.do && (
        <View style={{ width: "100%" }}>
          <View
            style={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              justifyContent: "center",
              height: "100%",
              width: "100%",
            }}
          >
            <View
              style={{
                display: "flex",
                flexDirection: "row",
                justifyContent: "space-between",
                alignItems: "stretch",
              }}
            >
              <Text style={styles.text}>1. {val1}</Text>
              <Text style={styles.text}>2. {val2}</Text>
            </View>
            <View style={{ display: "flex", flexDirection: "row" }}>
              <Text style={styles.text}>3. {val3}</Text>
              <Text style={styles.text}>4. {val4}</Text>
            </View>
            <View style={{ display: "flex", flexDirection: "row" }}>
              <Text style={styles.text}>5. {val5}</Text>
              <Text style={styles.text}>6. {val6}</Text>
            </View>
            <View style={{ display: "flex", flexDirection: "row" }}>
              <Text style={styles.text}>7. {val7}</Text>
              <Text style={styles.text}>8. {val8}</Text>
            </View>
            <View style={{ display: "flex", flexDirection: "row" }}>
              <Text style={styles.text}>9. {val9}</Text>
              <Text style={styles.text}>10. {val10}</Text>
            </View>
            <View style={{ display: "flex", flexDirection: "row" }}>
              <Text style={styles.text}>11. {val11}</Text>
              <Text style={styles.text}>12. {val12}</Text>
            </View>
            <Text style={styles.text}>
              Never share your secret phrase with anyone, store it securely!
            </Text>
          </View>
        </View>
      )}
      {!route.params.do && (
        <View
          style={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "space-between",
            height: "100%",
          }}
        >
          <View>
            <View
            style={{
                height: 160,
                padding: 20,
                backgroundColor: "white",
                marginTop: 40,
                margin: 19,
                borderRadius: 10,
                paddingTop:20,
            }}>
              <TextInput
                multiline
                editable
                placeholder='Secret Phrase'
                style={styles.input}
                onChangeText={onChangeText}
                value={text}
              />
            </View>

            <Text style={{ color: "gray", alignSelf: "center" }}>
              Your Secret Phrase would consist of typically 12 words separated
              by single spaces{" "}
            </Text>
          </View>
          <View style={{ marginBottom: 30 }}>
            <TouchableOpacity
              style={styles.button}
              onPress={() => navigation.navigate("Home", {})}
            >
              <Text style={{ padding: 25, fontSize: 17, color: "white" }}>
                Move on
              </Text>
            </TouchableOpacity>
            <Text style={{ fontWeight: 300, alignSelf: "center" }}>
              What is Secret Phrase
            </Text>
          </View>
        </View>
      )}
    </View>
  );
};

const styles = StyleSheet.create({
  button: {
    position: "relative",
    alignItems: "center",
    justifyContent: "center",
    paddingVertical: 1,
    paddingHorizontal: 20,
    borderRadius: 10,
    marginBottom: 17,
    margin: 30,
    bottom: 0,
    borderColor: "#608BFB",
    borderWidth: 0,
    backgroundColor: "#3A71FF",
  },
  input: {
   

  },
  text: {
    fontSize: 15,
    color: "black",
    padding: 30,
    paddingHorizontal: 60,
    fontWeight: "200",
    alignSelf: "center",
  },
});

export default CreateWallet;
