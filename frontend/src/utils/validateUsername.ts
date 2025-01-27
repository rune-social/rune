export default function validateUsername(username: string): boolean {
  const re = new RegExp(/^\w{1,20}$/);
  return re.test(username);
}
