import { Button, ProgressBar, useT } from "substrate-platform-ui";
import { formatBytes } from "../format";
import type { Operation } from "./useOperation";

/**
 * "The Installer needs to be updated", as Visual Studio Installer asks it
 * before anything else: Update downloads the new version here, with its
 * progress (the update is an operation like any other), and the Installer
 * starts again by itself. Not now leaves it for the banner.
 */
export function SelfUpdateDialog({ current, next, op, onUpdate, onLater }: { current: string; next: string; op: Operation | null; onUpdate: () => void; onLater: () => void }) {
  const t = useT();
  const updating = op !== null && op.finished === null;
  const fraction = op && op.bytes > 0 ? op.downloaded / op.bytes : null;
  const done = op !== null && op.finished?.ok === true;
  return (
    <div className="sp-dialog-backdrop">
      <div className="sp-dialog mi-self-update-dialog" role="dialog" aria-modal="true" aria-label={t("Meridian Installer needs to be updated")}>
        <div className="sp-dialog-title">{t("Meridian Installer needs to be updated")}</div>
        <div className="sp-dialog-message">
          {t("Before you go on, Meridian Installer updates itself from {current} to {next}. It starts again by itself when it's done.", { current, next })}
        </div>
        {(updating || done) && (
          <div className="mi-self-update-progress">
            <div className="mi-op-row">
              <span>
                {done
                  ? t("Starting the new version…")
                  : op && op.bytes > 0
                    ? t("Downloading: {done} of {total}", { done: formatBytes(op.downloaded), total: formatBytes(op.bytes) })
                    : t("Updating…")}
              </span>
              <span>{fraction !== null && !done ? `${Math.round(fraction * 100)}%` : ""}</span>
            </div>
            <ProgressBar value={done ? null : fraction} label={t("Downloading")} />
          </div>
        )}
        <div className="sp-dialog-buttons">
          <Button variant="primary" disabled={updating || done} onClick={onUpdate}>
            {t("Update")}
          </Button>
          <Button disabled={updating || done} onClick={onLater}>
            {t("Not now")}
          </Button>
        </div>
      </div>
    </div>
  );
}
