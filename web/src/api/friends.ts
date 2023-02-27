import type { NewFriend } from '../types/generated';

export const getFriends = async () => {
	const res = await fetch('http://localhost:8080/friends');
	const data = await res.json();

	return data;
};

export const createFriend = async (friend: NewFriend) => {
	const res = await fetch('http://localhost:8080/friends', {
		method: 'POST',
		body: JSON.stringify(friend),
		headers: {
			'Content-Type': 'application/json'
		}
	});
	const data = await res.json();

	return data;
};
