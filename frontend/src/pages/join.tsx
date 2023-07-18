import { joinGame } from "@/api";
import { GetServerSidePropsContext, GetServerSidePropsResult } from "next";
import { useRouter } from "next/router";

export default function Join() {
  return <></>;
}

export async function getServerSideProps(
  context: GetServerSidePropsContext
): Promise<GetServerSidePropsResult<{}>> {
  console.log(JSON.stringify(context.req.cookies));
  console.log(JSON.stringify(context.query));

  if ("session" in context.req.cookies) {
    let bearer = context.req.cookies["session"];
    if ("uid" in context.query) {
      await joinGame(context.query["uid"] as string, bearer);
      return {
        redirect: {
          permanent: false,
          destination: "/game",
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
  } else {
    return {
      redirect: {
        permanent: false,
        destination: "/login",
      },
    };
  }
}
