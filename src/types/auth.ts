import { create } from "zustand";

interface AuthState {
  isLoggedIn: boolean;
  lastChecked: number | null;
  setLoggedIn: (status: boolean) => void;
  setLastChecked: (ts: number) => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  isLoggedIn: false,
  lastChecked: null,
  setLoggedIn: (status) => set({ isLoggedIn: status }),
  setLastChecked: (ts) => set({ lastChecked: ts }),
}));
