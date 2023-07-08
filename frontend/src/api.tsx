import Cookies from "universal-cookie";

var API_ROOT = process.env["API_ROOT_URL"];

export interface LoginResponse {
  token: string
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

export interface ApiError {
  error: string;
  msg?: string;
}

function get_session_token(): string | null {
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

export async function create_game(): Promise<string | null> {
  let res = await fetch(`https://localhost/api/game`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${get_session_token()}`,
    },
  });
  return res.status == 200 ? res.json() : null;
}

export async function get_game(uid: string): Promise<GameState | null> {
  let res = await fetch(`https://localhost/api/game/${uid}`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${get_session_token()}`,
    },
  });
  return res.status == 200 ? res.json() : null;
}
