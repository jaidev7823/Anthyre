import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
export async function syncCalendar() {
  console.log('Tauri: sync_calendar invoked');
  // Uncomment when Tauri is set up:
  // import { invoke } from '@tauri-apps/api/tauri';
  // await invoke('sync_calendar');
}