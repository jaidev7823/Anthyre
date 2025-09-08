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

export async function daily_summary() {
  try {
    const result = await invoke("get_daily_summary");
    console.log("daily summary:", result);
    return result;
  } catch (err) {
    console.error("can't fetch daily summary:", err);
    return null;
  }
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

export async function updateHoursRange(
  startISO: string,
  endISO: string,
  meta?: { date?: string; start?: string; end?: string }
) {
  try {
    const result = await invoke("update_hours_range", {
      args: {
        startIso: startISO,
        endIso: endISO,
        ...(meta || {}),
      },
    });
    console.log("Range updated successfully:", result);
    return result;
  } catch (err) {
    console.error("Range update failed:", err);
    return null;
  }
}

