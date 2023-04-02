import { View, Text, StyleSheet, TextInput } from 'react-native'
import {AntDesign, MaterialIcons} from '@expo/vector-icons';
import { useState } from 'react';
import { SafeAreaView } from 'react-native-safe-area-context';

const index = () => {

    const [newMessage,setNewMessage] = useState('');

    const onSend=() =>{
        console.warn("sending");
        setNewMessage('');
    }

  return (
    <SafeAreaView edges={['bottom']} style={styles.container}>
    <AntDesign name='plus' size={20} color="royalblue"/>
    
    <TextInput style={styles.input} onChangeText={setNewMessage} value={newMessage} placeholder="Type your message..."/>
    <MaterialIcons onPress={onSend} style={styles.send} name="send" size={16} color="white"/>
    </SafeAreaView>
  )
}

const styles = StyleSheet.create({
    container:{
        flexDirection:'row',
        backgroundColor: 'whitesmoke',
        padding:5,
        paddingHorizontal:10,
        alignItems:'center',
    },
    input: {
        flex:1,
        backgroundColor: 'white',
        padding:5,
        borderRadius:50,
        marginHorizontal:10,
        paddingHorizontal:15,
        borderColor:'lightgray',
        borderWidth: StyleSheet.hairlineWidth,

    },
    send:{
        backgroundColor:'royalblue',
        padding:7,
        borderRadius:15,
        overflow:'hidden',
    },
});

export default index