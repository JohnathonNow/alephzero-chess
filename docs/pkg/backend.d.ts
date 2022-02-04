/* tslint:disable */
/* eslint-disable */
/**
*/
export class WasmBoard {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} s
*/
  build(s: string): void;
/**
* @returns {string}
*/
  deconstruct(): string;
/**
* @param {string} piece
* @param {boolean} white
* @param {string} rank
* @param {string} file
* @returns {number | undefined}
*/
  place_piece(piece: string, white: boolean, rank: string, file: string): number | undefined;
/**
* @param {string} rank
* @param {string} file
* @returns {number | undefined}
*/
  get_piece_at(rank: string, file: string): number | undefined;
/**
* @param {string} file
* @param {string} zoom
* @returns {number | undefined}
*/
  add_pawns(file: string, zoom: string): number | undefined;
/**
* @param {number} id
* @returns {string | undefined}
*/
  get_piece_info(id: number): string | undefined;
/**
* @param {string} rank
* @param {string} file
* @param {string} new_type
* @returns {number | undefined}
*/
  promote(rank: string, file: string, new_type: string): number | undefined;
/**
* @param {string} rank
* @param {string} file
* @param {string} to_rank
* @param {string} to_file
* @returns {number | undefined}
*/
  do_move(rank: string, file: string, to_rank: string, to_file: string): number | undefined;
/**
* @param {string} rank
* @param {string} file
* @param {string} to_rank
* @param {string} to_file
* @returns {boolean | undefined}
*/
  is_move_legal(rank: string, file: string, to_rank: string, to_file: string): boolean | undefined;
/**
* @returns {string | undefined}
*/
  get_pieces(): string | undefined;
/**
* @param {string} srank
* @param {string} sfile
* @param {string} swinx
* @param {string} swiny
* @param {string} szoom
* @returns {string | undefined}
*/
  get_legal_moves(srank: string, sfile: string, swinx: string, swiny: string, szoom: string): string | undefined;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmboard_free: (a: number) => void;
  readonly wasmboard_new: () => number;
  readonly wasmboard_build: (a: number, b: number, c: number) => void;
  readonly wasmboard_deconstruct: (a: number, b: number) => void;
  readonly wasmboard_place_piece: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly wasmboard_get_piece_at: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wasmboard_add_pawns: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wasmboard_get_piece_info: (a: number, b: number, c: number) => void;
  readonly wasmboard_promote: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly wasmboard_do_move: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => void;
  readonly wasmboard_is_move_legal: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => number;
  readonly wasmboard_get_pieces: (a: number, b: number) => void;
  readonly wasmboard_get_legal_moves: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
