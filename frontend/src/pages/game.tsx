import {
  GameState,
  applyUpdate,
  getUpdates,
  get_current_game,
  startGame,
} from "@/api";
import { useRouter } from "next/router";
import { MutableRefObject, useEffect, useRef, useState } from "react";
import { ROOT_URL, sleep } from "@/utils";
import { Button } from "@/components/inputs";

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

export default function Game() {
  const router = useRouter();
  const [game, setGame] = useState(null as GameState | null);
  const inviteLinkRef = useRef<HTMLParagraphElement>(null);

  useEffect(() => {
    get_current_game().then((res) => {
      if (res.ok) {
        setGame(res.value);
      } else {
        router.push("/");
      }
    });
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
    <div className="flex flex-col items-center justify-center h-screen gap-20 pb-20">
      <div className="flex flex-col items-center justify-center gap-8">
        {generatePlayerInfos()}
      </div>
      {game && game.player_names.length != 5 ? (
        <div className="flex flex-col gap-5 justify-center items-center">
          <p className="text-3xl">Invite your friends</p>
          <p ref={inviteLinkRef} className="text-xl">
            {ROOT_URL}/game/join?uid={game?.uid}
          </p>
          <button
            onClick={() => {
              if (inviteLinkRef.current && inviteLinkRef.current.textContent) {
                navigator.clipboard.writeText(
                  inviteLinkRef.current.textContent
                );
              }
            }}
          >
            Copy
          </button>
        </div>
      ) : (
        <div>
          <Button onClick={startGame} className="text-4xl py-3">
            Start
          </Button>
        </div>
      )}
    </div>
  );
}

Game.requireLogin = true;
