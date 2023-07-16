import localFont from "next/font/local";

import "../styles/globals.css";
import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import getSessionJWT, { Session, decodeSession } from "@/session";
import { useRouter } from "next/router";

const TITLE_FONT = localFont({
  src: "../../fonts/Rhuma Sinera Regular.ttf",
});

export default function App({ Component, pageProps }: any) {
  var [cookies, setCookies, removeCookies] = useCookies(["session"]);
  const [session, setSession] = useState(null as Session | null);
  const router = useRouter();

  function setSessionToken(token: string) {
    if (token) {
      setCookies("session", token);
      setSession(decodeSession(token));
    } else {
      removeCookies("session");
      setSession(null);
    }
  }

  useEffect(() => {
    var tmp = getSessionJWT();
    setSession(tmp);
  }, []);

  useEffect(() => {
    if (!session && Component.requireLogin) {
      router.push("/login");
    }
  }, [session]);

  return (
    <main className="bg-sky-600 min-h-screen text-slate-200 select-none">
      <style jsx global>{`
        html {
          --font-rhuma-sinera: ${TITLE_FONT.style.fontFamily};
        }
      `}</style>
      <Component
        session={session}
        setSessionToken={setSessionToken}
        {...pageProps}
      />
    </main>
  );
}
