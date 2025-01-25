import styles from "./../styles/iocont.module.css"
import {Splide, SplideSlide, SplideTrack} from "@splidejs/react-splide";
import '@splidejs/react-splide/css';
import React, {useEffect, useRef, useState} from 'react';
import {BarChart, Bar, Rectangle, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer} from 'recharts';

import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";
import chart from "./chart.jsx";

export default function IoCont({selectedAlgo, restartChart, restart, handleDate}) {
    const ref_AT = useRef(null);
    const ref_CBT = useRef(null);
    const ref_NP = useRef(null);
    const ref_CS = useRef(null);
    const ref_TS = useRef(null);
    const refM_AT = useRef(null);
    const refM_CBT = useRef(null);
    const refM_CS = useRef(null);
    const refM_TS = useRef(null);
    const refSelectQ1 = useRef(null);
    const refSelectQ2 = useRef(null);
    const refSelectQ3 = useRef(null);
    const refSelectQ4 = useRef(null);
    const [notify, setNotify] = React.useState(false);
    const [isRunning, setRunning] = React.useState(false);
    const [whichTab, setTab] = React.useState(0);
    const [nextForm, setNextForm] = React.useState(true);
    const [to_be_generated_processes, setToBeGeneratedProcesses] = useState([]);

    const [maxValue, setMaxValue] = useState(0)

    const [chartData, setData] = React.useState([
        {subject: 'TT'},
        {subject: 'WT'},
        {subject: 'RT',},
        {subject: 'CU',},
    ]);

    useEffect(() => {
        const unlistenMetrics = listen('send_metrics', (event) => {
            let tt = {subject: 'TT',};
            let wt = {subject: 'WT'};
            let rt = {subject: 'RT',};
            let cu = {subject: 'CU',};
            console.log("data is sent (metrics is sent)");
            console.log(event.payload);
            if (Array.isArray(event.payload)) {
                console.log("is an array");
                let element = ["A", "B", "C"];
                for (let [index, payload] of event.payload.entries()) {
                    console.log({payload, index});
                    console.log(tt[element[index]]);
                    tt[element[index]] = clampTo100(turnToNormalDuration(payload.average_turnaround_time.DurationValue));
                    wt[element[index]] = clampTo100(turnToNormalDuration(payload.average_waiting_time.DurationValue));
                    rt[element[index]] = clampTo100(turnToNormalDuration(payload.average_response_time.DurationValue));
                    cu[element[index]] = payload.cpu_utilization.PercentageValue.toFixed(2);
                }
            } else {
                console.log("is not an array");
                let metrics = event.payload;
                tt['TT'] = clampTo100(turnToNormalDuration(metrics.average_turnaround_time.DurationValue));
                wt['WT'] = clampTo100(turnToNormalDuration(metrics.average_waiting_time.DurationValue));
                rt['RT'] = clampTo100(turnToNormalDuration(metrics.average_response_time.DurationValue));
                cu['CU'] = metrics.cpu_utilization.PercentageValue.toFixed(2);
            }
            let data = [tt, wt, rt, cu];
            setMaxValue(Math.max(...data.map(data => data.A)));
            console.log(data);
            setData(data);
            setRunning(false)
        });

        return () => {
            unlistenMetrics.then((fn) => fn());
        }
    }, [])
    useEffect(() => {
        if (selectedAlgo != "MLQ" || selectedAlgo != "MLFQ") {
            setNextForm(true);
        }
    }, [selectedAlgo]);

    function turnToNormalDuration(duration) {
        let secs_to_millis = duration.secs * 1000;
        let nanos_to_millis = duration.nanos / 10e6;
        return (secs_to_millis + nanos_to_millis).toFixed(3);
    }

    const notify_this = (text) => {
        setNotify(text);
        setTimeout(() => {
            setNotify(false)
        }, 4000);
    }

    function clampTo100(value) {
        return (((value - 0) / (100 - 0)) * 100).toFixed(2);
    }

    const checkIFalgoIsPreemptive = (algo) => {
        switch (algo) {
            case "FIFO":
                return false
            case "SPN":
                return false
            case "FCFS":
                return false
            case "SJF":
                return true
            case "HRRN":
                return false
            case "RR":
                return true
            case "SRF":
                return true
            case "MLQ":
                return true
            case "MLFQ":
                return true
        }
    }
    const on_start_click = async (e) => {
        e.preventDefault();
        if (isRunning) {
            console.log(isRunning)
            return false;
        }

        setData([
            {subject: 'TT',},
            {subject: 'WT'},
            {subject: 'RT',},
            {subject: 'CU',},
        ])
        let at = ref_AT.current.value;
        let cbt = ref_CBT.current.value;
        let np = ref_NP.current.value;
        let cs = ref_CS.current.value;
        let ts = ref_TS.current.value;
        let isPreemptive = checkIFalgoIsPreemptive(selectedAlgo);
        if (isPreemptive) {
            if (!ts) {
                notify_this("Time Slice Is Required!");
                return false;
            }
        }

        if (!at || !cbt || !np || !cs) {
            notify_this("You Should Specify Inputs First!");
            return false;
        }

        if (at < 0.01 || at > 1000) {
            notify_this("Arrival Time Lambda Must Be Between 0.01 and 1000");
            return false;
        }

        if (cbt < 0.00001 || cbt > 1000) {
            notify_this("CBT Lambda Must Be Between 0.00001 and 1000");
            return false;
        }

        if (np < 1 || np > 100) {
            notify_this("Number Of Processes Must Be Between 1 and 100");
            return false;
        }

        if (cs < 1 || cs > 100000) {
            notify_this("Context Switch Must Be Between 1 and 100000");
            return false;
        }

        if (ts < 1 || ts > 100000) {
            if (isPreemptive) {
                notify_this("Time Slice Must Be Between 1 and 100000");
                return false;
            }
        }
        setRunning(true);
        restartChart(!restart);
        handleDate(new Date());
        console.log({at, cbt, np, selectedAlgo, cs, ts})
        await invoke("run_simulation", {
            atLambda: parseFloat(at),
            cbtLambda: parseFloat(cbt),
            numOfPrcss: parseInt(np),
            contextSwitch: parseInt(cs),
            queue: selectedAlgo,
            timeQuantum: parseInt(ts)
        });
    }

    const on_manual_start_click = async (e) => {
        e.preventDefault();
        if (isRunning) {
            console.log(isRunning)
            return false;
        }

        const cs = refM_CS.current.value;
        const ts = refM_TS.current.value;

        let isPreemptive = checkIFalgoIsPreemptive(selectedAlgo);
        if (isPreemptive) {
            if (!ts) {
                notify_this("Time Slice Is Required!");
                return false;
            }
        }

        if (cs < 1 || cs > 100000) {
            notify_this("Context Switch Must Be Between 1 and 100000");
            return false;
        }

        if (ts < 1 || ts > 100000) {
            if (isPreemptive) {
                notify_this("Time Slice Must Be Between 1 and 100000");
                return false;
            }
        }

        if (to_be_generated_processes.length == 0) {
            notify_this("No Process Specified!");
            return false;
        }

        if (!cs || !ts) {
            notify_this("Please Fill All Required Fields");
            return false;
        }

        setRunning(true);
        restartChart(!restart);
        handleDate(new Date());
        await invoke("run_with_parameters", {
            arrayOfProcesses: to_be_generated_processes,
            queue: selectedAlgo,
            contextSwitch: parseInt(cs),
            timeQuantum: parseInt(ts)
        });
    }
    const on_manual_add_click = (e) => {
        e.preventDefault();
        let at = refM_AT.current.value;
        let cbt = refM_CBT.current.value;
        at = parseInt(at);
        cbt = parseInt(cbt);

        if (!at || !cbt) {
            notify_this("You Should Specify Inputs First!");
            return false;
        }

        if (at < 0 || at > 200000) {
            notify_this("Arrival Time Must Be Between 1 and 10000");
            return false;
        }

        if (cbt < 1 || cbt > 200000) {
            notify_this("CBT Must Be Between 1 and 10000");
            return false;
        }

        handleDate(new Date());

        notify_this(`Process With ${at}ms Arrival Time And ${cbt}ms CBT Is Added To The Queue`);

        setToBeGeneratedProcesses([...to_be_generated_processes, [at, cbt]])

        console.log(to_be_generated_processes)

    }

    const on_reset_click = (e) => {
        e.preventDefault();
        setRunning(false);
        setToBeGeneratedProcesses([]);
        setData([
            {subject: 'TT',},
            {subject: 'WT'},
            {subject: 'RT',},
            {subject: 'CU',},
        ])
        restartChart(!restart);
        notify_this("Variables have been reset!");
    }

    const onNextClick = (e) => {
        e.preventDefault();
        setNextForm(false);
    }
    return <div className={styles.main}>
        <div className={styles.inputs}>
            <div className={styles.header}>
                Inputs
            </div>
            <div className={styles.notif}
                 style={notify ? {display: "block"} : {display: "none"}}>{notify ? notify : ""}</div>
            <div className={styles.tabs}>
                <button style={whichTab == 0 ? {backgroundColor: "#151E28"} : {}} onClick={() => setTab(0)}>Simulator
                </button>
                <button style={whichTab == 1 ? {backgroundColor: "#151E28"} : {}} onClick={() => setTab(1)}>Manual
                </button>
            </div>
            <div className={styles.tabs_container}>
                {
                    whichTab == 0 ?
                        <form className={styles.form}>
                            <div>
                                <label htmlFor="at">ARRIVAL TIME λ:</label>
                                <input id="at" ref={ref_AT} type="number" defaultValue={1}/>
                            </div>
                            <div>
                                <label htmlFor="cbt">CBT λ:</label>
                                <input id="cbt" ref={ref_CBT} type="number" defaultValue={1}/>
                            </div>
                            <div>
                                <label htmlFor="np">Enter The Number Of Processes:</label>
                                <input id="np" ref={ref_NP} type="number" defaultValue={10}/>
                            </div>
                            <div>
                                <label htmlFor="cs">Context Switch (ms):</label>
                                <input id="cs" ref={ref_CS} type="number" defaultValue={1000}/>
                            </div>
                            <div>
                                <label htmlFor="ts">Time Slice (ms):</label>
                                <input id="ts" ref={ref_TS} type="number" defaultValue={1000}/>
                            </div>
                            <div>
                                <input id="start" type="submit" value="start" onClick={on_start_click}/>
                            </div>
                        </form>
                        :
                        selectedAlgo == "MLQ" && nextForm || selectedAlgo == "MLFQ" && nextForm ?

                            <form className={styles.form_manual_mlq}>
                                <div>
                                    <label htmlFor="q1">#1 Queue Dicsipline:</label>
                                    <select name="q1" id="q1" ref={refSelectQ1}>
                                        <option value="RR">RR</option>
                                        <option value="SJF">SJF</option>
                                        <option value="SRTF">SRTF</option>
                                    </select>
                                </div>
                                <div>
                                    <label htmlFor="q2">#2 Queue Dicsipline:</label>
                                    <select name="q2" id="q2" ref={refSelectQ2}>
                                        <option value="RR">RR</option>
                                        <option value="SJF">SJF</option>
                                        <option value="SRTF">SRTF</option>
                                    </select>
                                </div>
                                <div>
                                    <label htmlFor="q3">#3 Queue Dicsipline:</label>
                                    <select name="q3" id="q3" ref={refSelectQ3}>
                                        <option value="RR">RR</option>
                                        <option value="SJF">SJF</option>
                                        <option value="SRTF">SRTF</option>
                                    </select>
                                </div>
                                <div>
                                    <label htmlFor="q4">#4 Queue Dicsipline:</label>
                                    <select name="q4" id="q4" ref={refSelectQ4}>
                                        <option value="FCFS">FCFS</option>
                                        <option value="SPN">SPN</option>
                                        <option value="HRRN">HRRN</option>
                                    </select>
                                </div>
                                <div>
                                    <label htmlFor="csq">Context Switch:</label>
                                    <input id="csq" ref={refM_AT} type="number" defaultValue={1}/>
                                </div>
                                <div>
                                    <label htmlFor="tmq">Time Slice:</label>
                                    <input id="tmq" ref={refM_AT} type="number" defaultValue={1}/>
                                </div>
                                <div>
                                    <input id="" type="submit" value="next" onClick={onNextClick}/>
                                </div>
                            </form>
                            :
                            <form className={styles.form_manual}>
                                <div>
                                    <label htmlFor="atm">ARRIVAL TIME (ms):</label>
                                    <input id="atm" ref={refM_AT} type="number" defaultValue={1}/>
                                </div>
                                <div>
                                    <label htmlFor="cbtm">CBT (ms):</label>
                                    <input id="cbtm" ref={refM_CBT} type="number" defaultValue={1}/>
                                </div>
                                {selectedAlgo == "MLQ" || selectedAlgo == "MLFQ" ?
                                    null
                                    : <>
                                        <div>
                                            <label htmlFor="csm">Context Switch (ms):</label>
                                            <input id="csm" ref={refM_CS} type="number" defaultValue={1000}/>
                                        </div>
                                        <div>
                                            <label htmlFor="tsm">Time Slice (ms):</label>
                                            <input id="tsm" ref={refM_TS} type="number" defaultValue={1000}/>
                                        </div>

                                    </>
                                }
                                <div>
                                    <input id="Addm" type="submit" value="Add" onClick={on_manual_add_click}/>
                                </div>
                                <div>
                                    <input id="stopm" type="submit" value="reset" onClick={on_reset_click}/>
                                </div>
                                <div>
                                    <input id="startm" type="submit" value="start" onClick={on_manual_start_click}/>
                                </div>
                            </form>

                }
            </div>

        </div>
        <div className={styles.metrics}>
            <div className={styles.header}>
                Metrics
            </div>
            {chartData[0]['TT'] == undefined ?
                <p style={{marginTop: 10, marginLeft: "auto", marginRight: "auto", color: "rgb(106, 110, 122)"}}>No Data
                    Yet</p> :
                <Splide className={styles.splide} options={{
                    pagination: false,
                }} hasTrack={false}>
                    <SplideTrack className={styles.tracker}>
                        <SplideSlide className={styles.slide}>
                            <ResponsiveContainer width="100%" height="100%">
                                <BarChart
                                    data={chartData}
                                >
                                    <CartesianGrid strokeDasharray="3 3"/>
                                    <XAxis dataKey="name"/>
                                    <YAxis/>
                                    <Tooltip/>
                                    <Legend/>
                                    <Bar dataKey="WT" fill="#FF5722" activeBar={<Rectangle fill="pink"/>}/>
                                    <Bar dataKey="TT" fill="#8884d8" activeBar={<Rectangle fill="pink"/>}/>
                                    <Bar dataKey="RT" fill="#3FC1C9" activeBar={<Rectangle fill="pink"/>}/>
                                    <Bar dataKey="CU" fill="#B8DE6F" activeBar={<Rectangle fill="pink"/>}/>
                                </BarChart>
                            </ResponsiveContainer>
                        </SplideSlide>
                        <SplideSlide className={styles.slider}>
                            <h4>Average Statistic:</h4>
                            <p>Average Response Time: <span
                                style={{color: "rgb(106, 110, 122)"}}>{chartData[2]['RT'] || 0}ms</span></p>
                            <p>Average Waiting Time: <span
                                style={{color: "rgb(106, 110, 122)"}}>{chartData[1]['WT'] || 0}ms</span></p>
                            <p>Average Turnaround Time: <span
                                style={{color: "rgb(106, 110, 122)"}}>{chartData[0]['TT'] || 0}ms</span></p>
                            <p>CPU Utilization: <span
                                style={{color: "rgb(106, 110, 122)"}}>{chartData[3]['CU'] || 0}%</span></p>
                        </SplideSlide>
                    </SplideTrack>
                </Splide>
            }
        </div>
    </div>;
}