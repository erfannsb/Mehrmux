import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke} from "@tauri-apps/api/core";

function ProcessMonitor() {
    const [processes, setProcesses] = useState([]);

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

    useEffect(()=> {
        console.log(processes);
    }, [processes]);

    const run_simulation = async () => {
        await invoke("run_simulation", processes);
    }

    return (
        <div>
            <button onClick={run_simulation}>run simulation</button>
            {processes.map((process) => (
                <div key={process.id}>
                    <p>ID: {process.id}</p>
                    <p>Status: {process.status}</p>
                    <p>CBT: {process.cpu_burst_time.secs} secs {process.cpu_burst_time.nanos} nanos </p>
                    <p>Processed Time: {process.processed_time.secs} secs {process.processed_time.nanos} nanos</p>
                </div>
            ))}
        </div>
    );
}

export default ProcessMonitor;