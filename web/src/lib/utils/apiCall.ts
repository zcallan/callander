import { PUBLIC_API_URL } from '$env/static/public';

async function apiCall<T>(url: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${PUBLIC_API_URL}${url}`, {
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
    ...options,
  });

  const data = await res.json();

  return data;
}

export default apiCall;
