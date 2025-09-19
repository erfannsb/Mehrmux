import useStore from "./../ui_storage";
import styles from "./../styles/algoChoose.module.css";
import { useEffect, useRef } from "react";
export default function AlgoChoose() {
  const setSelectedAlgo = useStore((state) => state.setSelectedAlgo);
  const algo = useStore((state) => state.selectedAlgo);
  const refsB = useRef<(HTMLButtonElement | null)[]>([]);
  const refA = useRef<HTMLDivElement | null>(null); // Container Div

  const handleResize = () => {
    refsB.current.forEach((ref) => {
      if (ref) {
        let h = refA.current?.clientHeight;
        if (!h) return;
        h -= 65;
        ref.style.height = h / 7 + "px";
      }
    });
  };

  useEffect(() => {
    // Add event listener for window resize
    refsB.current.forEach((ref, index) => {
      if (ref) {
        let h = refA.current?.clientHeight;
        if (!h) return;
        h -= 65;
        ref.style.height = h / 7 + "px";
      }
    });
    window.addEventListener("resize", handleResize);
  }, []);

  const buttonList = [
    { image: "./queue_icons/Stairs.svg", name: "FCFS", value: "FCFS" },
    { image: "./queue_icons/Sharp Cone.svg", name: "SPN", value: "SPN" },
    { image: "./queue_icons/3 Cube.svg", name: "SJF", value: "SJF" },
    { image: "./queue_icons/8Star.svg", name: "HRRN", value: "HRRN" },
    {
      image: "./queue_icons/Sharp Rectangle with Circle.svg",
      name: "RR",
      value: "RR",
    },
    { image: "./queue_icons/Torus Knot 5.svg", name: "MLQ", value: "MLQ" },
    { image: "./queue_icons/Sphere 7.svg", name: "MLFQ", value: "MLFQ" },
    { image: "./queue_icons/Solid Bar.svg", name: "FIFO", value: "FIFO" },
    { image: "./queue_icons/Plus.svg", name: "SRTF", value: "SRTF" },
  ];

  const on_click = (value: string) => {
    setSelectedAlgo(value);
  };

  return (
    <div className={styles.main} ref={refA}>
      {buttonList.map((element, index) => (
        <button
          key={index}
          className={
            algo === element.value
              ? `${styles.active_button} ${styles.buttons}`
              : styles.buttons
          }
          ref={(el) => {
            refsB.current[index] = el; // assign to array
          }}
          onClick={() => on_click(element.value)}
        >
          <img src={element.image} alt="img" />
          <span>{element.name}</span>
        </button>
      ))}
    </div>
  );
}
