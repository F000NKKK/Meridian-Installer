import { useEffect, useState } from "react";
import { Button, IconButton, Icon, Tab, useT } from "substrate-platform-ui";
import type { Mode, Product, Selection, Snapshot } from "../api";
import { formatBytes } from "../format";
import { ComponentsTab } from "./ComponentsTab";
import { DetailsPanel } from "./DetailsPanel";
import { LocationsTab } from "./LocationsTab";
import { WorkloadsTab } from "./WorkloadsTab";
import { useModify } from "./useModify";
import "./Modify.css";

type Page = "workloads" | "components" | "locations";

export interface ModifySettings {
  mode: Mode;
  keepDownloads: boolean;
}

/**
 * Installing or modifying a product, Visual Studio Installer's way:
 * Workloads, Individual components, Installation locations;
 * Installation details on the right; the location, the space it takes, the
 * download mode and the button at the bottom.
 */
export function ModifyWindow({ product, snapshot, settings, onSettings, onApply, onClose }: {
  product: Product;
  snapshot: Snapshot;
  settings: ModifySettings;
  onSettings: (s: ModifySettings) => void;
  onApply: (selection: Selection) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [page, setPage] = useState<Page>("workloads");
  const { selection, plan, error, toggleWorkload, setComponent } = useModify(product.id, settings.mode, settings.keepDownloads);
  const installing = product.installed === null;
  const workloads = snapshot.workloads.filter((w) => product.workloads.includes(w.id));
  const chosen = workloads.filter((w) => selection?.workloads.includes(w.id));
  const reasons = plan?.reasons ?? {};
  const nothingToDo = plan !== null && plan.changes.length === 0;
  const blocked = !selection || !plan || plan.conflicts.length > 0;

  // Escape leaves, as the close button does.
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && onClose();
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onClose]);

  const tabs: [Page, string][] = [
    ["workloads", t("Workloads")],
    ["components", t("Individual components")],
    ["locations", t("Installation locations")],
  ];

  return (
    <div className="mi-modify" role="dialog" aria-label={product.name}>
      <header className="mi-modify-header" data-tauri-drag-region>
        <span data-tauri-drag-region>
          {installing ? t("Installing") : t("Modifying")} — {product.name} — {product.installed ?? product.latest}
        </span>
        <IconButton title={t("Close")} onClick={onClose}>
          <Icon name="close" />
        </IconButton>
      </header>
      <nav className="mi-modify-tabs">
        {tabs.map(([id, label]) => (
          <Tab key={id} active={page === id} onClick={() => setPage(id)}>
            {label}
          </Tab>
        ))}
      </nav>
      <div className="mi-modify-body">
        <main className="mi-modify-page">
          {error && <p className="mi-op-error">{error}</p>}
          {selection && page === "workloads" && <WorkloadsTab workloads={workloads} chosen={selection.workloads} onToggle={toggleWorkload} />}
          {selection && page === "components" && <ComponentsTab components={snapshot.components} groups={snapshot.groups} reasons={reasons} onSet={setComponent} />}
          {page === "locations" && (
            <LocationsTab home={snapshot.home} keepDownloads={settings.keepDownloads} onKeepDownloads={(keepDownloads) => onSettings({ ...settings, keepDownloads })} />
          )}
        </main>
        <DetailsPanel workloads={chosen} components={snapshot.components} reasons={reasons} onSet={setComponent} />
      </div>
      {plan && plan.conflicts.length > 0 && (
        <div className="mi-conflicts">
          {plan.conflicts.map((c) => (
            <div key={`${c.component}-${c.needs}`}>
              {t("{component} {version} needs {needs} {requirement} — {found}.", {
                component: c.component,
                version: c.version,
                needs: c.needs,
                requirement: c.requirement,
                found: c.found ? t("there will be {version}", { version: c.found }) : t("it won't be installed"),
              })}
            </div>
          ))}
        </div>
      )}
      <footer className="mi-modify-footer">
        <div className="mi-modify-location">
          <span className="mi-muted">{t("Location")}</span>
          <code className="mi-path">{snapshot.home}</code>
        </div>
        <div className="mi-modify-total">
          <span className="mi-muted">{t("Total download")}</span>
          <strong>{plan ? formatBytes(plan.downloadBytes) : "…"}</strong>
        </div>
        <div className="mi-modify-actions">
          <select className="mi-mode" value={settings.mode} onChange={(e) => onSettings({ ...settings, mode: e.target.value as Mode })}>
            <option value="installWhileDownloading">{t("Install while downloading")}</option>
            <option value="downloadAllThenInstall">{t("Download all, then install")}</option>
          </select>
          {nothingToDo ? (
            // Nothing chosen differs from what's there: the button only leaves.
            <Button variant="primary" className="mi-modify-go" onClick={onClose}>
              {t("Close")}
            </Button>
          ) : (
            <Button variant="primary" className="mi-modify-go" disabled={blocked} onClick={() => selection && onApply(selection)}>
              {installing ? t("Install") : t("Modify")}
            </Button>
          )}
        </div>
      </footer>
    </div>
  );
}
