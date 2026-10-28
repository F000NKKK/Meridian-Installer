import { Button, ProgressBar, useT } from "substrate-platform-ui";
import { formatBytes } from "../format";
import type { Operation } from "./useOperation";

/**
 * "The Installer needs to be updated", as Visual Studio Installer asks it
 * before anything else: Update downloads the new version here, with its
 * progress (the update is an operation like any other), and the Installer
 * starts again by itself. Not now leaves it for the banner.
 */
export function SelfUpdateDialog({ current, next, op, restartError, onRestart, onUpdate, onLater }: {
  current: string;
  next: string;
  op: Operation | null;
  restartError: string | null;
  onRestart: () => void;
  onUpdate: () => void;
  onLater: () => void;
}) {
  const t = useT();
  const updating = op !== null && op.finished === null;
  const fraction = op && op.bytes > 0 ? op.downloaded / op.bytes : null;
  const done = op !== null && op.finished?.ok === true;
  const waitingToStart = done && restartError === null;
  return (
    <div className="sp-dialog-backdrop">
      <div className="sp-dialog mi-self-update-dialog" role="dialog" aria-modal="true" aria-label={t("Meridian Installer needs to be updated")}>
        <div className="sp-dialog-title">{t("Meridian Installer needs to be updated")}</div>
        <div className="sp-dialog-message">
          {t("Before you go on, Meridian Installer updates itself from {current} to {next}. It starts again by itself when it's done.", { current, next })}
        </div>
        {op?.finished && !op.finished.ok && (
          <p className="mi-op-error" role="alert">
            {op.finished.cancelled ? t("Cancelled.") : t("Installation failed: {error}", { error: op.finished.error ?? "" })}
          </p>
        )}
        {restartError && <p className="mi-op-error" role="alert">{restartError}</p>}
        {(updating || waitingToStart) && (
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
          <Button variant="primary" disabled={updating || waitingToStart} onClick={done ? onRestart : onUpdate}>
            {done ? t("Launch") : t("Update")}
          </Button>
          <Button disabled={updating || waitingToStart} onClick={onLater}>
            {t("Not now")}
          </Button>
        </div>
      </div>
    </div>
  );
}
