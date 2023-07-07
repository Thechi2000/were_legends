import jwtDecode from "jwt-decode";

export interface Session {
  name: string;
  summoner_name: string | null;
}

export function decodeSession(token: string): Session {
  return token
    ? (jwtDecode(token) as Session)
    : {
        name: "",
        summoner_name: null,
      };
}
