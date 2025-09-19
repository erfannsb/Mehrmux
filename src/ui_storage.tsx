import { create } from "zustand";

export interface Time {
  secs: number;
  nanos: number;
}

export interface Metrics {
  response_time: Time;
  total_waiting_time: Time;
  total_time: Time;
}

export interface Process {
  id: string;
  arrival_time: string; // ISO string
  cpu_burst_time: Time;
  status: string;
  waiting_time: Time;
  processed_time: Time;
  process_type: string;
  last_execution: string; // ISO string
  metrics: Metrics;
}

interface StoreState {
  selectedAlgo: string;
  setSelectedAlgo: (algo: string) => void;

  restartChart: boolean;
  setRestartChart: (value: boolean) => void;

  processes: Process[];
  setProcesses: (value: Process[]) => void;

  startingDate: Date | null;
  setStartingDate: (value: Date | null) => void;
}

const useStore = create<StoreState>((set) => ({
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
