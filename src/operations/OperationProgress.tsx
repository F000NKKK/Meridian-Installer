import { Button, ProgressBar, useT } from "substrate-platform-ui";
import { formatBytes } from "../format";
import type { Operation } from "./useOperation";

/** A running operation inside its product's card, as Visual Studio Installer shows it. */
export function OperationProgress({ op, onPause, onCancel, onDismiss }: { op: Operation; onPause: (paused: boolean) => void; onCancel: () => void; onDismiss: () => void }) {
  const t = useT();
  if (op.finished) {
    if (op.finished.ok) return null;
    return (
      <div className="mi-op mi-op--ended">
        <span className={op.finished.cancelled ? "mi-op-note" : "mi-op-error"}>
          {op.finished.cancelled ? t("Cancelled.") : t("Installation failed: {error}", { error: op.finished.error ?? "" })}
        </span>
        <Button variant="ghost" onClick={onDismiss}>
          {t("Dismiss")}
        </Button>
      </div>
    );
  }
  const downloadFraction = op.bytes > 0 ? op.downloaded / op.bytes : null;
  const installFraction = op.installing ? (op.installing.index - 1) / op.installing.of : 0;
  return (
    <div className="mi-op">
      <div className="mi-op-row">
        <span>
          {t("Downloading and verifying: {done} of {total}", { done: formatBytes(op.downloaded), total: formatBytes(op.bytes) })}
          {op.speed > 0 && ` (${t("{speed}/sec", { speed: formatBytes(op.speed) })})`}
        </span>
        <span>{downloadFraction !== null ? `${Math.round(downloadFraction * 100)}%` : ""}</span>
      </div>
      <ProgressBar value={downloadFraction} label={t("Downloading")} />
      <div className="mi-op-row">
        <span>{op.installing ? t("Installing: package {index} of {of}", { index: op.installing.index, of: op.installing.of }) : t("Waiting to install")}</span>
        <span>{`${Math.round(installFraction * 100)}%`}</span>
      </div>
      <ProgressBar value={op.installing ? installFraction : 0} label={t("Installing")} />
      <div className="mi-op-detail">{op.lastLine ?? op.installing?.component ?? ""}</div>
      <div className="mi-op-buttons">
        <Button onClick={() => onPause(!op.paused)}>{op.paused ? t("Resume") : t("Pause")}</Button>
        <Button onClick={onCancel}>{t("Cancel")}</Button>
      </div>
    </div>
  );
}
