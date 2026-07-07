#!/usr/bin/env node
import fs from "node:fs";

const LOCK_FILE = process.argv[2] ?? "ui/pnpm-lock.yaml";
const MIN_MAJOR = 0;
const MIN_MINOR = 28;
const MIN_PATCH = 1;

function parseSemver(value) {
  const match = value.match(/^(\d+)\.(\d+)\.(\d+)/);
  if (!match) {
    throw new Error(`Invalid semver value: ${value}`);
  }
  return {
    major: Number(match[1]),
    minor: Number(match[2]),
    patch: Number(match[3]),
  };
}

function compareVersion(a, b) {
  if (a.major !== b.major) return Math.sign(a.major - b.major);
  if (a.minor !== b.minor) return Math.sign(a.minor - b.minor);
  return Math.sign(a.patch - b.patch);
}

const lockText = fs.readFileSync(LOCK_FILE, "utf8");
const matches = [...lockText.matchAll(/^\s*esbuild@(npm:)?(\d+\.\d+\.\d+):/gm)];
if (!matches.length) {
  throw new Error("Could not find esbuild entry in pnpm lockfile");
}

const versions = matches.map((entry) => parseSemver(entry[2]));
const locked = versions.reduce((max, candidate) => {
  if (compareVersion(candidate, max) > 0) {
    return candidate;
  }

  return max;
}, versions[0]);

const minimum = { major: MIN_MAJOR, minor: MIN_MINOR, patch: MIN_PATCH };
if (compareVersion(locked, minimum) < 0) {
  throw new Error(
    `esbuild lockfile floor violation: expected >= ${MIN_MAJOR}.${MIN_MINOR}.${MIN_PATCH}, found ${locked.major}.${locked.minor}.${locked.patch}`,
  );
}

console.log(`esbuild lockfile floor check passed: ${locked.major}.${locked.minor}.${locked.patch} >= ${MIN_MAJOR}.${MIN_MINOR}.${MIN_PATCH}`);
