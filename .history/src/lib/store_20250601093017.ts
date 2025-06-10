import { writable } from "svelte/store";

export const current_lang = writable<string>("text");
export const line_cloumn = writable<string>("Ln 0, Col 0");
export const warn_error = writable<string>("Wn 0, Er 0");
