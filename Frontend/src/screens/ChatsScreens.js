import { View, Text, FlatList } from 'react-native';
import chats from '../../assets/data/chats.json';
import ChatListItem from '../components/ChatListItem';

const ChatsScreens = () => {
  return (
    <FlatList
        data={chats}
        renderItem={({item})=><ChatListItem chat={item}/>}
        style={{backgroundColor:'white'}}
    />
  )
}

export default ChatsScreens