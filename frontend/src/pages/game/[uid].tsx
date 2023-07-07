import { GameState, get_game } from "@/api";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";

export default function Game() {
  const router = useRouter();
  const [game, setGame] = useState(null as GameState | null);

  useEffect(() => {
    if (router.query.uid !== undefined && !game) {
      console.log(router.query.uid);
      get_game(router.query.uid as string).then((g) => setGame(g));
    }
  });

  return (
    <>
      <p>Game {router.query.uid}</p>
      <p>{JSON.stringify(game)}</p>
    </>
  );
}
