import { WindowControls, useT } from "substrate-platform-ui";
import { Notifications } from "./home/Notifications";

/** The window's own title bar: its name, the notifications, the window buttons. */
export function TitleBar({ notifications }: { notifications: string[] }) {
  const t = useT();
  return (
    <header className="mi-titlebar" data-tauri-drag-region>
      <span className="mi-titlebar-name" data-tauri-drag-region>
        {t("Meridian Installer")}
      </span>
      <div className="mi-titlebar-actions">
        <Notifications messages={notifications} />
        <WindowControls />
      </div>
    </header>
  );
}
