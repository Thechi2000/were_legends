import {
  GameState,
  applyUpdate,
  getUpdates,
  getCurrentGame,
  quitGame,
  startGame,
  sendVotes,
} from "@/api";
import { promises as fs } from "fs";
import { useRouter } from "next/router";
import {
  DetailedHTMLProps,
  HTMLAttributes,
  useEffect,
  useRef,
  useState,
} from "react";
import { ROOT_URL, sleep } from "@/utils";
import { Button } from "@/components/inputs";
import { Data } from "@/idata";
import { RoleDisplay } from "@/components/roles";
import path from "path";
import getSessionJWT from "@/session";

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

export default function Game({ data }: { data: Data }) {
  const router = useRouter();
  const [game, setGame] = useState(null as GameState | null);
  const inviteLinkRef = useRef<HTMLParagraphElement>(null);
  const [votes, setVotes] = useState({} as { [key: string]: string });

  useEffect(() => {
    refreshGame();
  }, []);

  useEffect(() => {
    var running = true;

    async function fetchUpdates() {
      if (game) {
        var new_game = game;
        while (running) {
          await sleep(3000);

          var updates = await getUpdates();

          if (updates && updates.length > 0) {
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

  function hasVoted() {
    var session = getSessionJWT();
    return (
      game &&
      game.state.state == "waiting_votes" &&
      session &&
      game.state.players.indexOf(session.name) === -1
    );
  }

  function RoleInfo({
    state,
    role,
    ...props
  }: {
    state: "selected" | "wrong" | "correct" | "none";
    role: string | undefined;
  } & DetailedHTMLProps<HTMLAttributes<HTMLDivElement>, HTMLDivElement>) {
    return (
      <div
        className={
          "text-2xl text-sky-400 p-3 rounded-lg text-center align-middle leading-10 " +
          (role === undefined
            ? "bg-sky-900"
            : state == "selected"
            ? "bg-slate-200"
            : state == "wrong"
            ? "bg-red-900 text-slate-300"
            : state == "correct"
            ? "bg-green-700 text-slate-200"
            : "bg-sky-700 text-slate-300")
        }
        style={{
          minHeight: "4rem",
          maxHeight: "4rem",
          minWidth: "10rem",
          maxWidth: "10rem",
        }}
        {...props}
      >
        {role ? data.roles[role].name : ""}
      </div>
    );
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

  function QuitButton({ big }: { big?: boolean }) {
    return (
      <Button
        onClick={() => {
          quitGame().then(refreshGame);
        }}
        className={(big ? "text-4xl py-3 " : "text-2xl py-2 ") + "max-w-fit"}
      >
        Quit
      </Button>
    );
  }

  function Layout() {
    if (game) {
      if (game.state.state == "finished") {
        return (
          <div className="flex flex-col gap-20 items-center">
            <table className="border-separate border-spacing-x-8 border-spacing-y-4">
              <tbody>
                <tr>
                  <td />
                  {game.player_names.map((n) => (
                    <th className="text-3xl" scope="col" key={n}>
                      {n}
                    </th>
                  ))}
                </tr>
                <tr>
                  <td />
                  {game.player_names.map((n) => (
                    <td className="text-3xl" scope="col" key={n}>
                      <RoleInfo state="none" role={game.roles[n]} />
                    </td>
                  ))}
                </tr>
                {game.player_names.map((n) => (
                  <tr key={n}>
                    <th className="text-3xl" scope="row">
                      {n}
                    </th>
                    {game.player_names.map((n2) => (
                      <td key={n2}>
                        <RoleInfo
                          role={game.votes[n][n2]}
                          state={
                            game.roles[n2] == game.votes[n][n2]
                              ? "correct"
                              : "wrong"
                          }
                        />
                      </td>
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
            <QuitButton big />
          </div>
        );
      } else if (game.state.state == "waiting_votes") {
        var session = getSessionJWT();

        return (
          <div className="flex flex-col items-center gap-10">
            <table className="border-separate border-spacing-x-8 border-spacing-y-4">
              <thead>
                <tr>
                  {game.player_names
                    .filter((p) => p != session?.name)
                    .map((p) => (
                      <th className="text-3xl" key={p}>
                        {p}
                      </th>
                    ))}
                </tr>
              </thead>
              <tbody>
                {Object.keys(data).map((r) => (
                  <tr key={r}>
                    {game.player_names
                      .filter((n) => n != session?.name)
                      .map((n) => (
                        <td
                          className={
                            "text-2xl text-sky-400 p-3 rounded-lg text-center align-middle leading-10 bg-sky-800 " +
                            (votes[n] == r ? "bg-slate-200" : "bg-sky-800")
                          }
                          key={`${r}${n}`}
                          onClick={() => {
                            if (!hasVoted()) {
                              var new_votes = JSON.parse(JSON.stringify(votes));
                              new_votes[n] = r;
                              setVotes(new_votes);
                            }
                          }}
                        >
                          {data.roles[r].name}
                        </td>
                      ))}
                  </tr>
                ))}
              </tbody>
            </table>
            <Button
              onClick={() => sendVotes(votes)}
              disabled={Object.keys(votes).length != 4 || hasVoted()}
              className="text-3xl w-fit py-3"
            >
              Submit
            </Button>
            {game.state.state == "waiting_votes" ? (
              <p>Waiting for {game.state.players.join(", ")}</p>
            ) : (
              <></>
            )}
          </div>
        );
      } else if (game.player_state) {
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
            <p ref={inviteLinkRef} className="text-xl select-text">
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
