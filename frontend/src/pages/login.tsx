import { ApiError, login } from "@/api";
import { Button } from "@/components/inputs";
import { useRouter } from "next/router";
import { ComponentElement, useEffect, useState } from "react";

export default function Login({ session, setSessionToken }: any) {
  const [name, setName] = useState("");
  const [error, setError] = useState(null as ApiError | null);
  const [errorInfo, setErrorInfo] = useState(null as string | null);
  const router = useRouter();

  useEffect(() => {
    if (session) {
      router.back();
    }
  }, [session]);

  useEffect(() => {
    if (error) {
      switch (error.error) {
        case "INVALID_NAME":
          setErrorInfo("Name must not be empty");
          break;

        default:
          setErrorInfo("Internal error occurred");
          break;
      }
    }
  }, [error]);

  return (
    <div className="flex flex-col items-center justify-center h-screen gap-8 pb-20">
      <p className="text-9xl font-rhuma-sinera">Login</p>
      <div>
        <input
          placeholder="Name"
          className={
            "text-slate-800 border-solid rounded p-2 bg-slate-200 text-2xl " +
            (error && error.error == "INVALID_NAME"
              ? "border-red-500 border-4"
              : "border-slate-500 border")
          }
          onChange={(e) => setName(e.target.value)}
        />
        {errorInfo ? <p className="text-red-500 pl-4 p-1">{errorInfo}</p> : <></>}
      </div>

      <Button
        className="py-3"
        onClick={async () => {
          let res = await login(name);
          console.log(res);

          if ("token" in res) {
            setSessionToken(res.token);
          } else if ("error" in res) {
            setError(res);
          }
        }}
      >
        <p className="text-4xl">Login</p>
      </Button>
    </div>
  );
}
