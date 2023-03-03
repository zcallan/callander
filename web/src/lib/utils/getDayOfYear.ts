function getDayOfYear(date: Date): number {
  const today = Date.UTC(date.getFullYear(), date.getMonth(), date.getDate());
  const startOfYear = Date.UTC(date.getFullYear(), 0, 0);
  const oneDay = 1000 * 60 * 60 * 24;
  return Math.floor((today - startOfYear) / oneDay);
}

export default getDayOfYear;
