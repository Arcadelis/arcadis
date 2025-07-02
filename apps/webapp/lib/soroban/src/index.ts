import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}




export const Errors = {
  101: {message:"UserHasVoted"},
  102: {message:"GameNotFound"},
  103: {message:"GameNameCannotBeEmpty"}
}


export interface NewGameAdded {
  creator: string;
  game_id: u32;
}


export interface VoteRegistered {
  game_id: u32;
  voter: string;
}


/**
 * Struct representing a Game.
 */
export interface Game {
  creator: string;
  id: u32;
  name: string;
  votes: u32;
}

export interface Client {
  /**
   * Construct and simulate a add_game transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Adds a new game to the system.
   */
  add_game: ({creator, name}: {creator: string, name: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a vote transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Allows a user to vote for a specific game.
   */
  vote: ({voter, game_id}: {voter: string, game_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a has_voted transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Checks if a user has already voted for a game.
   */
  has_voted: ({user, game_id}: {user: string, game_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a get_game_votes transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Returns the number of votes for a game.
   */
  get_game_votes: ({game_id}: {game_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u32>>

  /**
   * Construct and simulate a get_total_games transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Returns the total number of games added.
   */
  get_total_games: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u32>>

  /**
   * Construct and simulate a get_game_info transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Returns full game info (id, name, votes, creator) for a given game ID.
   */
  get_game_info: ({game_id}: {game_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<readonly [u32, string, u32, string]>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy(null, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAABAAAAAAAAAAAAAAABUVycm9yAAAAAAAAAwAAAAAAAAAMVXNlckhhc1ZvdGVkAAAAZQAAAAAAAAAMR2FtZU5vdEZvdW5kAAAAZgAAAAAAAAAVR2FtZU5hbWVDYW5ub3RCZUVtcHR5AAAAAAAAZw==",
        "AAAAAQAAAAAAAAAAAAAADE5ld0dhbWVBZGRlZAAAAAIAAAAAAAAAB2NyZWF0b3IAAAAAEwAAAAAAAAAHZ2FtZV9pZAAAAAAE",
        "AAAAAQAAAAAAAAAAAAAADlZvdGVSZWdpc3RlcmVkAAAAAAACAAAAAAAAAAdnYW1lX2lkAAAAAAQAAAAAAAAABXZvdGVyAAAAAAAAEw==",
        "AAAAAQAAABtTdHJ1Y3QgcmVwcmVzZW50aW5nIGEgR2FtZS4AAAAAAAAAAARHYW1lAAAABAAAAAAAAAAHY3JlYXRvcgAAAAATAAAAAAAAAAJpZAAAAAAABAAAAAAAAAAEbmFtZQAAABAAAAAAAAAABXZvdGVzAAAAAAAABA==",
        "AAAAAAAAAB5BZGRzIGEgbmV3IGdhbWUgdG8gdGhlIHN5c3RlbS4AAAAAAAhhZGRfZ2FtZQAAAAIAAAAAAAAAB2NyZWF0b3IAAAAAEwAAAAAAAAAEbmFtZQAAABAAAAAA",
        "AAAAAAAAACpBbGxvd3MgYSB1c2VyIHRvIHZvdGUgZm9yIGEgc3BlY2lmaWMgZ2FtZS4AAAAAAAR2b3RlAAAAAgAAAAAAAAAFdm90ZXIAAAAAAAATAAAAAAAAAAdnYW1lX2lkAAAAAAQAAAAA",
        "AAAAAAAAAC5DaGVja3MgaWYgYSB1c2VyIGhhcyBhbHJlYWR5IHZvdGVkIGZvciBhIGdhbWUuAAAAAAAJaGFzX3ZvdGVkAAAAAAAAAgAAAAAAAAAEdXNlcgAAABMAAAAAAAAAB2dhbWVfaWQAAAAABAAAAAEAAAAB",
        "AAAAAAAAACdSZXR1cm5zIHRoZSBudW1iZXIgb2Ygdm90ZXMgZm9yIGEgZ2FtZS4AAAAADmdldF9nYW1lX3ZvdGVzAAAAAAABAAAAAAAAAAdnYW1lX2lkAAAAAAQAAAABAAAABA==",
        "AAAAAAAAAChSZXR1cm5zIHRoZSB0b3RhbCBudW1iZXIgb2YgZ2FtZXMgYWRkZWQuAAAAD2dldF90b3RhbF9nYW1lcwAAAAAAAAAAAQAAAAQ=",
        "AAAAAAAAAEZSZXR1cm5zIGZ1bGwgZ2FtZSBpbmZvIChpZCwgbmFtZSwgdm90ZXMsIGNyZWF0b3IpIGZvciBhIGdpdmVuIGdhbWUgSUQuAAAAAAANZ2V0X2dhbWVfaW5mbwAAAAAAAAEAAAAAAAAAB2dhbWVfaWQAAAAABAAAAAEAAAPtAAAABAAAAAQAAAAQAAAABAAAABM=" ]),
      options
    )
  }
  public readonly fromJSON = {
    add_game: this.txFromJSON<null>,
        vote: this.txFromJSON<null>,
        has_voted: this.txFromJSON<boolean>,
        get_game_votes: this.txFromJSON<u32>,
        get_total_games: this.txFromJSON<u32>,
        get_game_info: this.txFromJSON<readonly [u32, string, u32, string]>
  }
}