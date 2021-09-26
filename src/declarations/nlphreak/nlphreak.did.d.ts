import type { Principal } from '@dfinity/principal';
export interface Game {
  'id' : number,
  'found' : Array<boolean>,
  'answers' : Array<string>,
  'score' : number,
  'keywords' : Array<string>,
  'state' : GameState,
  'questions' : Array<string>,
  'principal_str' : string,
  'points' : Array<number>,
}
export type GameState = { 'New' : null } |
  { 'Answered' : null } |
  { 'Asked' : null };
export interface Profile {
  'name' : string,
  'max_score' : number,
  'games' : Array<number>,
}
export interface Result { 'ok' : boolean, 'payload' : string }
export interface _SERVICE {
  'addAnswers' : (arg_0: number, arg_1: Array<string>) => Promise<Result>,
  'addQuestions' : (arg_0: number, arg_1: Array<string>) => Promise<Result>,
  'clearAll' : () => Promise<undefined>,
  'getAsked' : () => Promise<Array<number>>,
  'getGame' : (arg_0: number) => Promise<Game>,
  'getSelf' : () => Promise<Profile>,
  'listAllGames' : () => Promise<Array<Game>>,
  'startGame' : () => Promise<Result>,
  'update' : (arg_0: string) => Promise<Result>,
}
