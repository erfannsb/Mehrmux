import {useEffect, useState} from "react";
import styles from "./../styles/ganttchart.module.css"
import ChromeDinoGame from 'react-chrome-dino';
import {listen} from "@tauri-apps/api/event";
import {Chart} from "react-google-charts";
import Dino from "./dino.jsx";
// import Chart from "./chart.jsx";

export default function GanttCont({processes, restartChart, firstDate, selectedAlgo}) {
    const [processEvent, setProcessEvent] = useState([]);
    const [finishedP, setFinishedP] = useState([]);
    const [rows, setRows] = useState([]);
    let [passed_times_for_all, setPTFA] = useState({});

    useEffect(() => {
        console.log("Resetting chart........................");
        setRows([]);          // Reset rows
        setFinishedP([]);     // Reset finished processes
    }, [restartChart]); // Trigger effect when `restartChart` changes

    useEffect(()=> {

        const unlistenPS = listen('process_stopped', (event) => {
            setProcessEvent(event.payload)
        });

        const unlistenPF = listen('finished_process', (event) => {
            setFinishedP([...event.payload])
        });

        return () => {
            unlistenPF.then((fn)=> fn());
            unlistenPS.then((fn) => fn());
        }
    }, [])
    
    function calculateTimeWithDate(date) { // calculates how much time passed from the first date in seconds   
        date = new Date(date)
        if(firstDate == null) {
            return 0;
        }

        // Calculate the difference in milliseconds
        return Math.abs(firstDate - date);
    }
    

    const columns = [
        { type: "string", id: "Process" },
        { type: "string", id: "Name" },
        { type: "date", id: "Start" },
        { type: "date", id: "End" },
    ];

    useEffect(()=> {
        if(processEvent == undefined || processEvent.length == 0) {
            return  
        }
        let keyNameForPassedTimes = processEvent.id.slice(0, 8);
        const last_execution = calculateTimeWithDate(processEvent.last_execution)
        const start_time = new Date(0,0,0,0,0,0, last_execution)
        let passed_time;
        if(passed_times_for_all[keyNameForPassedTimes] == undefined){
            passed_time = processEvent.processed_time.secs + processEvent.processed_time.nanos / 10e9;
            passed_time = passed_time * 1000;
            let ptfa = {...passed_times_for_all}
            ptfa[keyNameForPassedTimes] = passed_time;
            setPTFA({...passed_times_for_all, ...ptfa});
        } else {
            passed_time = processEvent.processed_time.secs + processEvent.processed_time.nanos / 10e9;
            passed_time = passed_time * 1000;
            passed_time = passed_time - passed_times_for_all[keyNameForPassedTimes];
            let ptfa = {...passed_times_for_all}
            ptfa[keyNameForPassedTimes] += passed_time;
            setPTFA({...passed_times_for_all, ...ptfa});
        }

        let end_time = new Date(0,0,0,0,0, 0, last_execution + passed_time);
        setRows([...rows, [processEvent.id.slice(0,8), `Process: ${processEvent.id.slice(0,8)}`, start_time, end_time]])
    }, [processEvent])

    const data = [columns, ...rows];
    const colors = [
        "#F38181", // Coral Red
        "#FCE38A", // Lemon Yellow
        "#EAFFD0", // Light Lime Green
        "#95E1D3", // Teal
        "#A8D8EA", // Sky Blue
        "#AA96DA", // Lavender Purple
        "#FC5185", // Watermelon Pink
        "#3FC1C9", // Aqua Blue
        "#FFDD59", // Bright Yellow
        "#FF5722", // Vibrant Orange
        "#C1C8E4", // Soft Periwinkle
        "#FFD460", // Soft Mustard
        "#B8DE6F", // Soft Green
        "#FF6B6B", // Soft Red
        "#6A0572", // Deep Purple
        "#FFE5B4", // Pale Peach
        "#9DDCDC", // Light Cyan
        "#FFB6C1", // Light Pink
        "#F8B195", // Peach Pink
        "#355C7D"  // Slate Blue
    ];
    function formatDateToTime(dateString) {
        let date = new Date(dateString);
        // Use Intl.DateTimeFormat to convert to Tehran time zone
        let options = {
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            fractionalSecond: '3-digit', // For milliseconds
            timeZone: 'Asia/Tehran',
        };

        // Use Intl.DateTimeFormat to get the time in Tehran time zone
        let tehranTime = new Intl.DateTimeFormat('en-GB', options).format(date);

        // Extract milliseconds manually
        let milliseconds = date.getMilliseconds();

        // Return formatted time with milliseconds
        return `${tehranTime}.${milliseconds}`;
    }

    function turnToNormalDuration(duration) {
        let secs_to_millis = duration.secs * 1000;
        let nanos_to_millis = duration.nanos / 10e6;
        return (secs_to_millis + nanos_to_millis).toFixed(3);
    }


    return <div className={styles.main}>
        <div className={styles.gantt}>
            <div className={styles.gantt_head}>
                <div className={styles.title}>Simulating CPU Scheduler</div>
                <div className={styles.info}>
                </div>
            </div>
            <div className={styles.gantt_main}>
                {selectedAlgo == "MLQ" || selectedAlgo == "MLFQ" ?
                    <div className={styles.multiChartCont}>
                        <div>
                            <div className={styles.chartTitle}><span>Q1</span></div>
                            <Chart className={styles.thechart} chartType="Timeline" data={data} width="100%"
                                   height="100%" options={{
                                colors: colors,
                                backgroundColor: "#0E1321",
                            }}/>
                        </div>
                        <div>
                            <div className={styles.chartTitle}><span>Q2</span></div>
                            <Chart className={styles.thechart} chartType="Timeline" data={data} width="100%"
                                   height="100%" options={{
                                colors: colors,
                                backgroundColor: "#0E1321",
                            }}/>
                        </div>
                        <div>
                            <div className={styles.chartTitle}><span>Q3</span></div>
                            <Chart className={styles.thechart} chartType="Timeline" data={data} width="100%"
                                   height="100%" options={{
                                colors: colors,
                                backgroundColor: "#0E1321",
                            }}/>
                        </div>
                        <div>
                            <div className={styles.chartTitle}><span>Q4</span></div>
                            <Chart className={styles.thechart} chartType="Timeline" data={data} width="100%"
                                   height="100%" options={{
                                colors: colors,
                                backgroundColor: "#0E1321",
                            }}/>
                        </div>

                    </div>
                    :

                    <Chart className={styles.thechart} chartType="Timeline" data={data} width="100%" height="100%"
                           options={{
                               colors: colors,
                               backgroundColor: "#0E1321",
                           }}/>
                }
            </div>
            <div className={styles.gantt_footer}>
                <ChromeDinoGame className="gantt_footer_game" />
            </div>
        </div>
        <div className={styles.readyQueue}>
            <div className={styles.title}>Ready Queue</div>
            {
                processes.length > 0 &&
                processes.map((process, index) =>  <ProcessElement id={process.id.toString().slice(0, 8)} AT={calculateTimeWithDate(process.arrival_time)} STATUS={process.status} CBT={`${turnToNormalDuration(process.cpu_burst_time)}ms`}  color={colors[index % 20]}/>
                )
            }
        </div>
        <div className={styles.finished}>
            <div className={styles.title}>Finished Processes</div>
            {
                finishedP.length > 0 &&
                finishedP.map((process, index) =>  <ProcessElement id={process.id.toString().slice(0, 8)} AT={calculateTimeWithDate(process.arrival_time)} STATUS={process.status} CBT={`${turnToNormalDuration(process.cpu_burst_time)}ms`}  color={colors[index % 20]}/>
                )
            }
        </div>
    </div>
}

export function ProcessElement({id, AT, STATUS, CBT, color}) {
    return (
        <div className={styles.process_card_in_queue} >
            <h3 style={{color: color}}>Process {id}</h3>
            <p>AT: {AT}ms</p>
            <p>CBT: {CBT}</p>
            <p>STATUS: {STATUS}</p>
        </div>
    )
}