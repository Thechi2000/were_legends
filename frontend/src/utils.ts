export const ROOT_URL: string = "https://localhost";

export async function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
