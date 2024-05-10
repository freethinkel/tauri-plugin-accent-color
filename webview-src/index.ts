import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

type Callback<T> = (value: T) => void;

class Store<T> {
  private value!: T;
  private listeners: Array<Callback<T>> = [];
  constructor(initialValue?: T) {
    if (initialValue) {
      this.value = initialValue;
    }
  }
  private notifyListeners() {
    this.listeners.forEach((callback) => callback(this.value));
  }
  set(value: T) {
    this.value = value;
    this.notifyListeners();
  }
  get(): T {
    return this.value;
  }
  update(callback: (value: T) => T) {
    this.set(callback(this.value));
  }
  subscribe(callback: Callback<T>): () => void {
    this.listeners.push(callback);
    callback(this.value);

    return () => {
      this.listeners = this.listeners.filter(
        (listener) => listener !== callback
      );
    };
  }
}

export const accentColor = new Store("#007AFF");

listen("on_change_accent_color", (event) => {
  setTimeout(() => {
    getAccentColor().then((value) => accentColor.set(value));
  });
});

getAccentColor().then((value) => accentColor.set(value));

export async function getAccentColor(): Promise<string> {
  const value = await invoke("plugin:accent-color|get_accent_color");

  return String(value);
}
