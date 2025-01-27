import { describe, expect, test } from "@jest/globals";
import validateUsername from "./validateUsername";

const validUsernames: string[] = [
  "1",
  "_",
  "a",
  "Z",
  "chocologic",
  "gorae",
  "bombwhale",
  "froyo",
  "chocologic2",
  "saki_hanami",
  "china_kuramoto22",
  "KotoneFujita",
];

const invalidUsernames: string[] = [
  "123456789012345678901",
  "ちょちょいのちょい",
  "후지타 코토네",
  "",
  "-",
  "김수한무",
];

describe("test validateUsername", () => {
  for (const username of validUsernames) {
    test(`${username} is valid`, () => {
      expect(validateUsername(username)).toStrictEqual(true);
    });
  }

  for (const username of invalidUsernames) {
    test(`${username} is invalid`, () => {
      expect(validateUsername(username)).toStrictEqual(false);
    });
  }
});
