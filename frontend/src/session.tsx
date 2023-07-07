import jwtDecode from "jwt-decode";
import { Cookies } from "react-cookie";

export interface Session {
  name: string;
  summoner_name: string | null;
}


function getSessionToken(): string | null {
  return new Cookies().get("session");
}

export function decodeSession(token: string | null): Session | null {
  return token
    ? (jwtDecode(token) as Session)
    : null;
}

export default function getSessionJWT(): Session | null {
  return decodeSession(getSessionToken())
}
