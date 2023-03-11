import { PUBLIC_API_URL } from '$env/static/public';

async function apiCall<T>(url: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${PUBLIC_API_URL}${url}`, {
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
    credentials: 'include',
    ...options,
  });

  const data = await res.json();

  if (res.status >= 400) {
    throw new Error(data?.message || 'Something went wrong');
  }

  return data;
}

export default apiCall;
