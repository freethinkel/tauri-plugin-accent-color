declare type Callback<T> = (value: T) => void;
declare class Store<T> {
    private value;
    private listeners;
    constructor(initialValue?: T);
    private notifyListeners;
    set(value: T): void;
    get(): T;
    update(callback: (value: T) => T): void;
    subscribe(callback: Callback<T>): () => void;
}
export declare const accentColor: Store<string>;
export declare function getAccentColor(): Promise<string>;
export {};
