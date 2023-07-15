import Cookies from "universal-cookie";

var API_ROOT = process.env["API_ROOT_URL"];

export interface LoginResponse {
  token: string;
}

export interface GameState {
  uid: string;
  player_names: string[];
  has_started: boolean;
  player_state?: PlayerState;
}

export interface PlayerState {
  class: string;
  juliette?: string;
  inting?: boolean;
  mission?: string;
}

export type Update =
  | {
      type: "hi";
    }
  | {
      type: "player_join";
      name: string;
    }
  | {
      type: "role";
      role: string;
    }
  | {
      type: "mission";
      mission: string;
    }
  | {
      type: "juliette";
      name: string;
    }
  | {
      type: "two_face_state";
      inting: boolean;
    };

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

export async function getUpdates(): Promise<Update[] | null> {
  let res = await fetch(`https://localhost/api/updates`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${getSessionToken()}`,
    },
  });
  return res.status == 200 ? res.json() : null;
}

export async function startGame() {
    let res = await fetch(`https://localhost/api/game/start`, {
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


export function applyUpdate(state: GameState, update: Update): GameState {
  var cloned: GameState = JSON.parse(JSON.stringify(state));

  switch (update.type) {
    case "hi":
      break;

    case "player_join":
      if (cloned.player_names.indexOf(update.name) == -1) {
        cloned.player_names.push(update.name);
      }
      break;

    case "role":
      cloned.player_state = {
        class: update.role,
      };
      break;

    case "mission":
      if (cloned.player_state) {
        cloned.player_state.mission = update.mission;
      }
      break;

    case "juliette":
      if (cloned.player_state) {
        cloned.player_state.juliette = update.name;
      }
      break;

    case "two_face_state":
      if (cloned.player_state) {
        cloned.player_state.inting = update.inting;
      }
      break;
  }

  return cloned;
}
