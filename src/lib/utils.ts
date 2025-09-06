import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"
import { invoke } from "@tauri-apps/api/core";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export async function syncCalendar() {
  try {
    const result = await invoke("login_with_google");
    console.log("Auth result:", result);
  } catch (err) {
    console.error("Login failed:", err);
  }
}
