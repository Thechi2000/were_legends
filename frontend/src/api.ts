import Cookies from "universal-cookie";

var API_ROOT = "http://localhost/api";

export interface LoginResponse {
  token: string;
}

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
  let res = await fetch(`${API_ROOT}/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ name: name }),
  });

  return res.json();
}

export async function createGame(bearer?: string): Promise<string | null> {
  let res = await fetch(`${API_ROOT}/game`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${bearer || getSessionToken()}`,
    },
  });
  return res.status == 200 ? res.json() : null;
}

export async function getGame(
  uid: string,
  bearer?: string
): Promise<GameState | null> {
  let res = await fetch(`${API_ROOT}/game/${uid}`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${bearer || getSessionToken()}`,
    },
  });
  return res.status == 200 ? res.json() : null;
}

export async function getCurrentGame(
  bearer?: string
): Promise<Response<GameState>> {
  let res = await fetch(`${API_ROOT}/game`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${bearer || getSessionToken()}`,
    },
  });
  return await convertResponse(res);
}

export async function joinGame(uid: string, bearer?: string) {
  let res = await fetch(`${API_ROOT}/game/${uid}/join`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${bearer || getSessionToken()}`,
    },
  });
  return await convertResponse(res);
}

export async function startGame(bearer?: string) {
  let res = await fetch(`${API_ROOT}/game/start`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${bearer || getSessionToken()}`,
    },
  });
}

export async function endGame(bearer?: string) {
  let res = await fetch(`${API_ROOT}/game/end`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${bearer || getSessionToken()}`,
    },
  });
}

export async function quitGame(bearer?: string) {
  await fetch(`${API_ROOT}/game/quit`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${bearer || getSessionToken()}`,
    },
  });
}

export async function sendVotes(
  votes: { [key: string]: string },
  bearer?: string
) {
  await fetch(`${API_ROOT}/game/votes`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${bearer || getSessionToken()}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(votes),
  });
}
