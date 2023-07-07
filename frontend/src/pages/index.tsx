import { create_game, login } from "@/api";
import { decodeSession } from "@/session";
import jwtDecode from "jwt-decode";
import { cookies } from "next/dist/client/components/headers";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";

export default function Home() {
  var [name, setName] = useState("");
  var [session, setSession] = useState(null as null | string);
  var [cookies, setCookies, removeCookies] = useCookies(["session"]);
  const router = useRouter();

  function refreshSession() {
    setSession(cookies.session || null);
  }

  useEffect(() => {
    refreshSession();
  });

  return (
    <>
      {session ? (
        <>
          <p>Hi {decodeSession(cookies.session).name}</p>
          <button
            onClick={() => {
              removeCookies("session");
              refreshSession();
            }}
          >
            Log out
          </button>
          <button
            onClick={() => {
              create_game().then((uid) => {
                if (uid) {
                  router.push(`/game/${uid}`);
                }
              });
            }}
          >
            Create game
          </button>
        </>
      ) : (
        <>
          <p>Login:</p>
          <input onChange={(e) => setName(e.target.value)} />
          <button
            onClick={async () => {
              let token = await login(name);
              setCookies("session", token);
              refreshSession();
            }}
          >
            Login
          </button>
        </>
      )}
    </>
  );
}
