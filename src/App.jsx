import { useEffect, useState } from "react";
import "./styles/App.css";
import Titlebar from "./Components/titlebar.jsx";
import AlgoChoose from "./Components/algoChoose.tsx";
import GanttCont from "./Components/ganttCont.tsx";
import IoCont from "./Components/ioCont.jsx";
import { listen } from "@tauri-apps/api/event";
import useStore from "./ui_storage.tsx";

function App() {
  const setProcesses = useStore((state) => state.setProcesses);

  useEffect(() => {
    const unlisten = listen("update_process", (event) => {
      setProcesses(() => [
        ...event.payload, // Assuming event.payload is the Process object
      ]);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return (
    <main className="container">
      <Titlebar />
      <AlgoChoose />
      <GanttCont />
      <IoCont />
    </main>
  );
}

export default App;
