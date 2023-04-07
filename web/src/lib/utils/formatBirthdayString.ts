import dayjs from 'dayjs';

const formatBirthdayString = (date: string): string => {
  const currentYear = new Date().getFullYear();

  const formattedDate = dayjs(date).format('MMM D');
  const daysUntilBday = dayjs(date).set('year', currentYear).diff(dayjs(), 'days');

  return `${formattedDate} (${Math.abs(daysUntilBday)} days ${daysUntilBday > 0 ? 'away' : 'ago'})`;
};

export default formatBirthdayString;
