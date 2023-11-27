import {Center, Flex, Image} from "@mantine/core";
import {appWindow, getCurrent} from "@tauri-apps/api/window";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api";
import {TauriEvent} from "@tauri-apps/api/event";

const TitleBar = () => {
    const [isFocused, setIsFocused] = useState(true);

    useEffect(() => {
        getCurrent().listen(TauriEvent.WINDOW_CLOSE_REQUESTED, () => {
            invoke("save_state").then(() => {
            }).catch(e => console.error(e));
        });
        window.addEventListener("focus", () => setIsFocused(true));
        window.addEventListener("blur", () => setIsFocused(false));
    }, [])


    return (
        <Flex data-tauri-drag-region className="justify-end h-9"
              style={{backgroundColor: isFocused ? "#2b2d30" : "#3c3f41"}}>
            <Center className="h-9 w-9 hover:bg-[#484b4d]" onClick={() => appWindow.minimize()}>
                <Image src="/minimize.svg"/>
            </Center>
            <Center className="h-9 w-9 hover:bg-[#484b4d]" onClick={() => appWindow.toggleMaximize()}>
                <Image src="/maximize.svg"/>
            </Center>
            <Center className="h-9 w-9 hover:bg-[#e81123]" onClick={() => {
                invoke("save_state").then(null).catch(e => console.error(e));
                return appWindow.close();
            }}>
                <Image src="/close.svg"/>
            </Center>
        </Flex>
    );
}

export default TitleBar;