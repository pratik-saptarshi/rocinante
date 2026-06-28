export interface AdminBridgeResult {
  ok: boolean;
  command: string;
  payload: string;
  message: string;
}

type CommitIngestionEvent = {
  commit_id: string;
  repo_name: string;
  release: string;
  committer: string;
  telemetry: Array<{
    plugin: string;
    metric_key: string;
    metric_value: number;
    details: string;
  }>;
};

export interface AdminIngestPayload {
  token: string;
  event: CommitIngestionEvent;
}

export interface AdminPromotePayload {
  token: string;
}

export interface AdminQueryPayload {
  token: string;
  name?: string;
  release?: string;
}

export interface AdminReleaseBaselineQueryPayload {
  token: string;
  repoName: string;
}

export interface AdminReleaseBaselineReseedPayload {
  token: string;
  repoName: string;
  baselineComplexity: number;
}

export interface AdminCommitterScoresPayload {
  token: string;
  name?: string;
  release?: string;
}

export interface AdminPrFileSignal {
  path: string;
  risk: number;
}

export interface AdminRankPrsPayload {
  token: string;
  prs: Array<{
    pr_id: string;
    repo_name: string;
    author: string;
    release: string;
    file_risk: number;
    author_velocity: number;
    approval_fidelity: number;
    files?: AdminPrFileSignal[];
    circuit_breaker_triggered?: boolean;
  }>;
}

export interface AdminUpdateWeightsPayload {
  token: string;
  weights: {
    version: string;
    complexity_weight: number;
    coverage_weight: number;
    churn_weight: number;
    pipeline_weight: number;
    pr_file_risk_weight: number;
    pr_velocity_weight: number;
    pr_approval_weight: number;
  };
}

export type AdminBridgeCommand =
  | 'ingest_event'
  | 'promote_lifecycle'
  | 'query_aggregates'
  | 'committer_scores'
  | 'rank_prs'
  | 'query_release_baseline'
  | 'reseed_release_baseline'
  | 'update_scoring_weights';

type AdminCommandArgs = {
  ingest_event: AdminIngestPayload;
  promote_lifecycle: AdminPromotePayload;
  query_aggregates: AdminQueryPayload;
  committer_scores: AdminCommitterScoresPayload;
  rank_prs: AdminRankPrsPayload;
  query_release_baseline: AdminReleaseBaselineQueryPayload;
  reseed_release_baseline: AdminReleaseBaselineReseedPayload;
  update_scoring_weights: AdminUpdateWeightsPayload;
};

type InvokeFn = (command: string, args?: unknown) => Promise<unknown>;
type InvokeOptions = {
  invoke?: InvokeFn;
  timeoutMs?: number;
};

let testInvoke: InvokeFn | null = null;

export function setAdminInvokeForTesting(invoke: InvokeFn | null) {
  testInvoke = invoke;
}

function resolveInvoke(override?: InvokeFn): InvokeFn | null {
  if (override) {
    return override;
  }

  if (testInvoke) {
    return testInvoke;
  }

  const tauriCore = (globalThis as { __TAURI__?: { core?: { invoke?: InvokeFn } } }).__TAURI__?.core;
  if (tauriCore?.invoke) {
    return tauriCore.invoke;
  }
  return null;
}

function withTimeout<T>(operation: Promise<T>, timeoutMs: number): Promise<T> {
  if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) {
    return operation;
  }

  return new Promise<T>((resolve, reject) => {
    const timeoutId = setTimeout(() => {
      reject(new Error(`Admin bridge timed out after ${timeoutMs}ms`));
    }, timeoutMs);

    operation.then(
      (value) => {
        clearTimeout(timeoutId);
        resolve(value);
      },
      (error) => {
        clearTimeout(timeoutId);
        reject(error);
      }
    );
  });
}

export async function invokeAdminCommand(
  command: AdminBridgeCommand,
  args: AdminCommandArgs[typeof command],
  options: InvokeOptions = {}
): Promise<AdminBridgeResult> {
  const invoke = resolveInvoke(options.invoke);
  if (!invoke) {
    return {
      ok: false,
      command,
      payload: JSON.stringify(args),
      message: 'Tauri runtime not detected. Command bridge is available in desktop runtime only.'
    };
  }

  try {
    const response =
      typeof options.timeoutMs === 'number' ? await withTimeout(invoke(command, args), options.timeoutMs) : await invoke(command, args);
    return {
      ok: true,
      command,
      payload: JSON.stringify(args),
      message: typeof response === 'string' ? response : JSON.stringify(response)
    };
  } catch (error) {
    return {
      ok: false,
      command,
      payload: JSON.stringify(args),
      message: error instanceof Error ? error.message : 'Command failed'
    };
  }
}
