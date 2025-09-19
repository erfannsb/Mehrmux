import { useEffect, useState, useRef } from "react";
import styles from "./../styles/ganttchart.module.css";
import ChromeDinoGame from "react-chrome-dino";
import { listen } from "@tauri-apps/api/event";
import useStore from "./../ui_storage.jsx";
import * as d3 from "d3";

function getContrastColor(hex) {
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
  const firstDate = useStore((state) => state.startingDate);
  const selectedAlgo = useStore((state) => state.selectedAlgo);
  const [processEvent, setProcessEvent] = useState([]);
  const [finishedP, setFinishedP] = useState([]);
  const [rows, setRows] = useState([]);
  const [rows1, setRows1] = useState([]);
  const [rows2, setRows2] = useState([]);
  const [rows3, setRows3] = useState([]);
  const [rows4, setRows4] = useState([]);
  let [passed_times_for_all, setPTFA] = useState({});
  const ChartContRef = useRef(null);

  useEffect(() => {
    setRows([]); // Reset rows
    setRows1([]);
    setRows2([]);
    setRows3([]);
    setRows4([]);
    setFinishedP([]); // Reset finished processes
  }, [restartChart]); // Trigger effect when `restartChart` changes

  useEffect(() => {
    const unlistenPS = listen("process_stopped", (event) => {
      setProcessEvent(event.payload);
    });

    const unlistenPF = listen("finished_process", (event) => {
      setFinishedP((prev) =>
        selectedAlgo === "MLFQ" || selectedAlgo === "MLQ"
          ? [...prev, ...event.payload]
          : [...event.payload]
      );
    });

    return () => {
      unlistenPF.then((fn) => fn());
      unlistenPS.then((fn) => fn());
    };
  }, []);

  function calculateTimeWithDate(date) {
    // calculates how much time passed from the first date in seconds
    date = new Date(date);
    if (firstDate == null) {
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

  useEffect(() => {
    let pE = processEvent[1];
    let numOfQ = processEvent[0];
    if (pE == undefined || pE.length == 0) {
      return;
    }
    let keyNameForPassedTimes = pE.id.slice(0, 8);
    const last_execution = calculateTimeWithDate(pE.last_execution);
    const start_time = new Date(0, 0, 0, 0, 0, 0, last_execution);
    let passed_time;
    if (passed_times_for_all[keyNameForPassedTimes] == undefined) {
      passed_time = pE.processed_time.secs + pE.processed_time.nanos / 1e9;
      passed_time = passed_time * 1000;
      let ptfa = { ...passed_times_for_all };
      ptfa[keyNameForPassedTimes] = passed_time;
      setPTFA({ ...passed_times_for_all, ...ptfa });
    } else {
      passed_time = pE.processed_time.secs + pE.processed_time.nanos / 1e9;
      passed_time = passed_time * 1000;
      passed_time = passed_time - passed_times_for_all[keyNameForPassedTimes];
      let ptfa = { ...passed_times_for_all };
      ptfa[keyNameForPassedTimes] += passed_time;
      setPTFA({ ...passed_times_for_all, ...ptfa });
    }

    let end_time = new Date(0, 0, 0, 0, 0, 0, last_execution + passed_time);
    if (selectedAlgo == "MLQ" || selectedAlgo == "MLFQ") {
      switch (numOfQ) {
        case 1:
          setRows1([
            ...rows1,
            [
              pE.id.slice(0, 8),
              `Process: ${pE.id.slice(0, 8)}`,
              start_time,
              end_time,
            ],
          ]);
          break;
        case 2:
          setRows2([
            ...rows2,
            [
              pE.id.slice(0, 8),
              `Process: ${pE.id.slice(0, 8)}`,
              start_time,
              end_time,
            ],
          ]);
          break;
        case 3:
          setRows3([
            ...rows3,
            [
              pE.id.slice(0, 8),
              `Process: ${pE.id.slice(0, 8)}`,
              start_time,
              end_time,
            ],
          ]);
          break;
        case 4:
          setRows4([
            ...rows4,
            [
              pE.id.slice(0, 8),
              `Process: ${pE.id.slice(0, 8)}`,
              start_time,
              end_time,
            ],
          ]);
      }
    } else {
      setRows([
        ...rows,
        [
          pE.id.slice(0, 8),
          `Process: ${pE.id.slice(0, 8)}`,
          start_time,
          end_time,
        ],
      ]);
    }
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

  const svgRef = useRef();

  const [chartSize, setChartSize] = useState({ width: 0, height: 0 });

  useEffect(() => {
    if (!ChartContRef.current) return;

    // Only set once
    if (chartSize.width === 0 && chartSize.height === 0) {
      const { width, height } = ChartContRef.current.getBoundingClientRect();
      setChartSize({ width, height });
    }
  }, [ChartContRef]);

  useEffect(() => {
    if (!data || data.length === 0) return;

    const drawChart = () => {
      const width = ChartContRef.current.clientWidth;
      const height = ChartContRef.current.clientHeight;

      const svg = d3.select(svgRef.current);
      svg.selectAll("*").remove(); // clear previous

      const margin = { top: 20, right: 20, bottom: 40, left: 20 };

      const x = d3
        .scaleTime()
        .domain([d3.min(data, (d) => d.start), d3.max(data, (d) => d.end)])
        .range([margin.left, width - margin.right]);

      const y = d3
        .scaleBand()
        .domain(data.map((d) => d.name))
        .range([margin.top, height - margin.bottom])
        .padding(0.1);

      const tooltip = d3.select("#tooltip");
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

      const color = d3
        .scaleOrdinal()
        .domain(data.map((d) => d.name)) // map task names to colors
        .range(colors);

      // Bars

      const rowPadding = 15;

      svg
        .selectAll("rect")
        .data(data)
        .join("rect")
        .attr("x", (d) => x(d.start))
        .attr("y", (d) => y(d.name) + rowPadding / 2)
        .attr("width", (d) => x(d.end) - x(d.start))
        .attr("height", y.bandwidth() - rowPadding)
        .attr("fill", (d) => color(d.name))
        .attr("rx", 4) // horizontal border radius
        .attr("ry", 4) // vertical border radius
        .on("mouseover", (event, d) => {
          tooltip.style("opacity", 1).html(`
      <strong>${d.name}</strong><br/>
      Start: ${d.start}<br/>
      End: ${d.end}
    `);
        })
        .on("mousemove", (event) => {
          tooltip
            .style("left", event.offsetX + 15 + "px")
            .style("top", event.offsetY + "px");
        })
        .on("mouseout", () => {
          tooltip.style("opacity", 0);
        });

      svg
        .selectAll("text.label")
        .data(data)
        .join("text")
        .attr("class", "label")
        .attr("x", (d) => x(d.start) + 5)
        .attr("y", (d) => y(d.name) + y.bandwidth() / 2)
        .attr("dy", "0.35em")
        .text((d) => "Process: " + d.name)
        .attr("fill", (d) => getContrastColor(color(d.name)))
        .attr("font-size", 10);

      svg.selectAll(".domain, .tick line").attr("stroke", "#444");

      // Axes

      // Add horizontal + vertical grid lines for clarity:

      svg
        .append("g")
        .attr("class", "grid")
        .attr("transform", `translate(0,${height - margin.bottom})`)
        .call(
          d3
            .axisBottom(x)
            .ticks(10)
            .tickSize(-(height - margin.top - margin.bottom))
        )
        .selectAll("line")
        .attr("stroke", "#1B1E2D"); // subtle grid

      svg
        .append("g")
        .attr("class", "y-grid")
        .selectAll("line")
        .data(y.domain())
        .join("line")
        .attr("x1", margin.left)
        .attr("x2", width - margin.right)
        .attr("y1", (d, i) => {
          // place line between bands
          if (i === 0) return y(d) - (y.padding() * y.bandwidth()) / 2;
          return y(d) - (y.padding() * y.bandwidth()) / 2;
        })
        .attr("y2", (d, i) => {
          if (i === 0) return y(d) - (y.padding() * y.bandwidth()) / 2;
          return y(d) - (y.padding() * y.bandwidth()) / 2;
        })
        .attr("stroke", "#1B1E2D")
        .attr("stroke-dasharray", "2,2");

      svg.selectAll(".tick text").attr("fill", "#aaa");

      svg
        .selectAll("row-bg")
        .data(data)
        .join("rect")
        .attr("x", margin.left)
        .attr("y", (d, i) => y(d.name))
        .attr("width", width - margin.left - margin.right)
        .attr("height", y.bandwidth())
        .attr("fill", (d, i) => (i % 2 === 0 ? "#0D1321" : "#0D111D")) // even rows darker
        .lower(); // ensures background is behind bars
    };

    drawChart(); // initial draw
    const handleResize = () => {
      drawChart(); // redraw on resize
    };

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, [data]);

  // ---------------------------------------------------------------------------

  //   const data = [columns, ...rows];
  const data1 = [columns, ...rows1];
  const data2 = [columns, ...rows2];
  const data3 = [columns, ...rows3];
  const data4 = [columns, ...rows4];

  function turnToNormalDuration(duration) {
    let secs_to_millis = duration.secs * 1000;
    let nanos_to_millis = duration.nanos / 10e6;
    return (secs_to_millis + nanos_to_millis).toFixed(3);
  }

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
            <ProcessElement
              id={process.id.toString().slice(0, 8)}
              AT={calculateTimeWithDate(process.arrival_time)}
              STATUS={process.status}
              CBT={`${turnToNormalDuration(process.cpu_burst_time)}ms`}
              color={colors[index % 20]}
            />
          ))}
      </div>
      <div className={styles.finished}>
        <div className={styles.title}>Finished Processes</div>
        {finishedP.length > 0 &&
          finishedP.map((process, index) => (
            <ProcessElement
              id={process.id.toString().slice(0, 8)}
              AT={calculateTimeWithDate(process.arrival_time)}
              STATUS={process.status}
              CBT={`${turnToNormalDuration(process.cpu_burst_time)}ms`}
              color={colors[index % 20]}
            />
          ))}
      </div>
    </div>
  );
}

export function ProcessElement({ id, AT, STATUS, CBT, color }) {
  return (
    <div className={styles.process_card_in_queue}>
      <h3 style={{ color: color }}>Process {id}</h3>
      <p>AT: {AT}ms</p>
      <p>CBT: {CBT}</p>
      <p>STATUS: {STATUS}</p>
    </div>
  );
}
