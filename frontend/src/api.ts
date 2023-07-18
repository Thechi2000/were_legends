import Cookies from "universal-cookie";

var API_ROOT = process.env["API_ROOT_URL"];

export interface LoginResponse {
  token: string;
}

/* export type State =
  | { state: "not_started" }
  | { state: "waiting_game_start" }
  | { state: "in_game" }
  | { state: "finished" }
  | { state: "waiting_votes"; players: string[] };

export type GameState = {
  uid: string;
  player_names: string[];
  has_started: boolean;
  player_state?: PlayerState;
  votes: { [key: string]: { [key: string]: string } };
  state: State;
  roles: { [key: string]: string };
}; */

export type GameState = { uid: string; player_names: string[] } & (
  | {
      state: "setup";
    }
  | { state: "draft"; player_state: PlayerState }
  | { state: "in_game"; player_state: PlayerState }
  | { state: "voting"; votes_received: string[]; player_state: PlayerState }
  | {
      state: "end";
      votes: { [key: string]: { [key: string]: string } };
      roles: { [key: string]: string };
    }
);

export interface PlayerState {
  class: string;
  juliette?: string;
  inting?: boolean;
  mission?: string;
}


export interface ApiError {
  error: string;
  msg?: string;
}

export type Response<T> =
  | { ok: true; value: T }
  | { ok: false; error?: ApiError };

async function convertResponse<T>(
  res: globalThis.Response
): Promise<Response<T>> {
  const contentType = res.headers.get("content-type");
  if (contentType && contentType.indexOf("application/json") !== -1) {
    return res.status == 200
      ? { ok: true, value: (await res.json()) as T }
      : { ok: false, error: await res.json() };
  } else {
    return {
      ok: false,
    };
  }
}

function getSessionToken(): string | null {
  return new Cookies().get("session");
}

export async function login(name: string): Promise<LoginResponse | ApiError> {
  let res = await fetch(`https://localhost/api/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ name: name }),
  });

  return res.json();
}

export async function createGame(): Promise<string | null> {
  let res = await fetch(`https://localhost/api/game`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${getSessionToken()}`,
    },
  });
  return res.status == 200 ? res.json() : null;
}

export async function getGame(uid: string): Promise<GameState | null> {
  let res = await fetch(`https://localhost/api/game/${uid}`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${getSessionToken()}`,
    },
  });
  return res.status == 200 ? res.json() : null;
}

export async function getCurrentGame(): Promise<Response<GameState>> {
  let res = await fetch(`https://localhost/api/game`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${getSessionToken()}`,
    },
  });
  return await convertResponse(res);
}

export async function startGame() {
  let res = await fetch(`https://localhost/api/game/start`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${getSessionToken()}`,
    },
  });
}

export async function endGame() {
  let res = await fetch(`https://localhost/api/game/end`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${getSessionToken()}`,
    },
  });
}

export async function quitGame() {
  await fetch(`https://localhost/api/game/quit`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${getSessionToken()}`,
    },
  });
}

export async function sendVotes(votes: { [key: string]: string }) {
  await fetch(`https://localhost/api/game/votes`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${getSessionToken()}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(votes),
  });
}
