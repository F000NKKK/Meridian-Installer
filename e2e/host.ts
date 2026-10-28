import type { Page } from '@playwright/test';

/** Browser tests exercise the real UI against a deterministic IPC boundary. */
export async function host(page: Page, options: { installed?: boolean; selfUpdate?: boolean; controlledPlans?: boolean } = {}) {
  await page.addInitScript((options) => {
    const w = window as any;
    const handlers = new Set<(product: string, event: any) => void>();
    const calls: { command: string; args?: any }[] = [];
    const plans: { request: any; resolve: (value: any) => void; reject: (error: string) => void }[] = [];
    const snapshot = {
      version: '0.1.0', home: '/home/test/.meridian', installerId: 'meridian.installer', installerUpdate: options.selfUpdate ? '0.2.0' : null,
      problems: ['Catalog unavailable. ' + 'Long diagnostic '.repeat(50), 'A second notification.'],
      products: [{ id: 'meridian.idle', name: 'Meridian IDLE', description: 'Game editor', installed: options.installed ? '0.1.0' : null, latest: '0.2.0', updateAvailable: !!options.installed, notesUrl: null, program: 'meridian-idle', size: 1024, icon: null, workloads: ['game'] }],
      components: [{ id: 'sdk', name: 'Engine SDK', description: 'Engine source', group: 'tools', installed: null, managed: false, latest: '0.5.0', size: 512 }],
      groups: [{ id: 'tools', name: 'Compilers and tools' }],
      workloads: [{ id: 'game', name: 'Game development', description: 'Game tools', icon: null, required: ['sdk'], recommended: [], optional: [] }],
    };
    const plan = { changes: [{ kind: 'install', id: 'sdk', version: '0.5.0' }], reasons: { sdk: { kind: 'required', workload: 'game' } }, downloadBytes: 1024, conflicts: [], unread: [] };
    const emit = (event: any, product = 'meridian.idle') => handlers.forEach(h => h(product, event));
    const api = {
      snapshot: async () => structuredClone(snapshot),
      selectionOf: async () => ({ workloads: ['game'], checked: [], unchecked: [] }),
      plan: async (request: any) => options.controlledPlans ? new Promise((resolve, reject) => plans.push({ request, resolve, reject })) : structuredClone(plan),
      apply: async (args: any) => { calls.push({ command: 'apply', args }); emit({ kind: 'started', packages: 1, bytes: 1024 }); },
      pause: async (paused: boolean) => { calls.push({ command: 'pause', args: paused }); },
      cancel: async () => { calls.push({ command: 'cancel' }); emit({ kind: 'finished', ok: false, cancelled: true, error: null }); },
      uninstall: async () => { calls.push({ command: 'uninstall' }); snapshot.products[0].installed = null; emit({ kind: 'started', packages: 0, bytes: 0 }); emit({ kind: 'finished', ok: true, cancelled: false, error: null }); },
      launch: async (product: string) => { calls.push({ command: 'launch', args: product }); },
      selfUpdate: async () => { calls.push({ command: 'selfUpdate' }); emit({ kind: 'started', packages: 1, bytes: 1024 }, 'meridian.installer'); },
      onProgress: async (handler: (product: string, event: any) => void) => { handlers.add(handler); return () => handlers.delete(handler); },
    };
    w.__installerTest = { snapshot, calls, plans, plan, api, emit };
    w.__TAURI_INTERNALS__ = {
      metadata: { currentWindow: { label: 'main' }, currentWebview: { label: 'main' } },
      invoke: async (command: string) => { calls.push({ command }); return false; },
      transformCallback: () => 1, unregisterCallback: () => {},
    };
    w.__TAURI_EVENT_PLUGIN_INTERNALS__ = { unregisterListener: () => {} };
  }, options);
  await page.route('**/src/api.ts', route => route.fulfill({ contentType: 'application/javascript', body: 'export const api = window.__installerTest.api;' }));
  await page.goto('/');
}
