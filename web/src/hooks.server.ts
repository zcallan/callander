import { redirect, type Handle } from '@sveltejs/kit';
import cookie from 'cookie';

export const handle = (async ({ event, resolve }) => {
  const cookies = cookie.parse(event.request.headers.get('cookie') || '');
  const isLoggedIn = !!cookies.hanko;
  const isLoginPage = event.url.pathname.startsWith('/login');

  if (!isLoggedIn && !isLoginPage) {
    throw redirect(307, '/login');
  }

  if (isLoggedIn && isLoginPage) {
    throw redirect(307, '/');
  }

  const response = await resolve(event);

  return response;
}) satisfies Handle;
