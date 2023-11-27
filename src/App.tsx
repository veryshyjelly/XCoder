import {Box} from "@mantine/core";
import "./App.css";
import LandingPage from "./LandingPage";
import TitleBar from "./Titlebar";
import MainPage from "./MainPage";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api";

function App() {
    const [directory, setDirectory] = useState('');

    useEffect(() => {
        invoke("get_directory").then((dir) => {
            setDirectory(dir as string);
        }).catch(e => console.error(e));
    })

    return (
        <Box className="border border-[#3c3f41]" style={{height: "100%", width: "100%", position: "fixed"}}>
            <TitleBar/>
            {directory !== '' && <MainPage/>}
            {directory === '' && <LandingPage setDirectory={setDirectory}/>}
        </Box>
    );
}

export default App;