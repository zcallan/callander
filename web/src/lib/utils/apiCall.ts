import { PUBLIC_API_URL } from '$env/static/public';
import cookie from 'cookie';

const deleteCookie = (name: string) => {
  document.cookie = name + '=; Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT;';
};

async function apiCall<T>(url: string, options?: RequestInit): Promise<T> {
  const cookies = cookie.parse(document.cookie);

  const headers: any = {
    'Content-Type': 'application/json',
    ...options?.headers,
  };

  if (cookies?.hanko) {
    headers.Authorization = `Bearer ${cookies.hanko}`;
  }

  const res = await fetch(`${PUBLIC_API_URL}${url}`, {
    headers,
    ...options,
  });

  const data = await res.json();

  if (res.status >= 400) {
    if (res.status === 401) {
      // deleteCookie('hanko');
      // window.location.href = '/login';
    }

    throw new Error(data?.message || 'Something went wrong');
  }

  return data;
}

export default apiCall;
