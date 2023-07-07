import localFont from "next/font/local";

import "../styles/globals.css";
import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import getSessionJWT, { Session, decodeSession } from "@/session";
import { useRouter } from "next/router";

const TITLE_FONT = localFont({
  src: "../../fonts/Rhuma Sinera Regular.ttf",
});

export default function MyApp({ Component, pageProps }: any) {
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
    setSession(getSessionJWT());
  }, []);

  useEffect(() => {
    if (!session) {
      router.push('/login')
    }
  }, [session]);

  return (
    <main>
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
