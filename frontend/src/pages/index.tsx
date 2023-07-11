import { create_game, get_current_game, login } from "@/api";
import { Button, Href } from "@/components/inputs";
import { useRouter } from "next/router";
import { useEffect } from "react";

export default function Home({ session, setSessionToken }: any) {
  const router = useRouter();

  useEffect(() => {
    get_current_game().then((res) => {
      if (res.ok) {
        router.push("/game");
      }
    });
  }, []);

  return (
    <div className="h-full flex flex-col justify-center items-center h-screen gap-20 pb-20">
      <div className="flex flex-col justify-center items-center">
        <h1 className="font-rhuma-sinera text-9xl">Among Legends</h1>
        <h2 className="text-5xl">Who is the real inter ?</h2>
      </div>
      <div className="flex flex-col justify-center items-center gap-5">
        <Button
          className="w-72 py-3"
          onClick={() => {
            create_game().then((uid) => {
              if (uid) {
                router.push(`/game`);
              }
            });
          }}
        >
          <p className="text-4xl">Create game</p>
        </Button>
        <Button
          className="w-72 py-3"
          onClick={() => {
            setSessionToken(null);
          }}
        >
          <p className="text-4xl">Log out</p>
        </Button>
        <p className="text-3xl">Logged in as {session ? session.name : ""}</p>
      </div>
      <div className="flex flex-row gap-4">
        <p className="text-3xl">Check out the</p>
        <Href arrowLeft className="text-3xl" href="/rules ">
          Rules
        </Href>
      </div>
    </div>
  );
}

Home.requireLogin = true;
