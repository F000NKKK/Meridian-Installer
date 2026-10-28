import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invokeCommand } from "substrate-platform-ui";
import { events, installer } from "./host.generated";

/** `display::ProductView` — a product card. */
export interface Product {
  id: string;
  name: string;
  description: string;
  installed: string | null;
  latest: string | null;
  updateAvailable: boolean;
  notesUrl: string | null;
  program: string;
  size: number | null;
  /** Its icon's URL. */
  icon: string | null;
  workloads: string[];
}

/** `display::ComponentView` — a component's line. */
export interface Component {
  id: string;
  name: string;
  description: string;
  group: string;
  installed: string | null;
  managed: boolean;
  latest: string | null;
  size: number | null;
}

export interface Group {
  id: string;
  name: string;
}

export interface Workload {
  id: string;
  name: string;
  description: string;
  icon: string | null;
  required: string[];
  recommended: string[];
  optional: string[];
}

/** `display::Snapshot` — everything the window shows. */
export interface Snapshot {
  version: string;
  home: string;
  products: Product[];
  components: Component[];
  groups: Group[];
  workloads: Workload[];
  installerId: string | null;
  installerUpdate: string | null;
  problems: string[];
}

/** `display::SelectionRequest` — the Modify window's checkboxes. */
export interface Selection {
  workloads: string[];
  unchecked: string[];
  checked: string[];
}

export type Change =
  | { kind: "install"; id: string; version: string }
  | { kind: "update"; id: string; from: string; to: string }
  | { kind: "remove"; id: string; version: string };

export interface Conflict {
  component: string;
  version: string;
  needs: string;
  requirement: string;
  found: string | null;
}

/** `display::WhyView` — why a component is in a selection. */
export type Why =
  | { kind: "required"; workload: string }
  | { kind: "recommended"; workload: string }
  | { kind: "chosen" }
  | { kind: "requiredBy"; component: string };

export interface PlanView {
  changes: Change[];
  reasons: Record<string, Why>;
  downloadBytes: number;
  conflicts: Conflict[];
  unread: string[];
}

/** `activity::ModeView`. */
export type Mode = "installWhileDownloading" | "downloadAllThenInstall";

/** `activity::ApplyRequest` — what the Modify window asks for. */
export interface ApplyRequest {
  product: string;
  selection: Selection;
  mode: Mode;
  keepDownloads: boolean;
}

/** `display::EventView`. */
export type ProgressEvent =
  | { kind: "started"; packages: number; bytes: number }
  | { kind: "downloading"; component: string; done: number; total: number | null }
  | { kind: "installing"; component: string; index: number; of: number }
  | { kind: "output"; component: string; line: string }
  | { kind: "installed"; component: string; version: string }
  | { kind: "removed"; component: string }
  | { kind: "finished"; ok: boolean; cancelled: boolean; error: string | null };

export const api = {
  snapshot: (refresh: boolean) => invokeCommand<Snapshot>(installer.installer.snapshot, { refresh }),
  selectionOf: (product: string) => invokeCommand<Selection>(installer.installer.selectionOf, { product }),
  plan: (request: ApplyRequest) => invokeCommand<PlanView>(installer.installer.plan, { request }),
  apply: (request: ApplyRequest) => invokeCommand<void>(installer.installer.apply, { request }),
  uninstall: (product: string) => invokeCommand<void>(installer.installer.uninstall, { product }),
  cancel: () => invokeCommand<void>(installer.installer.cancel),
  pause: (paused: boolean) => invokeCommand<void>(installer.installer.pause, { paused }),
  launch: (product: string) => invokeCommand<void>(installer.installer.launch, { product }),
  selfUpdate: () => invokeCommand<void>(installer.installer.selfUpdate),
  onProgress: (handler: (product: string, event: ProgressEvent) => void): Promise<UnlistenFn> =>
    listen<{ product: string; event: ProgressEvent }>(events.progress, (e) => handler(e.payload.product, e.payload.event)),
};
