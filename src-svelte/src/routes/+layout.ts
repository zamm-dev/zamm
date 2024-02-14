export const prerender = true;
export const ssr = false;

export function load({ url }) {
  return {
    url: url.pathname || "/",
  };
}
