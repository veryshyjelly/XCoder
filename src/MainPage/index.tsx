import {appWindow, LogicalSize} from "@tauri-apps/api/window";
import {useEffect} from "react";
import {Button} from "@mantine/core";
import {invoke} from "@tauri-apps/api";

const MainPage = ({setDirectory}: { setDirectory: React.Dispatch<React.SetStateAction<string>> }) => {
    useEffect(() => {
        appWindow.setTitle("XCoder");
        appWindow.setSize(new LogicalSize(1080, 720));
        appWindow.maximize();
    }, [])

    return (<>
        <Button onClick={() => {
            invoke("set_directory", {directory: ""}).then(() => {
                setDirectory("");
            }).catch(e => console.error(e));
        }}>Close Project</Button>
    </>)
}

export default MainPage;