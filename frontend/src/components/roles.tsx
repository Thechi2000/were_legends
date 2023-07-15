import { PlayerState } from "@/api";
import { RolesData } from "@/idata";

export function RoleDisplay({
  playerState,
  data,
}: {
  playerState: PlayerState;
  data: RolesData;
}) {
  var comps = undefined;

  switch (playerState.class) {
    case "droid":
      if (playerState.mission !== undefined) {
        comps = (
          <p className="text-2xl">Current mission: {playerState.mission}</p>
        );
      }
      break;

    case "romeo":
      if (playerState.juliette !== undefined) {
        comps = <p className="text-2xl">In love with {playerState.juliette}</p>;
      }
      break;

    case "two_face":
      if (playerState.inting !== undefined) {
        comps = (
          <p className="text-2xl">
            You must {playerState.inting ? "loose" : "win"}
          </p>
        );
      }
  }

  return (
    <DefaultRoleDisplay playerState={playerState} data={data} comps={comps} />
  );
}

function DefaultRoleDisplay({
  playerState,
  data,
  comps,
}: {
  playerState: PlayerState;
  data: RolesData;
  comps?: JSX.Element[] | JSX.Element;
}) {
  return (
    <div className="flex flex-col items-center gap-10 max-w-2xl text-center">
      <h1 className="text-5xl">{data[playerState.class].name}</h1>
      <p className="text-2xl">{data[playerState.class].description}</p>
      {comps}
    </div>
  );
}
