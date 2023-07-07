import { login } from "@/api";
import { Button } from "@/components/inputs";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";

export default function Login({ session, setSessionToken }: any) {
  var [name, setName] = useState("");
  const router = useRouter();

  useEffect(() => {
    if (session) {
      router.back();
    }
  }, [session]);

  return (
    <div className="flex flex-col items-center justify-center h-screen gap-8 pb-20">
      <p className="text-9xl font-rhuma-sinera">Login</p>
      <input
        placeholder="Name"
        className="border-solid border border-slate-500 rounded p-2 bg-slate-200 text-2xl"
        onChange={(e) => setName(e.target.value)}
      />
      <Button
        className="py-3"
        onClick={async () => {
          let token = await login(name);
          console.log(token);
          setSessionToken(token);
        }}
      >
        <p className="text-4xl">Login</p>
      </Button>
    </div>
  );
}
