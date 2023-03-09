import type {
  Paginated,
  Friend,
  NewFriend,
  NewFriendsIdea,
  UpdateFriend,
  UpdateFriendsIdea,
  FriendsIdea,
} from '$lib/types/generated';
import apiCall from '$lib/utils/apiCall';

export const getFriends = async (params: any) => {
  return apiCall<Friend[]>('/friends');
};

export const getFriendById = async (id: string) => {
  return apiCall<Friend>(`/friends/${id}`);
};

export const createFriend = async (friend: NewFriend) => {
  return apiCall<Friend>('/friends', {
    method: 'POST',
    body: JSON.stringify(friend),
  });
};

export const updateFriend = async (id: string, friend: UpdateFriend) => {
  return apiCall<Friend>(`/friends/${id}`, {
    method: 'POST',
    body: JSON.stringify(friend),
  });
};

export const getFriendIdeas = async (friendId: string) => {
  return apiCall<FriendsIdea[]>(`/friends/${friendId}/ideas`);
};

export const getFriendIdeaById = async (id: string) => {
  return apiCall<FriendsIdea>(`/friend-ideas/${id}`);
};

export const createFriendIdea = async (newFriendIdea: NewFriendsIdea) => {
  return apiCall<FriendsIdea>('/friend-ideas', {
    method: 'POST',
    body: JSON.stringify(newFriendIdea),
  });
};

export const updateFriendIdea = async (id: string, updatedFriendIdea: UpdateFriendsIdea) => {
  return apiCall<FriendsIdea>(`/friend-ideas/${id}`, {
    method: 'POST',
    body: JSON.stringify(updatedFriendIdea),
  });
};
