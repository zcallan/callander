import type { FriendIdeaTypeEnum } from './generated';

export const FriendIdeaType: { [key in FriendIdeaTypeEnum]: FriendIdeaTypeEnum } = {
  place: 'place',
  gift: 'gift',
  activity: 'activity',
  other: 'other',
  conversation: 'conversation',
};
