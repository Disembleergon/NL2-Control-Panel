// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
  type Action =
    | ""
    | "dispatch"
    | "harnessOpen"
    | "harnessClose"
    | "gatesOpen"
    | "gatesClose"
    | "platformRaise"
    | "platformLower"
    | "carUnlock"
    | "carLock"
    | "emergencyOn"
    | "emergencyOff";

  interface PostData {
    action: Action;
    connectionTest: boolean;
  }

  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

export {};
