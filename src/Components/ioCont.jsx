import styles from "./../styles/iocont.module.css";
import { Splide, SplideSlide, SplideTrack } from "@splidejs/react-splide";
import "@splidejs/react-splide/css";
import React, { useEffect, useRef, useState } from "react";
import {
  BarChart,
  Bar,
  Rectangle,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from "recharts";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import useStore from "./../ui_storage.jsx";

export default function IoCont() {
  const selectedAlgo = useStore((state) => state.selectedAlgo);
  const restartChart = useStore((state) => state.setRestartChart);
  const restart = useStore((state) => state.restartChart);
  const handleDate = useStore((state) => state.setStartingDate);

  // REFERENCES HERE
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
  const refMQ_CS = useRef(null);
  const refMQ_TS = useRef(null);
  const [isRunning, setRunning] = React.useState(false);
  const [whichTab, setTab] = React.useState(0);
  const [nextForm, setNextForm] = React.useState(true);
  const [to_be_generated_processes, setToBeGeneratedProcesses] = useState([]);
  const [whichMeow, setMeow] = React.useState("TT");
  const [queueInfo, SetQueueInfo] = React.useState([]);
  const [isError, setError] = React.useState(false);
  const refMQ_PT = useRef(null);
  const [selectedPT, setSelectedPT] = useState();

  const [chartData, setData] = React.useState([
    { subject: "TT" },
    { subject: "WT" },
    { subject: "RT" },
    { subject: "CU" },
  ]);

  const [multiChart, setMultiChart] = React.useState([
    { name: "FCFS" },
    { name: "SPN" },
    { name: "SJF" },
    { name: "HRRN" },
    { name: "RR" },
    { name: "MLQ" },
    { name: "MLFQ" },
    { name: "SRTF" },
    { name: "FIFO" },
  ]);

  useEffect(() => {
    const unlistenMetrics = listen("send_metrics", (event) => {
      let tt = { subject: "TT" };
      let wt = { subject: "WT" };
      let rt = { subject: "RT" };
      let cu = { subject: "CU" };
      console.log("data is sent (metrics is sent)");
      console.log(event.payload);
      let metrics = event.payload;
      let copyOfMC = multiChart.map((e) => e);
      for (let queue of copyOfMC) {
        if (queue.name == metrics.queue_discipline.StringValue) {
          queue["TT"] = turnToNormalDuration(
            metrics.average_turnaround_time.DurationValue
          );
          queue["WT"] = turnToNormalDuration(
            metrics.average_waiting_time.DurationValue
          );
          queue["RT"] = turnToNormalDuration(
            metrics.average_response_time.DurationValue
          );
          queue["CU"] = metrics.cpu_utilization.PercentageValue.toFixed(2);
          tt["TT"] = turnToNormalDuration(
            metrics.average_turnaround_time.DurationValue
          );
          wt["WT"] = turnToNormalDuration(
            metrics.average_waiting_time.DurationValue
          );
          rt["RT"] = turnToNormalDuration(
            metrics.average_response_time.DurationValue
          );
          cu["CU"] = metrics.cpu_utilization.PercentageValue.toFixed(2);
        }
        setMultiChart(copyOfMC);
      }
      let data = [tt, wt, rt, cu];
      console.log(data);
      setData(data);
      setRunning(false);
    });
    const unlistenMetrics2 = listen("send_metrics_mlq", (event) => {
      let tt = { subject: "TT", TT: 0 };
      let wt = { subject: "WT", WT: 0 };
      let rt = { subject: "RT", RT: 0 };
      let cu = { subject: "CU", CU: 0 };
      let copyOfMC = multiChart.map((e) => e);
      for (let queue of copyOfMC) {
        if (queue.name == "MLQ") {
          try {
            for (let data of event.payload) {
              tt["TT"] += parseFloat(
                turnToNormalDuration(data.average_turnaround_time.DurationValue)
              );
              wt["WT"] += parseFloat(
                turnToNormalDuration(data.average_waiting_time.DurationValue)
              );
              rt["RT"] += parseFloat(
                turnToNormalDuration(data.average_response_time.DurationValue)
              );
              cu["CU"] += parseFloat(
                data.cpu_utilization.PercentageValue.toFixed(2)
              );
            }
            tt["TT"] /= event.payload.length;
            wt["WT"] /= event.payload.length;
            rt["RT"] /= event.payload.length;
            cu["CU"] /= event.payload.length;
            queue["TT"] = tt["TT"];
            queue["WT"] = wt["WT"];
            queue["RT"] = rt["RT"];
            queue["CU"] = cu["CU"];
          } catch (e) {
            console.log(e);
          }
        }
      }

      setMultiChart(copyOfMC);
      let data = [tt, wt, rt, cu];
      console.log(data);
      setData(data);
      setRunning(false);
    });
    const unlistenMetrics3 = listen("send_metrics_mlfq", (event) => {
      let tt = { subject: "TT", TT: 0 };
      let wt = { subject: "WT", WT: 0 };
      let rt = { subject: "RT", RT: 0 };
      let cu = { subject: "CU", CU: 0 };
      let copyOfMC = multiChart.map((e) => e);
      for (let queue of copyOfMC) {
        if (queue.name == "MLFQ") {
          try {
            for (let data of event.payload) {
              tt["TT"] += parseFloat(
                turnToNormalDuration(data.average_turnaround_time.DurationValue)
              );
              wt["WT"] += parseFloat(
                turnToNormalDuration(data.average_waiting_time.DurationValue)
              );
              rt["RT"] += parseFloat(
                turnToNormalDuration(data.average_response_time.DurationValue)
              );
              cu["CU"] += parseFloat(
                data.cpu_utilization.PercentageValue.toFixed(2)
              );
            }
            tt["TT"] /= event.payload.length;
            wt["WT"] /= event.payload.length;
            rt["RT"] /= event.payload.length;
            cu["CU"] /= event.payload.length;
            queue["TT"] = tt["TT"];
            queue["WT"] = wt["WT"];
            queue["RT"] = rt["RT"];
            queue["CU"] = cu["CU"];
          } catch (e) {
            console.log(e);
          }
        }
      }

      setMultiChart(copyOfMC);
      let data = [tt, wt, rt, cu];
      console.log(data);
      setData(data);
      setRunning(false);
    });

    return () => {
      unlistenMetrics.then((fn) => fn());
      unlistenMetrics2.then((fn) => fn());
      unlistenMetrics3.then((fn) => fn());
    };
  }, []);
  useEffect(() => {
    if (selectedAlgo != "MLQ" || selectedAlgo != "MLFQ") {
      setNextForm(true);
    }
  }, [selectedAlgo]);

  function turnToNormalDuration(duration) {
    let secs_to_millis = parseFloat(duration.secs) * 1000;
    let nanos_to_millis = parseFloat(duration.nanos) / 10e6;
    return (secs_to_millis + nanos_to_millis).toFixed(3);
  }

  const notify_this = (text, isError) => {
    setError(isError);
    setNotify(text);
    setTimeout(() => {
      setNotify(false);
    }, 4000);
  };

  const checkIFalgoIsPreemptive = (algo) => {
    switch (algo) {
      case "FIFO":
        return false;
      case "SPN":
        return false;
      case "FCFS":
        return false;
      case "SJF":
        return true;
      case "HRRN":
        return false;
      case "RR":
        return true;
      case "SRF":
        return true;
      case "MLQ":
        return true;
      case "MLFQ":
        return true;
    }
  };
  const on_start_click = async (e) => {
    e.preventDefault();
    if (isRunning) {
      console.log(isRunning);
      return false;
    }

    let at = ref_AT.current.value;
    let cbt = ref_CBT.current.value;
    let np = ref_NP.current.value;
    let cs = ref_CS.current.value;
    let ts = ref_TS.current.value;
    let isPreemptive = checkIFalgoIsPreemptive(selectedAlgo);
    if (isPreemptive) {
      if (!ts) {
        notify_this("Time Slice Is Required!", true);
        return false;
      }
    }

    if (!at || !cbt || !np || !cs) {
      notify_this("You Should Specify Inputs First!", true);
      return false;
    }

    if (at < 0.01 || at > 1000) {
      notify_this("Arrival Time Lambda Must Be Between 0.01 and 1000", true);
      return false;
    }

    if (cbt < 0.00001 || cbt > 1000) {
      notify_this("CBT Lambda Must Be Between 0.00001 and 1000", true);
      return false;
    }

    if (np < 1 || np > 100) {
      notify_this("Number Of Processes Must Be Between 1 and 100", true);
      return false;
    }

    if (cs < 1 || cs > 100000) {
      notify_this("Context Switch Must Be Between 1 and 100000", true);
      return false;
    }

    if (ts < 1 || ts > 100000) {
      if (isPreemptive) {
        notify_this("Time Slice Must Be Between 1 and 100000", true);
        return false;
      }
    }
    setRunning(true);
    restartChart(!restart);
    handleDate(new Date());
    console.log({ at, cbt, np, selectedAlgo, cs, ts });
    if (selectedAlgo == "MLQ" || selectedAlgo == "MLFQ") {
      await invoke("run_simulation", {
        atLambda: parseFloat(at),
        cbtLambda: parseFloat(cbt),
        numOfPrcss: parseInt(np),
        contextSwitch: parseInt(cs),
        queue: selectedAlgo,
        timeQuantum: parseInt(ts),
        listOfDiscipline: ["RR", "RR", "RR", "FCFS"],
      });
    } else {
      await invoke("run_simulation", {
        atLambda: parseFloat(at),
        cbtLambda: parseFloat(cbt),
        numOfPrcss: parseInt(np),
        contextSwitch: parseInt(cs),
        queue: selectedAlgo,
        timeQuantum: parseInt(ts),
        listOfDiscipline: null,
      });
    }
  };

  const on_manual_start_click = async (e) => {
    e.preventDefault();
    if (isRunning) {
      console.log(isRunning);
      return false;
    }

    let cs, ts;
    if (selectedAlgo == "MLQ" || selectedAlgo == "MLFQ") {
      cs = queueInfo[5];
      ts = queueInfo[4];
    } else {
      cs = refM_CS.current.value;
      ts = refM_TS.current.value;
    }
    console.log("on manual fuck click");
    console.log({ cs, ts, selectedAlgo });
    const selectedQ1 = queueInfo[0];
    const selectedQ2 = queueInfo[1];
    const selectedQ3 = queueInfo[2];
    const selectedQ4 = queueInfo[3];
    let pt;
    if (selectedPT == undefined) {
      pt = null;
    } else {
      pt = selectedPT;
    }

    console.log("what the shit");

    let isPreemptive = checkIFalgoIsPreemptive(selectedAlgo);
    if (isPreemptive) {
      if (!ts) {
        notify_this("Time Slice Is Required!", true);
        return false;
      }
    }

    if (cs < 1 || cs > 100000) {
      notify_this("Context Switch Must Be Between 1 and 100000", true);
      return false;
    }

    if (ts < 1 || ts > 100000) {
      if (isPreemptive) {
        notify_this("Time Slice Must Be Between 1 and 100000", true);
        return false;
      }
    }

    if (to_be_generated_processes.length == 0) {
      notify_this("No Process Specified!", true);
      return false;
    }

    if (!cs || !ts) {
      notify_this("Please Fill All Required Fields", true);
      return false;
    }

    console.log("selected algo when clicked: ", selectedAlgo);
    console.log("the fuck is ti: ", pt);
    setRunning(true);
    restartChart(!restart);
    handleDate(new Date());
    if (selectedAlgo == "MLQ" || selectedAlgo == "MLFQ") {
      await invoke("run_with_parameters", {
        arrayOfProcesses: to_be_generated_processes,
        queue: selectedAlgo,
        contextSwitch: parseFloat(cs),
        timeQuantum: parseFloat(ts),
        listOfDiscipline: [selectedQ1, selectedQ2, selectedQ3, selectedQ4],
        selectedProcessType: pt,
      });
    } else {
      await invoke("run_with_parameters", {
        arrayOfProcesses: to_be_generated_processes,
        queue: selectedAlgo,
        contextSwitch: parseFloat(cs),
        timeQuantum: parseFloat(ts),
        listOfDiscipline: null,
        selectedProcessType: pt,
      });
    }
  };
  const on_manual_add_click = (e) => {
    e.preventDefault();
    let at = refM_AT.current.value;
    let cbt = refM_CBT.current.value;
    at = parseInt(at);
    cbt = parseInt(cbt);
    let pt;
    if (selectedAlgo == "MLQ") {
      pt = Array.from(refMQ_PT.current.selectedOptions).map(
        (option) => option.value
      )[0];
      setSelectedPT(pt);
    } else {
      pt = null;
    }

    if (!at || !cbt) {
      notify_this("You Should Specify Inputs First!", true);
      return false;
    }

    if (at < 0 || at > 200000) {
      notify_this("Arrival Time Must Be Between 1 and 10000", true);
      return false;
    }

    if (cbt < 1 || cbt > 200000) {
      notify_this("CBT Must Be Between 1 and 10000", true);
      return false;
    }

    handleDate(new Date());

    notify_this(
      `Process With ${at}ms Arrival Time And ${cbt}ms CBT Is Added To The Queue`,
      false
    );

    setToBeGeneratedProcesses([...to_be_generated_processes, [at, cbt, pt]]);

    console.log(to_be_generated_processes);
  };

  const on_reset_click = (e) => {
    e.preventDefault();
    setRunning(false);
    setToBeGeneratedProcesses([]);
    setData([
      { subject: "TT" },
      { subject: "WT" },
      { subject: "RT" },
      { subject: "CU" },
    ]);
    restartChart(!restart);
    notify_this("Variables have been reset!", false);
  };

  const onNextClick = (e) => {
    e.preventDefault();

    const cs = parseInt(refMQ_CS.current.value);
    const ts = parseInt(refMQ_TS.current.value);

    const selectedQ1 = Array.from(refSelectQ1.current.selectedOptions).map(
      (option) => option.value
    )[0];
    const selectedQ2 = Array.from(refSelectQ2.current.selectedOptions).map(
      (option) => option.value
    )[0];
    const selectedQ3 = Array.from(refSelectQ3.current.selectedOptions).map(
      (option) => option.value
    )[0];
    const selectedQ4 = Array.from(refSelectQ4.current.selectedOptions).map(
      (option) => option.value
    )[0];

    if (cs < 1 || cs > 100000) {
      notify_this("Context Switch Must Be Between 1 and 100000", true);
      return false;
    }

    if (ts < 1 || ts > 100000) {
      if (isPreemptive) {
        notify_this("Time Slice Must Be Between 1 and 100000", true);
        return false;
      }
    }

    if (!cs || !ts) {
      notify_this("Please Fill All Required Fields", true);
      return false;
    }
    SetQueueInfo([selectedQ1, selectedQ2, selectedQ3, selectedQ4, ts, cs]);
    console.log([selectedQ1, selectedQ2, selectedQ3, selectedQ4, ts, cs]);
    setNextForm(false);
  };
  return (
    <div className={styles.main}>
      <div className={styles.inputs}>
        <div className={styles.header}>Inputs</div>
        <div
          className={styles.notif}
          style={
            notify
              ? isError
                ? { display: "flex", borderBottom: "3px solid #F04349" }
                : { display: "flex", borderBottom: "3px solid #01E17B" }
              : { display: "none" }
          }
        >
          <div
            className={styles.gradient}
            style={
              isError
                ? {
                    background:
                      "radial-gradient(circle,rgba(240, 66, 72, 0.38),rgba(240, 66, 72, 0))",
                  }
                : {
                    background: "radial-gradient(circle, #00ed5344, #00ed7a00)",
                  }
            }
          ></div>
          <img
            src={isError ? "./icons/icon2.svg" : "./icons/icon1.svg"}
            alt=""
          />
          <div style={{ marginLeft: 10 }}>
            <div className={styles.titleNotif}>
              {isError ? "Failed" : "Success"}
            </div>
            <div className={styles.info}>{notify ? notify : ""}</div>
          </div>
        </div>
        <div className={styles.tabs}>
          <button
            style={whichTab == 0 ? { backgroundColor: "#151E28" } : {}}
            onClick={() => setTab(0)}
          >
            Simulator
          </button>
          <button
            style={whichTab == 1 ? { backgroundColor: "#151E28" } : {}}
            onClick={() => setTab(1)}
          >
            Manual
          </button>
        </div>
        <div className={styles.tabs_container}>
          {whichTab == 0 ? (
            <form className={styles.form}>
              <div>
                <label htmlFor="at">ARRIVAL TIME λ:</label>
                <input id="at" ref={ref_AT} type="number" defaultValue={1} />
              </div>
              <div>
                <label htmlFor="cbt">CBT λ:</label>
                <input id="cbt" ref={ref_CBT} type="number" defaultValue={1} />
              </div>
              <div>
                <label htmlFor="np">Enter The Number Of Processes:</label>
                <input id="np" ref={ref_NP} type="number" defaultValue={10} />
              </div>
              <div>
                <label htmlFor="cs">Context Switch (ms):</label>
                <input id="cs" ref={ref_CS} type="number" defaultValue={1000} />
              </div>
              <div>
                <label htmlFor="ts">Time Slice (ms):</label>
                <input id="ts" ref={ref_TS} type="number" defaultValue={1000} />
              </div>
              <div>
                <input
                  id="start"
                  type="submit"
                  value="start"
                  onClick={on_start_click}
                />
              </div>
            </form>
          ) : (selectedAlgo == "MLQ" && nextForm) ||
            (selectedAlgo == "MLFQ" && nextForm) ? (
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
                <input id="csq" ref={refMQ_CS} type="number" defaultValue={1} />
              </div>
              <div>
                <label htmlFor="tmq">Time Slice:</label>
                <input id="tmq" ref={refMQ_TS} type="number" defaultValue={1} />
              </div>
              <div>
                <input id="" type="submit" value="next" onClick={onNextClick} />
              </div>
            </form>
          ) : (
            <form className={styles.form_manual}>
              <div>
                <label htmlFor="atm">ARRIVAL TIME (ms):</label>
                <input id="atm" ref={refM_AT} type="number" defaultValue={1} />
              </div>
              <div>
                <label htmlFor="cbtm">CBT (ms):</label>
                <input
                  id="cbtm"
                  ref={refM_CBT}
                  type="number"
                  defaultValue={1}
                />
              </div>
              {selectedAlgo == "MLQ" ? (
                <div style={{ gridArea: "2 / 1 / 2 / 3" }}>
                  <label htmlFor="pt">Process Type:</label>
                  <select name="pt" id="pt" ref={refMQ_PT}>
                    <option value="system">System Process</option>
                    <option value="interactive">Interactive Process</option>
                    <option value="batch">Batch Process</option>
                    <option value="student">Student Process</option>
                  </select>
                </div>
              ) : null}
              {selectedAlgo == "MLQ" || selectedAlgo == "MLFQ" ? null : (
                <>
                  <div>
                    <label htmlFor="csm">Context Switch (ms):</label>
                    <input
                      id="csm"
                      ref={refM_CS}
                      type="number"
                      defaultValue={1000}
                    />
                  </div>
                  <div>
                    <label htmlFor="tsm">Time Slice (ms):</label>
                    <input
                      id="tsm"
                      ref={refM_TS}
                      type="number"
                      defaultValue={1000}
                    />
                  </div>
                </>
              )}
              <div>
                <input
                  id="Addm"
                  type="submit"
                  value="Add"
                  onClick={on_manual_add_click}
                />
              </div>
              <div>
                <input
                  id="stopm"
                  type="submit"
                  value="reset"
                  onClick={on_reset_click}
                />
              </div>
              <div>
                <input
                  id="startm"
                  type="submit"
                  value="start"
                  onClick={on_manual_start_click}
                />
              </div>
              {selectedAlgo == "MLQ" || selectedAlgo == "MLFQ" ? (
                <div>
                  <input
                    id="stopm"
                    type="submit"
                    value="back"
                    onClick={(e) => {
                      e.preventDefault();
                      setNextForm(true);
                    }}
                  />
                </div>
              ) : null}
            </form>
          )}
        </div>
      </div>
      <div className={styles.metrics}>
        <div className={styles.header}>Metrics</div>
        {chartData[0]["TT"] == undefined ? (
          <p
            style={{
              marginTop: 10,
              marginLeft: "auto",
              marginRight: "auto",
              color: "rgb(106, 110, 122)",
            }}
          >
            No Data Yet
          </p>
        ) : (
          <Splide
            className={styles.splide}
            options={{
              pagination: false,
            }}
            hasTrack={false}
          >
            <SplideTrack className={styles.tracker}>
              <SplideSlide className={styles.slide}>
                <ResponsiveContainer
                  width="90%"
                  height="100%"
                  style={{ marginRight: "40px" }}
                >
                  <BarChart data={chartData}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="name" />
                    <YAxis />
                    <Tooltip />
                    <Legend />
                    <Bar
                      dataKey="WT"
                      fill="#FF5722"
                      activeBar={<Rectangle fill="pink" />}
                    />
                    <Bar
                      dataKey="TT"
                      fill="#8884d8"
                      activeBar={<Rectangle fill="pink" />}
                    />
                    <Bar
                      dataKey="RT"
                      fill="#3FC1C9"
                      activeBar={<Rectangle fill="pink" />}
                    />
                    <Bar
                      dataKey="CU"
                      fill="#B8DE6F"
                      activeBar={<Rectangle fill="pink" />}
                    />
                  </BarChart>
                </ResponsiveContainer>
              </SplideSlide>
              <SplideSlide className={styles.slider}>
                <div className={styles.selectWhichX}>
                  <div
                    style={{ display: "flex", alignItems: "center" }}
                    onClick={() => {
                      setMeow("WT");
                    }}
                  >
                    <div
                      style={{
                        backgroundColor: "#FF5722",
                        width: "10px",
                        height: "10px",
                        marginRight: "10px",
                        border: "white 2px solid",
                      }}
                    ></div>
                    WT
                  </div>
                  <div
                    style={{ display: "flex", alignItems: "center" }}
                    onClick={() => {
                      setMeow("TT");
                    }}
                  >
                    <div
                      style={{
                        backgroundColor: "#8884d8",
                        width: "10px",
                        height: "10px",
                        marginRight: "10px",
                        border: "white 2px solid",
                      }}
                    ></div>
                    TT
                  </div>
                  <div
                    style={{ display: "flex", alignItems: "center" }}
                    onClick={() => {
                      setMeow("RT");
                    }}
                  >
                    <div
                      style={{
                        backgroundColor: "#3FC1C9",
                        width: "10px",
                        height: "10px",
                        marginRight: "10px",
                        border: "white 2px solid",
                      }}
                    ></div>
                    RT
                  </div>
                  <div
                    style={{ display: "flex", alignItems: "center" }}
                    onClick={() => {
                      setMeow("CU");
                    }}
                  >
                    <div
                      style={{
                        backgroundColor: "#B8DE6F",
                        width: "10px",
                        height: "10px",
                        marginRight: "10px",
                        border: "white 2px solid",
                      }}
                    ></div>
                    CU
                  </div>
                </div>
                <ResponsiveContainer
                  width="90%"
                  height="100%"
                  style={{ marginRight: "40px" }}
                >
                  <BarChart data={multiChart}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="name" fontSize="9px" dx="5px" />
                    <YAxis />
                    <Tooltip />
                    <Legend />
                    {whichMeow == "WT" && (
                      <Bar
                        dataKey="WT"
                        fill="#FF5722"
                        activeBar={<Rectangle fill="pink" />}
                      />
                    )}
                    {whichMeow == "TT" && (
                      <Bar
                        dataKey="TT"
                        fill="#8884d8"
                        activeBar={<Rectangle fill="pink" />}
                      />
                    )}
                    {whichMeow == "RT" && (
                      <Bar
                        dataKey="RT"
                        fill="#3FC1C9"
                        activeBar={<Rectangle fill="pink" />}
                      />
                    )}
                    {whichMeow == "CU" && (
                      <Bar
                        dataKey="CU"
                        fill="#B8DE6F"
                        activeBar={<Rectangle fill="pink" />}
                      />
                    )}
                  </BarChart>
                </ResponsiveContainer>
              </SplideSlide>
            </SplideTrack>
          </Splide>
        )}
      </div>
    </div>
  );
}
