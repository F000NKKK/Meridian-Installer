import { CheckField, useT } from "substrate-platform-ui";

/** Where things go: the Installer's home (moved with MERIDIAN_HOME), and whether downloads are kept. */
export function LocationsTab({ home, keepDownloads, onKeepDownloads }: { home: string; keepDownloads: boolean; onKeepDownloads: (keep: boolean) => void }) {
  const t = useT();
  return (
    <div className="mi-locations">
      <section>
        <h3>{t("Meridian home")}</h3>
        <p className="mi-muted">{t("Products, the SDK, tools and the download cache go here. Set the MERIDIAN_HOME variable to move it.")}</p>
        <code className="mi-path">{home}</code>
        <CheckField checked={keepDownloads} onChange={onKeepDownloads} label={t("Keep download cache after the installation")} />
      </section>
      <section>
        <h3>{t("Shared components, tools and SDKs")}</h3>
        <p className="mi-muted">{t("The Rust toolchain, its targets and cargo tools go where rustup and cargo keep them (~/.cargo).")}</p>
      </section>
    </div>
  );
}
