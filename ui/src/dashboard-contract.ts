import type { InsightCommit, InsightLimits, InsightPayload, InsightSignal, InsightStage } from './insight-engine';

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function requireRecord(value: unknown, label: string): Record<string, unknown> {
  if (!isRecord(value)) {
    throw new Error(`${label} must be an object.`);
  }
  return value;
}

function requireString(value: unknown, label: string): string {
  if (typeof value !== 'string' || value.trim() === '') {
    throw new Error(`${label} must be a non-empty string.`);
  }
  return value;
}

function requireFiniteNonNegative(value: unknown, label: string): number {
  if (typeof value !== 'number' || !Number.isFinite(value) || value < 0) {
    throw new Error(`${label} must be a finite non-negative number.`);
  }
  return value;
}

function requireBoolean(value: unknown, label: string): boolean {
  if (typeof value !== 'boolean') {
    throw new Error(`${label} must be a boolean.`);
  }
  return value;
}

function validateRows<T>(
  value: unknown,
  collection: string,
  validate: (row: Record<string, unknown>, label: string) => T
): T[] | undefined {
  if (value === undefined) {
    return undefined;
  }
  if (!Array.isArray(value)) {
    throw new Error(`${collection} must be an array.`);
  }
  return value.map((candidate, index) => validate(requireRecord(candidate, `${collection}[${index}]`), `${collection}[${index}]`));
}

function validateCommit(row: Record<string, unknown>, label: string): InsightCommit {
  return {
    id: requireString(row.id, `${label}.id`),
    files: requireFiniteNonNegative(row.files, `${label}.files`),
    changedLines: requireFiniteNonNegative(row.changedLines, `${label}.changedLines`),
    dependencyChanges: requireFiniteNonNegative(row.dependencyChanges, `${label}.dependencyChanges`),
    testTouch: requireBoolean(row.testTouch, `${label}.testTouch`),
    failedAutomations: requireFiniteNonNegative(row.failedAutomations, `${label}.failedAutomations`)
  };
}

function validateStage(row: Record<string, unknown>, label: string): InsightStage {
  return {
    name: requireString(row.name, `${label}.name`),
    queueDepth: requireFiniteNonNegative(row.queueDepth, `${label}.queueDepth`),
    throughput: requireFiniteNonNegative(row.throughput, `${label}.throughput`),
    avgLatencyMs: requireFiniteNonNegative(row.avgLatencyMs, `${label}.avgLatencyMs`)
  };
}

function validateSignal(row: Record<string, unknown>, label: string): InsightSignal {
  const confidence = row.confidence;
  if (typeof confidence !== 'number' || !Number.isFinite(confidence) || confidence < 0 || confidence > 1) {
    throw new Error(`${label}.confidence must be a finite number between 0 and 1.`);
  }
  return {
    id: requireString(row.id, `${label}.id`),
    area: requireString(row.area, `${label}.area`),
    title: requireString(row.title, `${label}.title`),
    impact: requireFiniteNonNegative(row.impact, `${label}.impact`),
    effort: requireFiniteNonNegative(row.effort, `${label}.effort`),
    confidence
  };
}

function toPositiveOptional(value: unknown): number | undefined {
  if (typeof value !== 'number' || Number.isNaN(value)) {
    return undefined;
  }
  return Math.max(1, Math.floor(value));
}

export function readLimits(payload: Record<string, unknown>): InsightLimits {
  const limitsSource = payload.limits;
  const candidate = limitsSource === undefined ? payload : requireRecord(limitsSource, 'limits');

  for (const key of ['risks', 'opportunities', 'severityThreshold', 'latencyP95Ms']) {
    const value = candidate[key];
    if (value !== undefined && (typeof value !== 'number' || !Number.isFinite(value))) {
      throw new Error(`limits.${key} must be a finite number.`);
    }
  }
  for (const key of ['risks', 'opportunities', 'latencyP95Ms']) {
    const value = candidate[key];
    if (typeof value === 'number' && value < 0) {
      throw new Error(`limits.${key} must not be negative.`);
    }
  }

  return {
    risks: toPositiveOptional(candidate.risks),
    opportunities: toPositiveOptional(candidate.opportunities),
    severityThreshold:
      typeof candidate.severityThreshold === 'number' && Number.isFinite(candidate.severityThreshold)
        ? candidate.severityThreshold
        : undefined,
    latencyP95Ms: toPositiveOptional(candidate.latencyP95Ms)
  };
}

export function readPayload(payload: Record<string, unknown>): Record<string, unknown> {
  const nestedPayload = payload.payload;
  if (nestedPayload !== undefined) {
    return requireRecord(nestedPayload, 'payload.payload');
  }
  return payload;
}

export function validatePayload(payload: Record<string, unknown>): InsightPayload {
  return {
    commits: validateRows(payload.commits, 'commits', validateCommit),
    stages: validateRows(payload.stages, 'stages', validateStage),
    signals: validateRows(payload.signals, 'signals', validateSignal)
  };
}

export function getPayloadState(payload: InsightPayload): 'missing' | 'empty' | 'imported' {
  const collections = [payload.commits, payload.stages, payload.signals];
  if (collections.every((collection) => collection === undefined)) {
    return 'missing';
  }
  if (collections.every((collection) => collection === undefined || collection.length === 0)) {
    return 'empty';
  }
  return 'imported';
}
