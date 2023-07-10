function RoleDescriptor({
  children,
  name,
  image,
}: {
  children: string;
  name: string;
  image?: string;
}) {
  return (
    <div className="flex items-center justify-center gap-6 w-full">
      <div className="w-1/4">
        <h3 className="text-3xl text-right">{name}</h3>
      </div>
      <div className="w-3/4 text-2xl text-left">{children}</div>
    </div>
  );
}

export default function Rules({ session, setSession }: any) {
  return (
    <div className="h-full flex flex-col justify-center items-center pt-10 pb-20 gap-6">
      <h1 className="text-7xl">Rules</h1>

      <i className="h-4" />

      <p className="text-2xl w-2/3 text-center">
        You know League of Legends ? The game were you always loose because your
        team wants too ? Well now, they will continue, but while trying to not
        get caught. Be reassured, they won't stop, otherwise the game would be
        much less fun.
      </p>
      <p className="text-2xl w-2/3 text-center">
        At the start of the game, each player will be assigned a role. He must
        achieve its objectives, while keeping the role secret from the other
        players and trying to figure out the other players' roles.
      </p>

      <i className="h-2" />

      <h2 className="text-5xl">How to play</h2>
      <ul className="text-2xl leading-10 text-center">
        <li>1. Start by choosing a name and log in</li>
        <li>2. From the home page, create a game</li>
        <li>3. Invite your friends using the provided link</li>
        <li>
          4. Start a game on League of Legends, ideally a draft with everyone on
          fill
        </li>
        <li>5. Enjoy !</li>
      </ul>

      <h2 className="text-5xl">Roles</h2>
      <div className="flex flex-col justify-center items-center gap-5 w-1/2">
        <RoleDescriptor name="Super Hero">
          You have to win the game, by any means. You need not to hide your role
        </RoleDescriptor>
        <RoleDescriptor name="Impostor">
          You need to loose the game
        </RoleDescriptor>
        <RoleDescriptor name="Crook">
          You need to win the game, while tricking your allies into thinking
          that you are impostor
        </RoleDescriptor>
        <RoleDescriptor name="Kamikaze">
          You must win, while having the most damage and deaths in you team
        </RoleDescriptor>
        <RoleDescriptor name="Droid">
          You will periodically receive mission that you must accomplish
        </RoleDescriptor>
        <RoleDescriptor name="TwoFace">
          You a bipolar player, which will need to win or loose depending on the
          instruction you will receive
        </RoleDescriptor>
        <RoleDescriptor name="Romeo">
          You fell in love with a player, and are not allowed to kill him. If he
          dies, you have a minute to get killed
        </RoleDescriptor>
      </div>
    </div>
  );
}
