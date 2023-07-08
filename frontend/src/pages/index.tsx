import { create_game, login } from "@/api";
import { Button } from "@/components/inputs";
import getSessionJWT, { Session } from "@/session";
import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";

export default function Home({ session, setSessionToken }: any) {
  const router = useRouter();

  return (
    <div className="flex flex-col justify-items-center items-center gap-4 pt-10">
      <h1 className="font-rhuma-sinera text-9xl">Among Legends</h1>
      <h2 className="text-3xl">Who is the real inter ?</h2>
      <p>Hi {session ? session.name : ""}</p>
      <Button
        onClick={() => {
          setSessionToken(null);
        }}
      >
        Log out
      </Button>
      <Button
        onClick={() => {
          create_game().then((uid) => {
            if (uid) {
              router.push(`/game/${uid}`);
            }
          });
        }}
      >
        Create game
      </Button>
      <Link href="/rules">Rules</Link>
    </div>
  );
}

Home.requireLogin = true;
