import {
  GameState,
  getCurrentGame,
  quitGame,
  startGame,
  sendVotes,
  endGame,
  joinGame,
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
import { sleep } from "@/utils";
import { Button } from "@/components/inputs";
import { Data } from "@/idata";
import { RoleDisplay } from "@/components/roles";
import path from "path";
import getSessionJWT from "@/session";
import { GetServerSidePropsContext, GetServerSidePropsResult } from "next";

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

export default function Game({
  data,
  gameState,
}: {
  data: Data;
  gameState: GameState;
}) {
  const router = useRouter();
  const [game, setGame] = useState(gameState as GameState);
  const inviteLinkRef = useRef<HTMLParagraphElement>(null);
  const [votes, setVotes] = useState({} as { [key: string]: string });

  useEffect(() => {
    var running = true;

    async function fetchUpdates() {
      if (game) {
        while (running) {
          await sleep(3000);

          refreshGame();
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
      game.state == "voting" &&
      session &&
      game.votes_received.indexOf(session.name) !== -1
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
      switch (game.state) {
        case "setup":
          return (
            <>
              <div className="flex flex-col gap-5 justify-center items-center">
                <PlayerInfos />
              </div>
              {game.player_names.length == 5 ? (
                <Button onClick={startGame} className="text-4xl py-3">
                  Start
                </Button>
              ) : (
                <>
                  <div className="flex flex-col gap-5 justify-center items-center">
                    <p className="text-3xl">Invite your friends</p>
                    <p ref={inviteLinkRef} className="text-xl select-text">
                      {router.basePath}/game?join={game.uid}
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
                  </div>

                  <QuitButton />
                </>
              )}
            </>
          );

        case "draft":
          return (
            <div className="flex flex-col gap-20 justify-center items-center">
              <div className="flex flex-row items-center gap-40">
                <PlayerInfos />
                <RoleDisplay playerState={game.player_state} data={data} />
              </div>
              <div className="flex flex-row items-center gap-40 text-3xlnull">
                <p>Currently in draft mode</p>
                <Button
                  className="py-3"
                  onClick={() => startGame().then(refreshGame)}
                >
                  Start game
                </Button>
              </div>
            </div>
          );

        case "in_game":
          return (
            <div className="flex flex-col gap-20 justify-center items-center">
              <div className="flex flex-row items-center gap-40">
                <PlayerInfos />
                <RoleDisplay playerState={game.player_state} data={data} />
              </div>
              <div className="flex flex-row items-center gap-40 text-3xl">
                <p>Currently in game</p>
                <Button
                  className="py-3"
                  onClick={() => endGame().then(refreshGame)}
                >
                  End game
                </Button>
              </div>
            </div>
          );

        case "voting":
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
                  {Object.keys(data.roles).map((r) => (
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
                                var new_votes = JSON.parse(
                                  JSON.stringify(votes)
                                );
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
                onClick={() => sendVotes(votes).then(refreshGame)}
                disabled={Object.keys(votes).length != 4 || hasVoted()}
                className="text-3xl w-fit py-3"
              >
                Submit
              </Button>
              <p>
                Waiting for{" "}
                {game.player_names
                  .filter((n) => game.votes_received.indexOf(n) === -1)
                  .join(", ")}
              </p>
            </div>
          );

        case "end":
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
      }
    }
  }
  return (
    <div className="flex flex-col items-center justify-center h-screen gap-20 pb-20">
      <Layout />
    </div>
  );
}

export async function getServerSideProps(
  context: GetServerSidePropsContext
): Promise<GetServerSidePropsResult<{ data: Data; gameState: GameState }>> {
  if ("session" in context.req.cookies) {
    let bearer = context.req.cookies["session"];

    if ("join" in context.query) {
      await joinGame(context.query["join"] as string, bearer);
      return {
        redirect: {
          permanent: false,
          destination: "/game",
        },
      };
    } else {
      let game = await getCurrentGame(bearer);

      if (game.ok) {
        return {
          props: {
            data: JSON.parse(
              await fs.readFile(path.join(process.cwd(), "data.json"), "utf8")
            ),
            gameState: game.value,
          },
        };
      } else {
        return {
          redirect: {
            permanent: false,
            destination: "/",
          },
        };
      }
    }
  } else {
    return {
      redirect: {
        permanent: false,
        destination: "/login",
      },
    };
  }
}
