import {appWindow, LogicalSize} from "@tauri-apps/api/window";
import {useEffect} from "react";

const MainPage = () => {
    useEffect(() => {
        appWindow.setTitle("XCoder");
        appWindow.setSize(new LogicalSize(1080, 720));
        appWindow.maximize();
    }, [])

    return (<>
        Main Page
    </>)
}

export default MainPage;