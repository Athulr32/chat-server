import Onboarding from 'react-native-onboarding-swiper'
import {useNavigation} from '@react-navigation/native'
import Lottie from "lottie-react-native";
const OnboardingScreen = () => {

  const navigation= useNavigation();

  return (
<Onboarding
onSkip={()=> navigation.replace("Home")}
onDone={()=> navigation.replace("Home")}
bottomBarColor='white'
  pages={[
    {
      backgroundColor: '#fff',
      image: <Lottie source={require('../../lottie/68866-plant-moving.json')} autoPlay loop style={{width:300 , marginTop:-40, marginBottom:-50}} />,
      title: 'About Growth',
      subtitle: 'Done with React Native Onboarding Swiper',
    },
    {
      backgroundColor: '#fff',
      image: <Lottie source={require('../../lottie/9656-onboarding-page-1.json')} autoPlay loop style={{width:300 , marginTop:-40, marginBottom:-50}} />,
      title: 'Community',
      subtitle: 'Done with React Native Onboarding Swiper',
    },
    {
      backgroundColor: '#fff',
      image: <Lottie source={require('../../lottie/121906-preparing-screener.json')} autoPlay loop style={{width:300 , marginTop:-40, marginBottom:-50}} />,
      title: 'Onboarding',
      subtitle: 'Done with React Native Onboarding Swiper',
    },
  ]}
/>
  )
}

export default OnboardingScreen