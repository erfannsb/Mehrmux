import {useEffect, useState} from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./styles/App.css";
import ProcessMonitor from "./Components/test.jsx";
import Titlebar from "./Components/titlebar.jsx";
import AlgoChoose from "./Components/algoChoose.jsx";
import GanttCont from "./Components/ganttCont.jsx";
import IoCont from "./Components/ioCont.jsx";
import {listen} from "@tauri-apps/api/event";

function App() {
    const [processes, setProcesses] = useState([]);
    const [selectedAlgo, setSelectedAlgo] = useState("FCFS");
    const [restartChart, setRestartChart] = useState(false);
    const [firstDate, setFirstDate] = useState(null)

    // Function to handle the restart chart trigger
    const handleRestartChart = (data) => {
        setRestartChart(data);  // Update the state to trigger the reset
    };

    const handleDate = (data) => {
        setFirstDate(data)
    }

    useEffect(() => {
        const unlisten = listen('update_process', (event) => {
            setProcesses((prevProcesses) => [
                ...event.payload, // Assuming event.payload is the Process object
            ]);
        });
    

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

  const handleChildState = (data) => {
      setSelectedAlgo(data);
  };

  return (
    <main className="container">
        <Titlebar />
        <AlgoChoose onDataChange={handleChildState} />
        <GanttCont processes={processes} restartChart={restartChart} firstDate={firstDate} selectedAlgo={selectedAlgo}/>
        <IoCont selectedAlgo={selectedAlgo} restartChart={handleRestartChart} restart={restartChart} handleDate={handleDate} />
    </main>
  );
}

export default App;
