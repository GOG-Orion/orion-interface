import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

type OrionConfig = {
  install_root: string;
};

function ConfigurationsTab() {
  const [installRoot, setInstallRoot] = useState("");
  const [statusMessage, setStatusMessage] = useState("Loading configuration...");
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    let isMounted = true;

    const loadConfiguration = async () => {
      try {
        const configuration = await invoke<OrionConfig>("get_configuration");
        if (!isMounted) {
          return;
        }

        setInstallRoot(configuration.install_root);
        setStatusMessage("Configuration loaded.");
      } catch (error) {
        if (!isMounted) {
          return;
        }

        setStatusMessage(`Failed to load configuration: ${String(error)}`);
      }
    };

    void loadConfiguration();

    return () => {
      isMounted = false;
    };
  }, []);

  const handleSave = async () => {
    setIsSaving(true);
    setStatusMessage("Saving configuration...");

    try {
      const configuration = await invoke<OrionConfig>("set_install_root", {
        installRoot,
      });

      setInstallRoot(configuration.install_root);
      setStatusMessage("Install path saved.");
    } catch (error) {
      setStatusMessage(`Failed to save configuration: ${String(error)}`);
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <section className="configuration-tab">
      <h2>Configuration</h2>
      <p>Set the install directory used by Orion when applying releases.</p>
      <label className="configuration-field">
        <span>Install root</span>
        <input
          type="text"
          value={installRoot}
          onChange={(event) => setInstallRoot(event.target.value)}
          placeholder="C:\\Games\\GOG Galaxy\\Plugins\\Installed"
        />
      </label>
      <div className="configuration-actions">
        <button onClick={handleSave} disabled={isSaving || installRoot.trim().length === 0}>
          {isSaving ? "Saving..." : "Save path"}
        </button>
      </div>
      <p className="configuration-status">{statusMessage}</p>
    </section>
  );
}

export default ConfigurationsTab;
