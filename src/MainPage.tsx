import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { open as openDialog } from "@tauri-apps/api/dialog";
import React, { ChangeEvent, useEffect, useState } from "react";
import {
  Box,
  Flex,
  Group,
  Image,
  MultiSelect,
  ScrollArea,
  Select,
  Stack,
  Switch,
  Text,
  Textarea,
} from "@mantine/core";
import {
  create_file,
  get_contest_type,
  get_language,
  get_problem,
  get_problem_type,
  get_show_solved,
  next,
  open_file,
  previous,
  run,
  set_contest_type,
  set_directory,
  set_editor,
  set_language,
  set_problem_type,
  set_show_solved,
  submit,
} from "./commands.tsx";
import parse from "html-react-parser";
import { IconLoader } from "@tabler/icons-react";

const Languages = [
  { label: "C", value: "c" },
  { label: "C++", value: "cpp" },
  { label: "Dart", value: "dart" },
  { label: "Elixir", value: "elixir" },
  { label: "Fortran", value: "fortran" },
  { label: "F#", value: "f#" },
  { label: "Go", value: "go" },
  { label: "Haskell", value: "haskell" },
  { label: "Julia", value: "julia" },
  { label: "Kotlin", value: "kotlin" },
  { label: "OCaml", value: "ocaml" },
  { label: "Python", value: "python" },
  { label: "Rust", value: "rust" },
  { label: "Swift", value: "swift" },
  { label: "Zig", value: "zig" },
];

const PROBLEM_IDS = ["A", "B", "C", "D", "E", "F", "G", "H", "Ex"];

const MainPage = ({
  setDirectory,
  editor,
  setEditor,
}: {
  setDirectory: React.Dispatch<React.SetStateAction<string>>;
  editor: string;
  setEditor: React.Dispatch<React.SetStateAction<string>>;
}) => {
  let [problem, setProblem] = useState<{
    contest_id: number;
    contest_type: string;
    description: string;
    memory_limit: number;
    problem_id: string;
    test_cases_link: string;
    time_limit: number;
    title: string;
  } | null>(null);
  let [language, setLanguage] = useState("cpp" as string);
  let [contest, setContest] = useState("ABC" as string);
  let [problem_ids, setProblemIds] = useState(PROBLEM_IDS as string[]);
  let [testing, setTesting] = useState(false);
  let [showSolved, setShowSolved] = useState(false);
  let [showResult, setShowResult] = useState("description" as string);
  let [resultDisabled, setResultDisabled] = useState(false);
  let [caseIndex, setCaseIndex] = useState(0);
  let [finalVerdict, setFinalVerdict] = useState("Run Code" as string);
  let [verdicts, setVerdicts] = useState<
    {
      input: string;
      output: string;
      answer: string;
      status: string;
      time: number;
      memory: number;
    }[]
  >([]);

  const get_html_without_first_p = (html: string) => {
    let document = new DOMParser().parseFromString(html, "text/html");
    let first_p = document.getElementsByTagName("p")[0];
    first_p.remove();
    return document.body.innerHTML;
  };

  const main_get_problem = async () => {
    let problem = await get_problem();
    if (problem === null) return;
    setShowResult("description");
    setResultDisabled(false);
    setProblem(problem);
  };

  const onNext = async () => {
    await next();
    await main_get_problem();
  };

  const onPrevious = async () => {
    await previous();
    await main_get_problem();
  };

  const onSubmit = async () => {
    if (testing) return;
    setTesting(true);
    let verdicts = await submit();
    setTesting(false);
    handleVerdicts(verdicts ?? []);
  };

  const onRun = async () => {
    if (testing) return;
    setTesting(true);
    let verdicts = await run();
    setTesting(false);
    handleVerdicts(verdicts ?? []);
  };

  const handleVerdicts = (
    verdicts: {
      input: string;
      output: string;
      answer: string;
      status: string;
      time: number;
      memory: number;
    }[],
  ) => {
    console.log(verdicts);
    if (verdicts.length === 0) return;
    verdicts.every((v) => v.status === "AC")
      ? setFinalVerdict("Accepted")
      : setFinalVerdict("Wrong Answer");
    setVerdicts(verdicts);
    setResultDisabled(false);
    setShowResult("result");
  };

  const onCreate = async () => {
    await create_file();
  };

  const onChangeLanguage = async (value: string | null) => {
    if (value === null) return;
    let success = await set_language(value);
    if (success) setLanguage(value);
    await main_get_problem();
  };

  const onChangeContest = async (value: string | null) => {
    if (value === null) return;
    let success = await set_contest_type(value);
    if (success) setContest(value);
    await main_get_problem();
  };

  const onChangeProblemIds = async (value: string[]) => {
    let success = await set_problem_type(value);
    if (success) setProblemIds(value);
    await main_get_problem();
  };

  const onChangeShowSolved = async (e: ChangeEvent<HTMLInputElement>) => {
    console.log(e.target.checked);
    let success = await set_show_solved(!e.target.checked);
    if (success) setShowSolved(!e.target.checked);
    await main_get_problem();
  };

  const chooseEditor = async () => {
    const selected = await openDialog({
      directory: false,
      multiple: false,
      defaultPath: "C:/",
      filters: [{ name: "Application", extensions: ["exe"] }],
    });
    console.log(selected);
    if (!(Array.isArray(selected) || selected === null)) {
      let ed = selected.replace(/\\/g, "/");
      if (await set_editor(ed)) {
        setEditor(ed);
      }
    }
  };

  const onOpenFile = async () => {
    await open_file();
  };

  useEffect(() => {
    appWindow.setTitle("XCoder");
    get_problem().then((v) => setProblem(v));
    get_language().then((v) => setLanguage(v));
    get_contest_type().then((v) => setContest(v));
    get_problem_type().then((v) =>
      setProblemIds(v.map((x) => x.toUpperCase())),
    );
    get_show_solved().then((v) => setShowSolved(v));
  }, []);

  return (
    <Stack className={"p-2 h-full"}>
      <Box
        onClick={async () => {
          if (await set_directory("")) {
            setDirectory("");
            appWindow.setSize(new LogicalSize(600, 450)).then(null);
          }
        }}
        className="absolute px-2 py-2 cursor-pointer select-none right-2 top-10"
      >
        <Image src={"/close.svg"} w={19} />
      </Box>
      <Group className={"font-medium my-2 mx-10"}>
        <Box
          onClick={onPrevious}
          className="px-5 py-3 mt-auto active:drop-shadow-2xl border border-gray-500 cursor-pointer select-none"
        >
          <Image src={"/prev.svg"} w={15} />
        </Box>

        <Select
          c={"white"}
          value={contest}
          ml={20}
          className={"tracking-widest font-mono text-xl font-medium"}
          onChange={onChangeContest}
          label={"Contest"}
          data={["ABC", "ARC", "AGC"]}
          w={80}
          checkIconPosition={"right"}
          mb={"auto"}
          allowDeselect={false}
        />
        <Text
          mx={"auto"}
          mt={"auto"}
          fz={26}
          fw={500}
          c={"white"}
          className={"select-none tracking-wider cursor-pointer"}
          onClick={() => {
            open(
              `https://atcoder.jp/contests/${problem?.contest_type}${problem?.contest_id.toString().padStart(3, "0")}/tasks/${problem?.contest_type.toString().padStart(3, "0")}${problem?.contest_id}_${problem?.problem_id}`,
            );
          }}
        >
          {problem?.title}
        </Text>
        <MultiSelect
          c={"white"}
          mr={20}
          label={"Problem Type"}
          className={"tracking-widest font-mono text-xl font-medium"}
          data={PROBLEM_IDS}
          w={210}
          hidePickedOptions
          checkIconPosition={"right"}
          mb={3}
          maxValues={3}
          value={problem_ids}
          onChange={onChangeProblemIds}
        />

        <Box
          onClick={onNext}
          className="px-5 py-3 mt-auto active:drop-shadow-2xl border border-gray-500 cursor-pointer select-none"
        >
          <Image src={"/next.svg"} w={15} />
        </Box>
      </Group>

      <Flex ml={40}>
        <Box
          className={
            "rounded-full px-5 py-1 mx-3 font-mono hover:bg-[#2b2d30] active:italic cursor-pointer select-none"
          }
          style={{
            backgroundColor: showResult === "description" ? "#282828" : "",
            boxShadow: showResult === "description" ? "0 0 0 2px #f85d7e" : "",
            transform: showResult === "description" ? "scale(1.05)" : "",
          }}
          onClick={() => {
            setShowResult("description");
          }}
        >
          <Text fz={22} c={"white"}>
            Description
          </Text>
        </Box>
        <Box
          className={
            "rounded-full px-5 py-1 mx-3 font-mono hover:bg-[#2b2d30] active:italic cursor-pointer select-none"
          }
          style={{
            // use some grayish color please
            backgroundColor: showResult === "result" ? "#282828" : "",
            boxShadow: showResult === "result" ? "0 0 0 2px #5aff97" : "",
            transform: showResult === "result" ? "scale(1.05)" : "",
          }}
          onClick={() => {
            if (resultDisabled) return;
            setShowResult("result");
          }}
        >
          <Text fz={22} c={"white"}>
            Submit
          </Text>
        </Box>

        <Switch
          c={"white"}
          ml={"auto"}
          label="hide solved"
          checked={showSolved}
          onChange={onChangeShowSolved}
          my={"auto"}
          offLabel="OFF"
          onLabel="ON "
          className="mr-20 select-none font-mono"
        />
      </Flex>
      {/* Description box */}

      <Flex h={"82%"}>
        {showResult === "description" && (
          <ScrollArea
            c={"white"}
            w={"95%"}
            mx={"auto"}
            px={40}
            className={"text-2xl h-full border border-gray-700 rounded-md"}
          >
            <Group>
              <Text className="font-mono" fz={"xl"} c={"green"} mt={5}>
                Time Limit: {problem?.time_limit} sec{" "}
              </Text>
              <Text className="font-mono" fz={"xl"} c={"blue"} mt={5}>
                Memory Limit: {problem?.memory_limit} MB{" "}
              </Text>
            </Group>
            {parse(get_html_without_first_p(problem?.description ?? "<p></p>"))}
          </ScrollArea>
        )}

        {showResult === "result" && (
          <Stack
            c={"white"}
            w={"95%"}
            mx={"auto"}
            px={40}
            className={"text-2xl h-full border border-gray-700 rounded-md"}
          >
            {/*Heading Area*/}
            <Group mt={20}>
              <Select
                onChange={(v) => setCaseIndex(parseInt(v ?? "1") - 1)}
                data={Array.from(Array(verdicts.length).keys()).map((x) => ({
                  label:
                    `Case ${x + 1} ` +
                    (verdicts[x]?.status === "AC" ? "✔️" : "❌"),
                  value: `${x + 1}`,
                }))}
                w={120}
                mx={8}
                checkIconPosition={"right"}
                defaultValue={"1"}
                allowDeselect={false}
              />

              <Text
                fz={26}
                fw={600}
                ml={"auto"}
                className={"my-auto tracking-wider font-mono"}
                style={{
                  color:
                    finalVerdict === "Accepted"
                      ? "#2cad40"
                      : finalVerdict === "Wrong Answer"
                        ? "red"
                        : "gray",
                }}
              >
                {finalVerdict}
              </Text>

              <Group ml={"auto"} mr={40} className={"text-xl font-medium"}>
                <Select
                  c={"white"}
                  label={"Language"}
                  className={"tracking-widest font-mono"}
                  data={Languages}
                  checkIconPosition={"right"}
                  mb={3}
                  allowDeselect={false}
                  value={language}
                  onChange={onChangeLanguage}
                  searchable
                />
                <Box
                  c={"#c6c8cb"}
                  onClick={onCreate}
                  className="px-8 py-2 mt-auto bg-white/30 rounded-lg cursor-pointer select-none
                            hover:shadow hover:bg-[rgb(255 255 255 / 0.35);] active:bg-white/40"
                >
                  Create File
                </Box>
              </Group>
            </Group>

            {/*Result Area*/}
            <Group mt={20}>
              <Text
                fz={26}
                fw={600}
                ml={50}
                className={"tracking-wider font-mono"}
                style={{
                  color:
                    verdicts[caseIndex]?.status === "AC"
                      ? "#2cad40"
                      : verdicts[caseIndex]?.status === "WA"
                        ? "red"
                        : "gray",
                }}
              >
                {verdicts[caseIndex]?.status}
              </Text>
              <Text className="font-mono" fz={"xl"} c={"purple"} mx={20}>
                Time Taken: {verdicts[caseIndex]?.time?.toFixed(2)} sec{" "}
              </Text>
              <Text className="font-mono" fz={"xl"} c={"blue"} mx={20}>
                Memory : {verdicts[caseIndex]?.memory ?? "0"} MB{" "}
              </Text>
            </Group>
            <Group className="justify-between w-[100%]">
              <Box
                className={`h-[35rem] w-[31%] font-[500] bg-[#282828]
                        text-3xl border border-gray-600 relative text-center select-none font-mono
                         rounded-md tracking-widest pt-1`}
              >
                Input
                <Textarea
                  value={verdicts[caseIndex]?.input}
                  className={`h-[93%] w-full px-2 top-11 bg-[#3e3e3e]/50 rounded-md absolute`}
                  variant="unstyled"
                  maxRows={15}
                  autosize
                />
              </Box>
              <Box
                className={`mx-auto h-[35rem] w-[31%] font-[500] bg-[#282828]
                        text-3xl border border-gray-600 relative text-center select-none font-mono
                         rounded-md tracking-widest pt-1`}
              >
                Answer
                <Textarea
                  value={verdicts[caseIndex]?.answer}
                  className={`h-[93%] w-full px-2 top-11 bg-[#3e3e3e]/50 rounded-md absolute`}
                  variant="unstyled"
                  maxRows={15}
                  autosize
                />
              </Box>

              <Box
                className={`h-[35rem] w-[31%] font-[500] bg-[#282828]
                        text-3xl border border-gray-600 relative text-center select-none font-mono
                         rounded-md tracking-widest pt-1`}
              >
                Output
                <Textarea
                  value={verdicts[caseIndex]?.output ?? ""}
                  className={`h-[93%] w-full px-2 top-11 bg-[#3e3e3e]/50 rounded-md absolute`}
                  variant="unstyled"
                  maxRows={15}
                  autosize
                />
              </Box>
            </Group>

            {/*Submit Buttons*/}
            <Group className={"mt-8 text-xl font-medium"}>
              <Group className={"text-xl font-medium"}>
                {editor === "" ? (
                  <Box
                    c={"#c6c8cb"}
                    onClick={chooseEditor}
                    className="px-8 py-2 mt-auto bg-white/30 rounded-lg cursor-pointer select-none
                        hover:shadow hover:bg-[rgb(255 255 255 / 0.35);] active:bg-white/40"
                  >
                    Set Editor
                  </Box>
                ) : (
                  <Box
                    c={"#c6c8cb"}
                    onClick={onOpenFile}
                    className="px-8 py-2 mt-auto bg-white/30 rounded-lg cursor-pointer select-none
                        hover:shadow hover:bg-[rgb(255 255 255 / 0.35);] active:bg-white/40"
                  >
                    Open File
                  </Box>
                )}
              </Group>

              <Group ml={"auto"} mr={"xl"}>
                <Box
                  c={"#c6c8cb"}
                  onClick={onRun}
                  className="px-10 py-2 mx-10 bg-white/30 rounded-lg cursor-pointer select-none
                          hover:shadow hover:bg-[rgb(255 255 255 / 0.35);] active:bg-white/40"
                >
                  {testing ? <IconLoader size={"1.7rem"} /> : "Run"}
                </Box>
                <Box
                  c={"white"}
                  onClick={onSubmit}
                  className="px-10 py-2 w-36 bg-green-400/70 rounded-lg cursor-pointer select-none
                          hover:shadow hover:bg-green-400/80 active:bg-green-400/90"
                >
                  {testing ? (
                    <IconLoader size={"1.7rem"} className={"mx-auto"} />
                  ) : (
                    "Submit"
                  )}
                </Box>
              </Group>
            </Group>
          </Stack>
        )}
      </Flex>
    </Stack>
  );
};

export default MainPage;
