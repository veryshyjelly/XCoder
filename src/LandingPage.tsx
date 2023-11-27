import {Box, Flex, Image, Space, Stack, Text} from "@mantine/core";
import {invoke} from "@tauri-apps/api";
import {open} from '@tauri-apps/api/dialog';
import React from "react";

const LandingPage = ({setDirectory}: { setDirectory: React.Dispatch<React.SetStateAction<string>> }) => {
    const chooseFolder = async () => {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: "C:/"
        });
        console.log(selected);
        if (!(Array.isArray(selected) || selected === null)) {
            invoke("set_directory", {directory: selected}).then(() => {
                setDirectory(selected);
            }).catch(e => console.error(e));
        }
    }

    return (
        <Flex direction={"column"} align={"center"} style={{userSelect: "none"}}>
            <Text c={"#dee0e2"} fw={500} mt={50} fz={40}>
                Welcome to XCoder
            </Text>
            <Space h={"md"}/>
            <Text c={"#6f737a"} fz={"sm"}>
                Create a new project to start from scratch or open existing folder.
            </Text>
            <Space h={"lg"}/>
            <Flex>
                <Stack m={40}>
                    <Box bg={"#2b2d30"} p={15} style={{borderRadius: 7, cursor: 'pointer'}}>
                        <Image src={"/add.svg"} w={35} m={"auto"}/>
                    </Box>
                    <Text fz={"xs"} c={"#d5dee1"}>New Project</Text>
                </Stack>
                <Stack m={40}>
                    <Box bg={"#2b2d30"} p={15} style={{borderRadius: 7, cursor: 'pointer'}} onClick={chooseFolder}>
                        <Image src={"/folder.svg"} w={35} m={"auto"}/>
                    </Box>
                    <Text fz={"xs"} c={"#d5dee1"}>Open Existing</Text>
                </Stack>
            </Flex>
        </Flex>
    );
}

export default LandingPage;