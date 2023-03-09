/* https://stackoverflow.com/a/196991 */
function toSentenceCase(str: string, restLowerCase?: boolean) {
  return str.replace(/\w\S*/g, function (word) {
    const rest = restLowerCase ? word.substr(1).toLowerCase() : word.substr(1);
    return word.charAt(0).toUpperCase() + rest;
  });
}

export default toSentenceCase;
