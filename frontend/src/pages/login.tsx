import { login } from "@/api";
import { Button } from "@/components/inputs";
import { useRouter } from "next/router";
import { useState } from "react";
import { useCookies } from "react-cookie";

export default function Login({ session, setSessionToken }: any) {
  var [cookies, setCookies, removeCookies] = useCookies(["session"]);
  var [name, setName] = useState("");
  const router = useRouter();

  return (
    <>
      <p>Login:</p>
      <input
        placeholder="Name"
        className="border-solid border-2 border-sky-500 rounded p-2"
        onChange={(e) => setName(e.target.value)}
      />
      <Button
        onClick={async () => {
          let token = await login(name);
          console.log(token);
          setSessionToken(token);
          router.back();
        }}
      >
        Login
      </Button>
    </>
  );
}
