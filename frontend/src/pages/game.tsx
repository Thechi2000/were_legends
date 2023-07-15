import {
  GameState,
  applyUpdate,
  getUpdates,
  getCurrentGame,
  quitGame,
  startGame,
} from "@/api";
import { promises as fs } from "fs";
import { useRouter } from "next/router";
import { useEffect, useRef, useState } from "react";
import { ROOT_URL, sleep } from "@/utils";
import { Button } from "@/components/inputs";
import { RolesData } from "@/idata";
import { RoleDisplay } from "@/components/roles";
import path from "path";

function PlayerInfo({ name }: { name?: string }) {
  return (
    <p
      className={
        "text-2xl text-sky-400 p-3 rounded-lg text-center align-middle leading-10 " +
        (name ? "bg-slate-200" : "bg-sky-800")
      }
      style={{
        minWidth: "12rem",
        maxWidth: "12rem",
        minHeight: "4rem",
        maxHeight: "4rem",
      }}
    >
      {name}
    </p>
  );
}

export default function Game({ data }: { data: RolesData }) {
  const router = useRouter();
  const [game, setGame] = useState(null as GameState | null);
  const inviteLinkRef = useRef<HTMLParagraphElement>(null);

  useEffect(() => {
    refreshGame();
  }, []);

  useEffect(() => {
    var running = true;

    async function fetchUpdates() {
      while (running) {
        await sleep(3000);

        if (game) {
          var updates = await getUpdates();

          if (updates && updates.length > 0) {
            var new_game = game;
            for (var i = 0; i < updates.length; ++i) {
              new_game = applyUpdate(new_game, updates[i]);
            }
            setGame(new_game);
          }
        }
      }
    }

    fetchUpdates();

    return () => {
      running = false;
    };
  });

  async function refreshGame() {
    var res = await getCurrentGame();
    if (res.ok) {
      setGame(res.value);
    } else {
      router.push("/");
    }
  }

  function PlayerInfos() {
    function generatePlayerInfos() {
      if (game) {
        game.player_names.sort();
      }

      var arr = [];
      for (var i = 0; i < 5; ++i) {
        arr.push(
          <PlayerInfo
            key={`player${i}`}
            name={
              game && game?.player_names.length > i
                ? game.player_names[i]
                : undefined
            }
          />
        );
      }
      return arr;
    }

    return (
      <div className="flex flex-col items-center justify-center gap-8">
        {generatePlayerInfos()}
      </div>
    );
  }

  function QuitButton() {
    return (
      <Button
        onClick={() => {
          quitGame().then(refreshGame);
        }}
        className="text-2xl py-2 max-w-fit"
      >
        Quit
      </Button>
    );
  }

  function Layout() {
    if (game) {
      if (game.player_state) {
        return (
          <div className="flex flex-row items-center gap-40">
            <PlayerInfos />
            <RoleDisplay playerState={game.player_state} data={data} />
          </div>
        );
      } else if (game.player_names.length != 5) {
        return (
          <div className="flex flex-col gap-5 justify-center items-center">
            <PlayerInfos />
            <p className="text-3xl">Invite your friends</p>
            <p ref={inviteLinkRef} className="text-xl">
              {ROOT_URL}/game/join?uid={game?.uid}
            </p>
            <button
              onClick={() => {
                if (
                  inviteLinkRef.current &&
                  inviteLinkRef.current.textContent
                ) {
                  navigator.clipboard.writeText(
                    inviteLinkRef.current.textContent
                  );
                }
              }}
            >
              Copy
            </button>
            <QuitButton />
          </div>
        );
      } else {
        return (
          <div className="flex flex-col gap-5 justify-center items-center">
            <PlayerInfos />
            <div className="flex flex-col gap-4 items-center">
              <Button onClick={startGame} className="text-4xl py-3">
                Start
              </Button>
              <QuitButton />
            </div>
          </div>
        );
      }
    } else {
      return <></>;
    }
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen gap-20 pb-20">
      <Layout />
    </div>
  );
}

export async function getStaticProps() {
  var buf = await fs.readFile(
    path.join(process.cwd(), "data/roles.json"),
    "utf8"
  );
  return { props: { data: JSON.parse(buf) } };
}

Game.requireLogin = true;
