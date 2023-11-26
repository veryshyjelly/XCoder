import { Box } from "@mantine/core";
import "./App.css";
import LandingPage from "./LandingPage";
import TitleBar from "./Titlebar";

function App() {
  return (
    <Box className="border border-[#3c3f41]" style={{height: window.outerHeight}}>
      <TitleBar/>
      <LandingPage/>
    </Box>
  );
}

export default App;
