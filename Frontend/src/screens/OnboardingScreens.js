import Onboarding from "react-native-onboarding-swiper";
import { useNavigation } from "@react-navigation/native";
import Lottie from "lottie-react-native";
const OnboardingScreen = () => {
  const navigation = useNavigation();

  return (
    <Onboarding
      onSkip={() => navigation.replace("Getin")}
      onDone={() => navigation.replace("Getin")}
      bottomBarColor='white'
      pages={[
        {
          backgroundColor: "#fff",
          image: (
            <Lottie
              source={require("../../lottie/54002-lets-chat.json")}
              autoPlay
              loop
              style={{ width: 500, marginTop: -70, marginBottom: -220 }}
            />
          ),
          title: "Always stay in Touch",
          subtitle: "Done with React Native Onboarding Swiper",
        },
        {
          backgroundColor: "#fff",
          image: (
            <Lottie
              source={require("../../lottie/9656-onboarding-page-1.json")}
              autoPlay
              loop
              style={{ width: 300, marginTop: -40, marginBottom: -50 }}
            />
          ),
          title: "Community",
          subtitle: "Done with React Native Onboarding Swiper",
        },
        {
          backgroundColor: "#fff",
          image: (
            <Lottie
              source={require("../../lottie/94138-lock.json")}
              autoPlay
              loop
              style={{ width: 300, marginTop: -40, marginBottom: -50 }}
            />
          ),
          title: "Onboarding",
          subtitle: "Done with React Native Onboarding Swiper",
        },
      ]}
    />
  );
};

export default OnboardingScreen;
