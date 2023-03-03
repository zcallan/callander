function range(size: number, startAt = 0, descending?: boolean): number[] {
  return [...Array(size).keys()].map((i) => i + (descending ? -startAt : startAt));
}

export default range;
