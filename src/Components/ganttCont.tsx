import { useEffect, useState, useRef } from "react";
import styles from "./../styles/ganttchart.module.css";
import ChromeDinoGame from "react-chrome-dino";
import { listen } from "@tauri-apps/api/event";
import useStore from "../ui_storage.tsx";
import * as d3 from "d3";
import { Process, Time } from "../ui_storage.tsx";

// Type Interface for Incomming Data

interface ProcessStoppedEvent {
  event: String; // type of event
  payload: [
    number, // self.queue_number (It's the number of  queue)
    Process
  ];
  id: number;
}

interface FinishedProcessEvent {
  event: String; // type of event
  payload: [Process];
  id: number;
}

interface ChartDataType {
  name: string;
  start: Date;
  end: Date;
}

function getContrastColor(hex: String) {
  // Convert hex to RGB
  const r = parseInt(hex.substr(1, 2), 16);
  const g = parseInt(hex.substr(3, 2), 16);
  const b = parseInt(hex.substr(5, 2), 16);

  // Compute relative luminance (0 = dark, 255 = bright)
  const luminance = 0.299 * r + 0.587 * g + 0.114 * b;

  // Return black for bright colors, white for dark colors
  return luminance > 180 ? "#000" : "#fff";
}

export default function GanttCont() {
  const processes = useStore((state) => state.processes);
  const restartChart = useStore((state) => state.restartChart);
  const firstDate: Date | null = useStore((state) => state.startingDate);
  const selectedAlgo = useStore((state) => state.selectedAlgo);
  const [processEvent, setProcessEvent] = useState<[number, Process] | []>([]);
  const [finishedP, setFinishedP] = useState<Process[]>([]);
  const [rows, setRows] = useState<ChartDataType[]>([]);
  const [rows1, setRows1] = useState([]);
  const [rows2, setRows2] = useState([]);
  const [rows3, setRows3] = useState([]);
  const [rows4, setRows4] = useState([]);
  const [passed_times_for_all, setPTFA] = useState<Record<string, number>>({});
  const ChartContRef = useRef<HTMLDivElement | null>(null);

  // Getting Data From Backend -------------------------------------------------------------------------

  useEffect(() => {
    const unlistenPS = listen(
      "process_stopped",
      (event: ProcessStoppedEvent) => {
        setProcessEvent(event.payload);
      }
    );

    const unlistenPF = listen(
      "finished_process",
      (event: FinishedProcessEvent) => {
        setFinishedP((prev) =>
          selectedAlgo === "MLFQ" || selectedAlgo === "MLQ"
            ? [...prev, ...event.payload]
            : [...event.payload]
        );
      }
    );

    return () => {
      unlistenPF.then((fn) => fn());
      unlistenPS.then((fn) => fn());
    };
  }, []);

  // Restarting Chart and Variables --------------------------------------------------------------------

  useEffect(() => {
    setRows([]); // Reset rows
    setRows1([]);
    setRows2([]);
    setRows3([]);
    setRows4([]);
    setFinishedP([]); // Reset finished processes
  }, [restartChart]); // Trigger effect when `restartChart` changes

  function calculateTimeWithDate(date: string): number {
    // turn incoming string into a Date
    const newDate = new Date(date);

    if (!firstDate) {
      return 0;
    }

    // ensure both are numbers (timestamps in ms)
    const firstTime = new Date(firstDate).getTime();
    const newTime = newDate.getTime();

    // return difference in seconds
    return Math.abs(firstTime - newTime) / 1000;
  }

  const columns = [
    { type: "string", id: "Process" },
    { type: "string", id: "Name" },
    { type: "date", id: "Start" },
    { type: "date", id: "End" },
  ];

  // PreProcessing Incomming Data ----------------------------------------------------------------------

  useEffect(() => {
    const pE = processEvent[1]; // Process Event
    const numOfQ = processEvent[0]; // Number of Queue

    if (!pE || !firstDate) return;

    const keyNameForPassedTimes: string = pE.id.slice(0, 8);

    // Convert last_execution and processed_time to milliseconds
    const lastExecutionTime = new Date(pE.last_execution).getTime();

    const processedMs =
      pE.processed_time.secs * 1000 + pE.processed_time.nanos / 1e6;

    // Determine passed_time for this process
    let passedTime: number;

    if (passed_times_for_all[keyNameForPassedTimes] == undefined) {
      passedTime = processedMs;
      setPTFA({
        ...passed_times_for_all,
        [keyNameForPassedTimes]: passedTime,
      });
    } else {
      passedTime = processedMs - passed_times_for_all[keyNameForPassedTimes];
      setPTFA({
        ...passed_times_for_all,
        [keyNameForPassedTimes]:
          passed_times_for_all[keyNameForPassedTimes] + passedTime,
      });
    }

    // Compute start and end times relative to firstDate
    const firstTime = new Date(firstDate).getTime();

    const start_ms = lastExecutionTime - firstTime;
    const end_ms = start_ms + processedMs;

    const newData: ChartDataType = {
      name: keyNameForPassedTimes,
      start: new Date(firstTime + start_ms),
      end: new Date(firstTime + end_ms),
    };

    console.log({ newData });

    // Add to the rows for chart
    setRows((prev) => [...prev, newData]);
  }, [processEvent]);

  // Building The Chart: --------------------------------------------------------------

  const data = [
    {
      name: "P1",
      start: new Date(2025, 8, 14, 9, 0),
      end: new Date(2025, 8, 14, 11, 0),
    },
    {
      name: "P2",
      start: new Date(2025, 8, 14, 10, 0),
      end: new Date(2025, 8, 14, 12, 0),
    },
    {
      name: "P3",
      start: new Date(2025, 8, 14, 11, 30),
      end: new Date(2025, 8, 14, 14, 0),
    },
    {
      name: "P4",
      start: new Date(2025, 8, 14, 11, 30),
      end: new Date(2025, 8, 14, 14, 0),
    },
    {
      name: "P5",
      start: new Date(2025, 8, 14, 11, 30),
      end: new Date(2025, 8, 14, 14, 0),
    },
    {
      name: "P6",
      start: new Date(2025, 8, 14, 11, 30),
      end: new Date(2025, 8, 14, 14, 0),
    },

    {
      name: "P7",
      start: new Date(2025, 8, 14, 11, 30),
      end: new Date(2025, 8, 14, 14, 0),
    },
    {
      name: "P8",
      start: new Date(2025, 8, 14, 11, 30),
      end: new Date(2025, 8, 14, 14, 0),
    },
    {
      name: "P9",
      start: new Date(2025, 8, 14, 11, 30),
      end: new Date(2025, 8, 14, 14, 0),
    },
  ];

  const svgRef = useRef<SVGSVGElement | null>(null);

  const [chartSize, setChartSize] = useState({ width: 0, height: 0 });

  useEffect(() => {
    if (!ChartContRef.current) return;

    // Only set once
    if (chartSize.width === 0 && chartSize.height === 0) {
      const { width, height } = ChartContRef.current.getBoundingClientRect();
      setChartSize({ width, height });
    }
  }, [ChartContRef]);

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
    "#355C7D", // Slate Blue
  ];

  useEffect(() => {
    const data = rows;
    if (!data || data.length === 0) return;

    const drawChart = () => {
      const width = ChartContRef.current?.clientWidth;
      const height = ChartContRef.current?.clientHeight;
      if (!width || !height) return;

      const svg = d3.select(svgRef.current);
      svg.selectAll("*").remove(); // clear previous

      const margin = { top: 20, right: 20, bottom: 40, left: 50 }; // left margin increased for labels
      const rowPadding = 15;

      // x scale
      const x = d3
        .scaleTime()
        .domain([d3.min(data, (d) => d.start)!, d3.max(data, (d) => d.end)!])
        .range([margin.left, width - margin.right]);

      // y scale
      const y = d3
        .scaleBand()
        .domain(data.map((d) => d.name))
        .range([margin.top, height - margin.bottom])
        .padding(0.1);

      const tooltip = d3.select("#tooltip");

      const color: d3.ScaleOrdinal<string, string> = d3
        .scaleOrdinal<string, string>()
        .domain(data.map((d) => d.name))
        .range(colors);

      // 1️⃣ Background rows
      svg
        .selectAll("row-bg")
        .data(data)
        .join("rect")
        .attr("x", margin.left)
        .attr("y", (d) => y(d.name)!)
        .attr("width", width - margin.left - margin.right)
        .attr("height", y.bandwidth())
        .attr("fill", (d, i) => (i % 2 === 0 ? "#0D1321" : "#0D111D"))
        .lower();

      // 2️⃣ Grid lines
      svg
        .append("g")
        .attr("transform", `translate(0,${height - margin.bottom})`)
        .call(
          d3
            .axisBottom(x)
            .ticks(10)
            .tickSize(-(height - margin.top - margin.bottom))
        )
        .selectAll("line")
        .attr("stroke", "#1B1E2D");

      svg
        .append("g")
        .selectAll("line")
        .data(y.domain())
        .join("line")
        .attr("x1", margin.left)
        .attr("x2", width - margin.right)
        .attr("y1", (d) => y(d)! + y.bandwidth() / 2)
        .attr("y2", (d) => y(d)! + y.bandwidth() / 2)
        .attr("stroke", "#1B1E2D")
        .attr("stroke-dasharray", "2,2");

      // 3️⃣ Bars
      svg
        .selectAll("rect.bar")
        .data(data)
        .join("rect")
        .attr("class", "bar")
        .attr("x", (d) => x(d.start))
        .attr("y", (d) => y(d.name)! + rowPadding / 2)
        .attr("width", (d) => x(d.end) - x(d.start))
        .attr("height", y.bandwidth() - rowPadding)
        .attr("fill", (d) => color(d.name))
        .attr("rx", 4)
        .attr("ry", 4)
        .on("mouseover", (event, d) => {
          const startMs = d.start.getTime() - firstDate!.getTime(); // difference in ms
          const endMs = d.end.getTime() - firstDate!.getTime();

          const formatTime = (ms: number) => {
            const seconds = Math.floor(ms / 1000);
            const millis = ms % 1000;
            return `${seconds}s ${millis}ms`;
          };

          tooltip.style("opacity", 1).html(
            `<strong>${d.name}</strong><br/>
       Start: ${formatTime(startMs)}<br/>
       End: ${formatTime(endMs)}`
          );
        })
        .on("mousemove", (event) => {
          tooltip
            .style("left", event.offsetX + 15 + "px")
            .style("top", event.offsetY + "px");
        })
        .on("mouseout", () => {
          tooltip.style("opacity", 0);
        });

      // 4️⃣ Labels on top
      svg
        .selectAll("text.label")
        .data(data)
        .join("text")
        .attr("class", "label")
        .text((d) => `Process: ${d.name}`)
        .attr("font-size", 10)
        .attr("y", (d) => y(d.name)! + y.bandwidth() / 2)
        .attr("dy", "0.35em")
        .attr("fill", (d) => {
          const barWidth = x(d.end) - x(d.start);
          const tempText = svg
            .append("text")
            .text(`Process: ${d.name}`)
            .attr("font-size", 10);
          const labelWidth = tempText.node()!.getBBox().width;
          tempText.remove();
          return labelWidth > barWidth
            ? "#fff"
            : getContrastColor(color(d.name));
        })
        .attr("x", (d) => {
          const barWidth = x(d.end) - x(d.start);
          const tempText = svg
            .append("text")
            .text(`Process: ${d.name}`)
            .attr("font-size", 10);
          const labelWidth = tempText.node()!.getBBox().width;
          tempText.remove();

          if (labelWidth > barWidth) {
            // Outside bar on the left
            const pos = x(d.start) - labelWidth - 5;
            return pos < margin.left ? margin.left : pos;
          } else {
            // Inside bar
            return x(d.start) + 5;
          }
        });

      // Axes styling
      svg.selectAll(".domain, .tick line").attr("stroke", "#444");
      svg.selectAll(".tick text").attr("fill", "#aaa");
    };

    drawChart();

    const handleResize = () => drawChart();
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, [rows]);

  // ---------------------------------------------------------------------------

  //   const data = [columns, ...rows];
  const data1 = [columns, ...rows1];
  const data2 = [columns, ...rows2];
  const data3 = [columns, ...rows3];
  const data4 = [columns, ...rows4];

  return (
    <div className={styles.main}>
      <div className={styles.gantt}>
        <div className={styles.gantt_head}>
          <div className={styles.title}>Simulating CPU Scheduler</div>
          <div className={styles.info}></div>
        </div>
        <div className={styles.gantt_main} ref={ChartContRef}>
          <div id="tooltip" className={styles.tooltip}></div>
          <svg ref={svgRef} width="100%" height="100%"></svg>
        </div>
        <div className={styles.gantt_footer}>
          <ChromeDinoGame className="gantt_footer_game" />
        </div>
      </div>
      <div className={styles.readyQueue}>
        <div className={styles.title}>Ready Queue</div>
        {processes.length > 0 &&
          processes.map((process, index) => (
            <ProcessElement process={process} color={colors[index % 20]} />
          ))}
      </div>
      <div className={styles.finished}>
        <div className={styles.title}>Finished Processes</div>
        {finishedP.length > 0 &&
          finishedP.map((process, index) => (
            <ProcessElement process={process} color={colors[index % 20]} />
          ))}
      </div>
    </div>
  );
}

interface ProcessElementType {
  process: Process;
  color: string;
}

export function ProcessElement({ process, color }: ProcessElementType) {
  function turnToNormalDuration(duration: Time) {
    let secs_to_millis = duration.secs * 1000;
    let nanos_to_millis = duration.nanos / 10e6;
    return (secs_to_millis + nanos_to_millis).toFixed(3);
  }

  return (
    <div className={styles.process_card_in_queue}>
      <h3 style={{ color: color }}>Process {process.id}</h3>
      <p>AT: {process.arrival_time}ms</p>
      <p>CBT: {turnToNormalDuration(process.cpu_burst_time)}</p>
      <p>STATUS: {process.status}</p>
    </div>
  );
}
