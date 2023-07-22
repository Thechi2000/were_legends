export const ROOT_URL = process.env.NEXT_PUBLIC_BASE_URL;

export async function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
