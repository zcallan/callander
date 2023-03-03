import type { NewFriend, UpdateFriend } from '$lib/types/generated';

export const getFriends = async (params: any) => {
  console.log('getFriends', params);

  const res = await fetch('http://localhost:8080/friends');
  const data = await res.json();

  return data;
};

export const getFriendById = async (id: string) => {
  const res = await fetch(`http://localhost:8080/friends/${id}`);
  const data = await res.json();

  return data;
};

export const createFriend = async (friend: NewFriend) => {
  const res = await fetch('http://localhost:8080/friends', {
    method: 'POST',
    body: JSON.stringify(friend),
    headers: {
      'Content-Type': 'application/json',
    },
  });
  const data = await res.json();

  return data;
};

export const updateFriend = async (id: string, friend: UpdateFriend) => {
  const res = await fetch(`http://localhost:8080/friends/${id}`, {
    method: 'POST',
    body: JSON.stringify(friend),
    headers: {
      'Content-Type': 'application/json',
    },
  });
  const data = await res.json();

  return data;
};
