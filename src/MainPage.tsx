import {appWindow, LogicalSize} from "@tauri-apps/api/window";
import React, {useEffect, useState} from "react";
import {
    Box,
    Flex,
    Group,
    Image,
    MultiSelect,
    ScrollArea,
    SegmentedControl,
    Select,
    Stack,
    Text,
    Textarea
} from "@mantine/core";
import {
    create_file,
    get_contest_type,
    get_language,
    get_problem,
    get_problem_type,
    next,
    previous,
    run,
    set_contest_type,
    set_directory,
    set_language,
    set_problem_type,
    submit
} from "./commands.tsx";
import parse from "html-react-parser";
import {IconLoader} from "@tabler/icons-react";

const Languages = [
    {label: "C", value: "c"},
    {label: "C++", value: "cpp"},
    {label: "Go", value: "go"},
    {label: "Rust", value: "rust"},
    {label: "Kotlin", value: "kotlin"},
    {label: "Zig", value: "zig"},
    {label: "Swift", value: "swift"},
    {label: "Dart", value: "dart"},
    {label: "Haskell", value: "haskell"},
    {label: "Fortran", value: "fortran"},
    {label: "OCaml", value: "ocaml"},
]

const PROBLEM_IDS = ["A", "B", "C", "D", "E", "F", "G", "H", "Ex"];

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
    let [language, setLanguage] = useState("cpp" as string);
    let [contest, setContest] = useState("ABC" as string);
    let [problem_ids, setProblemIds] = useState(PROBLEM_IDS as string[]);
    let [testing, setTesting] = useState(false);
    let [showResult, setShowResult] = useState("description" as string);

    const get_html_without_first_p = (html: string) => {
        let document = new DOMParser().parseFromString(html, "text/html");
        let first_p = document.getElementsByTagName("p")[0];
        first_p.remove();
        return document.body.innerHTML;
    }

    const main_get_problem = async () => {
        let problem = await get_problem();
        setProblem(problem);
    }

    const onNext = async () => {
        await next();
        await main_get_problem();
    }

    const onPrevious = async () => {
        await previous();
        await main_get_problem();
    }

    const onSubmit = async () => {
        if (testing) return;
        setTesting(true);
        await submit();
        setTesting(false);
    }

    const onRun = async () => {
        if (testing) return;
        setTesting(true);
        let verdicts = await run();
        console.log(verdicts);
        setTesting(false);
    }

    const onCreate = async () => {
        await create_file();
    }

    const onChangeLanguage = async (value: string | null) => {
        if (value === null) return;
        let success = await set_language(value);
        if (success) setLanguage(value);
        await main_get_problem();
    }

    const onChangeContest = async (value: string | null) => {
        if (value === null) return;
        let success = await set_contest_type(value);
        if (success) setContest(value);
        await main_get_problem();
    }

    const onChangeProblemIds = async (value: string[]) => {
        let success = await set_problem_type(value);
        if (success) setProblemIds(value);
        await main_get_problem();
    }

    useEffect(() => {
        appWindow.setTitle("XCoder");
        get_problem().then(v => setProblem(v));
        get_language().then(v => setLanguage(v));
        get_contest_type().then(v => setContest(v));
        get_problem_type().then(v => setProblemIds(v.map(x => x.toUpperCase())));
    }, []);

    return (<Stack className={"p-2 h-full"}>

        <Box onClick={async () => {
            if (await set_directory("")) {
                setDirectory("");
                appWindow.setSize(new LogicalSize(600, 450)).then(null);
            }
        }}
             className="absolute px-2 py-2 cursor-pointer select-none right-2 top-10">
            <Image src={"/close.svg"} w={19}/>
        </Box>
        <Group className={"font-medium mt-5 mx-10"}>
            <Box onClick={onPrevious}
                 className="px-5 py-2 active:drop-shadow-2xl border border-gray-500 cursor-pointer select-none">
                <Image src={"/prev.svg"} w={15}/>
            </Box>
            <Text mx={"auto"} my={"auto"} fz={26} fw={500} c={"white"}
                  className={"select-none tracking-wider cursor-pointer"}>{problem?.title}</Text>
            <Box onClick={onNext}
                 className="px-5 py-2 active:drop-shadow-2xl border border-gray-500 cursor-pointer select-none">
                <Image src={"/next.svg"} w={15}/>
            </Box>
        </Group>

        <Box>
            <SegmentedControl size={"md"} value={showResult} onChange={setShowResult}
                              bg={"#2b2d30"}
                              ml={40}
                              data={[
                                  {label: "Description", value: "description"},
                                  {label: "Result", value: "result"}
                              ]}/>
        </Box>
        {/* Description box */}

        <Flex h={"82%"}>
            {showResult === "description" &&
                <ScrollArea c={"white"} w={"80%"} mx={"auto"} px={40}
                            className={"text-2xl h-full border border-gray-700 rounded-md"}>
                    {parse(get_html_without_first_p(problem?.description ?? "<p></p>"))}
                </ScrollArea>}

            {showResult === "result" &&
                <Stack c={"white"} w={"80%"} mx={"auto"} px={40}
                       className={"text-2xl h-full border border-gray-700 rounded-md"}>
                    <Group>
                        <Select data={[{label: "Case 1 ✔️", value: "1"}, {value: "2", label: "Case 2 ❌"}]} w={120}
                                mt={50} mx={8} checkIconPosition={"right"} defaultValue={"1"}/>

                    </Group>
                    <Group mt={50}>
                        <Box
                            className={`mx-[0.5%] h-[36rem] w-[31%] font-[500] bg-[#282828] 
                        text-3xl border border-gray-600 relative text-center select-none font-mono
                         rounded-md tracking-widest pt-1`}>
                            Input
                            <Textarea
                                value={"a\nb\nc\nd\ne\nf\ng\nh"}
                                className={`h-[93%] w-full px-2 top-11 bg-[#3e3e3e]/50 rounded-md absolute`}
                                variant="unstyled" maxRows={15} autosize/>
                        </Box>
                        <Box
                            className={`mx-[0.5%] h-[36rem] w-[31%] font-[500] bg-[#282828] 
                        text-3xl border border-gray-600 relative text-center select-none font-mono
                         rounded-md tracking-widest pt-1`}>
                            Answer
                            <Textarea
                                value={"a\nb\nc\nd\ne\nf\ng\nh"}
                                className={`h-[93%] w-full px-2 top-11 bg-[#3e3e3e]/50 rounded-md absolute`}
                                variant="unstyled" maxRows={15} autosize/>
                        </Box>

                        <Box
                            className={`mx-[0.5%] h-[36rem] w-[31%] font-[500] bg-[#282828] 
                        text-3xl border border-gray-600 relative text-center select-none font-mono
                         rounded-md tracking-widest pt-1`}>
                            Output
                            <Textarea
                                value={"a\nb\nc\nd\ne\nf\ng\nh"}
                                className={`h-[93%] w-full px-2 top-11 bg-[#3e3e3e]/50 rounded-md absolute`}
                                variant="unstyled" maxRows={15} autosize/>
                        </Box>
                    </Group>
                </Stack>
            }

            {/* Buttons and Controls */}
            <Stack mx={"auto"}>
                <Group mx={"auto"} className={"mt-24 text-xl font-medium"}>
                    <Select c={"white"} label={"Language"} className={"tracking-widest font-mono"} data={Languages}
                            w={110}
                            checkIconPosition={"right"} mb={3} allowDeselect={false} value={language}
                            onChange={onChangeLanguage}/>
                    <Box c={"#c6c8cb"} onClick={onCreate}
                         className="px-8 py-2 mt-auto bg-white/30 rounded-lg cursor-pointer select-none
                        hover:shadow hover:bg-[rgb(255 255 255 / 0.35);] active:bg-white/40">
                        Create File
                    </Box>
                </Group>

                <Group mx={"auto"} className={"mt-10 text-xl font-medium"}>
                    <Select c={"white"} value={contest} className={"tracking-widest font-mono"}
                            onChange={onChangeContest} label={"Contest"}
                            data={["ABC", "ARC", "AGC"]} w={80}
                            checkIconPosition={"right"} mb={"auto"} allowDeselect={false}/>
                    <MultiSelect c={"white"} label={"Problem"} className={"tracking-widest font-mono"}
                                 data={PROBLEM_IDS} w={190} hidePickedOptions
                                 checkIconPosition={"right"} mb={3} value={problem_ids} onChange={onChangeProblemIds}/>
                </Group>

                <Group mx={"auto"} className={"mt-auto text-xl font-medium"}>
                    <Box c={"#c6c8cb"} onClick={onRun}
                         className="px-10 py-2 bg-white/30 rounded-lg cursor-pointer select-none
                        hover:shadow hover:bg-[rgb(255 255 255 / 0.35);] active:bg-white/40">
                        {testing ? <IconLoader size={"1.7rem"}/> : "Run"}
                    </Box>
                    <Box c={"white"} onClick={onSubmit}
                         className="px-10 py-2 w-36 bg-green-400/70 rounded-lg cursor-pointer select-none
                        hover:shadow hover:bg-green-400/80 active:bg-green-400/90">
                        {testing ? <IconLoader size={"1.7rem"} className={"mx-auto"}/> : "Submit"}
                    </Box>
                </Group>
            </Stack>

        </Flex>

    </Stack>);
}

export default MainPage;