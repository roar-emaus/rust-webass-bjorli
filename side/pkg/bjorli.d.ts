/* tslint:disable */
/* eslint-disable */
/**
 * @param {string} date
 * @returns {any}
 */
export function generate_html_table_for_date(date: string): any;
/**
 * @param {number} latitude
 * @param {number} longitude
 * @returns {Promise<any>}
 */
export function get_weather_data(latitude: number, longitude: number): Promise<any>;
/**
 * @param {number} latitude
 * @param {number} longitude
 * @returns {Promise<any>}
 */
export function get_daily_weather(latitude: number, longitude: number): Promise<any>;
/**
 *The `RequestCredentials` enum.
 *
 **This API requires the following crate features to be activated: `RequestCredentials`*
 */
export type RequestCredentials = "omit" | "same-origin" | "include";
/**
 *The `RequestMode` enum.
 *
 **This API requires the following crate features to be activated: `RequestMode`*
 */
export type RequestMode = "same-origin" | "no-cors" | "cors" | "navigate";

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly generate_html_table_for_date: (a: number, b: number, c: number) => void;
  readonly get_weather_data: (a: number, b: number) => number;
  readonly get_daily_weather: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he77f6ca9152cf107: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h92d56b77a30a5712: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
