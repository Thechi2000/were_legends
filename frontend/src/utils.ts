export const ROOT_URL: string = "http://localhost";

export async function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
