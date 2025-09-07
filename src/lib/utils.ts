import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import { invoke } from "@tauri-apps/api/core";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
// Sync calendar (requires login)
export async function syncCalendar() {
  try {
    // Here you might call another Rust command, for now just reuse login
    const result = await invoke("login_with_google");
    console.log("Sync calendar result:", result);
    return result;
  } catch (err) {
    console.error("Sync calendar failed:", err);
    return null;
  }
}

export async function init(): Promise<boolean> {
  return await invoke<boolean>("check_calendar_token");
}

export async function updateHours() {
  try {
    // Here you might call another Rust command, for now just reuse login
    const result = await invoke("update_hours");
    console.log("Hour Updated successfully:", result);
    return result;
  } catch (err) {
    console.error("Hour Updated successfully failed:", err);
    return null;
  }
}
