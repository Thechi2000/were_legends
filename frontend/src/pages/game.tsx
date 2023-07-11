import { GameState, applyUpdate, getUpdates, get_current_game } from "@/api";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { sleep } from "@/utils";

export default function Game() {
  const router = useRouter();
  const [game, setGame] = useState(null as GameState | null);

  useEffect(() => {
    get_current_game().then((g) => setGame(g));
  }, []);

  useEffect(() => {
    var running = true;

    async function fetchUpdates() {
      console.log("background task");
      while (running) {
        await sleep(3000);

        console.log("fetching updates");
        if (game) {
          var updates = await getUpdates();

          if (updates) {
            for (var i = 0; i < updates.length; ++i) {
              setGame(applyUpdate(game, updates[i]));
            }
          }
        }
      }
    }

    fetchUpdates();

    return () => {
      running = false;
    };
  });

  return (
    <>
      <p>Game {game?.uid}:</p>
      <p>{JSON.stringify(game)}</p>
    </>
  );
}

Game.requireLogin = true;
