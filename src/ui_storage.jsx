import { create } from "zustand";

const useStore = create((set) => ({
  selectedAlgo: "FCFS",
  setSelectedAlgo: (algo) => set({ selectedAlgo: algo }),
  restartChart: false,
  setRestartChart: (value) => set({ restartChart: value }),
  processes: [],
  setProcesses: (value) => set({ processes: value }),
  startingDate: null,
  setStartingDate: (value) => set({ startingDate: value }),
}));

export default useStore;
