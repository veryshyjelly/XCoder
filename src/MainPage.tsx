import {appWindow, LogicalSize} from "@tauri-apps/api/window";
import React, {useEffect, useState} from "react";
import {Button, Text} from "@mantine/core";
import {invoke} from "@tauri-apps/api";
import {set_directory} from "./commands.tsx";

const MainPage = ({setDirectory}: { setDirectory: React.Dispatch<React.SetStateAction<string>> }) => {
    let [problem, setProblem] = useState<{
        contest_id: number,
        contest_type: string,
        description: string,
        memory_limit: number,
        problem_id: string,
        test_cases_link: string,
        time_limit: number,
        title: string
    } | null>(null);

    useEffect(() => {
        appWindow.setTitle("XCoder");
        appWindow.setSize(new LogicalSize(1080, 720));
        appWindow.maximize();
        invoke("get_problem").then((problem) => {
            console.log(problem);
            setProblem(problem as {
                contest_id: number,
                contest_type: string,
                description: string,
                memory_limit: number,
                problem_id: string,
                test_cases_link: string,
                time_limit: number,
                title: string
            });
        }).catch(e => console.error(e));
    }, []);

    return (<>
        <Text c={"white"}>{problem?.title}</Text>
        <Button onClick={async () => {
            if (await set_directory("")) {
                setDirectory("");
            }
        }}>Close Project</Button>
    </>)
}

export default MainPage;