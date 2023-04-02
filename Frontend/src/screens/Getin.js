import { View, Text, Image, Button, StyleSheet, TouchableOpacity } from "react-native";
import { useRoute, useNavigation } from "@react-navigation/native";
import React from "react";

const Getin = () => {

  const navigation = useNavigation();

  return (
    <View
      style={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        height: "100%",
        marginTop:60,
      }}
    >
      <Image source={require("../../assets/images/logo.png")} />
      <View style={{marginTop:170}}>
        <TouchableOpacity  style={styles.button}  onPress={()=> navigation.navigate('CreateWallet',{do:true, name:"Create Wallet"})} >
            <Text style={styles.text}>Create Wallet</Text>
            </TouchableOpacity>
            <TouchableOpacity  style={styles.button}  onPress={()=> navigation.navigate('CreateWallet',{do:false, name:"Wallet"})} >
            <Text style={styles.text}>I Already have a Wallet</Text>
            </TouchableOpacity>
      </View>
    </View>
  );

  
};

const styles = StyleSheet.create({
    button: {
      alignItems: 'center',
      justifyContent: 'center',
      paddingVertical: 18,
      paddingHorizontal: 22,
      borderRadius: 10,
      marginBottom:17,
    //   elevation: 3,
      borderColor:'#608BFB',

      borderWidth:0,
      backgroundColor: 'white',
    },
    text: {
        fontSize: 15,
        color: "black",
        fontWeight: "200",
        alignSelf: "center",
        // textTransform: "uppercase"
      }
  });

export default Getin;
