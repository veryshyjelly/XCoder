import { Box } from "@mantine/core";
import "./App.css";
import LandingPage from "./LandingPage";
import TitleBar from "./Titlebar";
import MainPage from "./MainPage.tsx";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";

function App() {
  const [directory, setDirectory] = useState("");
  const [editor, setEditor] = useState("");

  useEffect(() => {
    invoke("get_directory")
      .then((dir) => {
        if (dir !== "")
          appWindow.setSize(new LogicalSize(1600, 900)).then(null);
        setDirectory(dir as string);
      })
      .catch((e) => console.error(e));
    invoke("get_editor")
      .then((ed) => {
        setEditor(ed as string);
      })
      .catch((e) => console.error(e));
  });

  return (
    <Box
      className="bg-[#1e1f22] border border-[#3c3f41]"
      style={{ height: "100%", width: "100%", position: "fixed" }}
    >
      <TitleBar />
      {directory !== "" && (
        <MainPage
          setDirectory={setDirectory}
          setEditor={setEditor}
          editor={editor}
        />
      )}
      {directory === "" && <LandingPage setDirectory={setDirectory} />}
    </Box>
  );
}

export default App;
