// integrations_pattern.tsx
import { invoke } from "@tauri-apps/api/tauri";
import { confirm, message } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import steamIcon from "../../../assets/steamLogo.svg";
import epicGamesIcon from "../../../assets/epicgamesLogo.svg";
import ubisoftIcon from "../../../assets/ubisoftLogo.svg";

type InstallReadiness = {
  client_running: boolean;
  client_names: string[];
  message: string;
};

interface IntegrationsContainerPatternProps {
  name: string;
  integrationImage: string;
}

export function IntegrationsContainerPattern({
  name,
  integrationImage,
}: IntegrationsContainerPatternProps) {
  const extensionName = name;
  const [updateAvailable, setUpdateAvailable] = useState(false);
  const [installReadiness, setInstallReadiness] = useState<InstallReadiness>({
    client_running: false,
    client_names: [],
    message: "Checking install readiness...",
  });
  const [isVerifying, setIsVerifying] = useState(false);
  const [isInstalling, setIsInstalling] = useState(false);
  const [statusText, setStatusText] = useState("Ready");

  const integrationIcons: Record<string, string> = {
    steam: steamIcon,
    epicgames: epicGamesIcon,
    ubisoft: ubisoftIcon,
  };

  const refreshInstallReadiness = async () => {
    const readiness = await invoke<InstallReadiness>("get_install_readiness");
    setInstallReadiness(readiness);
    return readiness;
  };

  useEffect(() => {
    void refreshInstallReadiness().catch((error) => {
      console.error("Failed to check install readiness:", error);
      setInstallReadiness({
        client_running: true,
        client_names: [],
        message: `Failed to check install readiness: ${String(error)}`,
      });
    });
  }, []);

  const handleVerifyClick = async () => {
    const confirmation = await confirm(
      "Do you want to start verification?",
      "Verification Started"
    );
    if (confirmation) {
      setIsVerifying(true);
      setStatusText("Verifying...");
      try {
        const status = await invoke<{
          integration_name: string;
          installed_version: string;
          latest_version: string;
          update_available: boolean;
          download_url?: string | null;
        }>("verify_latest_version", {
          integrationName: extensionName,
        });

        setUpdateAvailable(status.update_available);

        if (status.update_available) {
          setStatusText(
            `Update available: ${status.installed_version} → ${status.latest_version}`
          );
          await message("New version available. You can install it now.", {
            title: "Info",
          });
        } else {
          setStatusText(`Installed version is up to date: ${status.latest_version}`);
          await message("Latest version installed.", {
            title: "Info",
          });
        }
      } catch (error) {
        console.error("Verification failed:", error);
        setStatusText(`Verification failed: ${String(error)}`);
        await message("Verification failed: " + String(error), {
          title: "Error",
          type: "error",
        });
      } finally {
        setIsVerifying(false);
      }
    }
  };

  const handleInstallClick = async () => {
    const readiness = await refreshInstallReadiness();
    if (readiness.client_running) {
      setStatusText(readiness.message);
      await message(readiness.message, {
        title: "Info",
      });
      return;
    }

    setIsInstalling(true);
    setStatusText("Installing...");
    try {
      const result = await invoke<string>("download_file", {
        integrationName: extensionName,
      });

      setUpdateAvailable(false);
      setStatusText(result);
      await message(result, {
        title: "Info",
      });
    } catch (error) {
      console.error("Installation failed:", error);
      setStatusText(`Installation failed: ${String(error)}`);
      await message("Installation failed: " + String(error), {
        title: "Error",
        type: "error",
      });
    } finally {
      setIsInstalling(false);
    }
  };

  const integrationImagePath =
    integrationIcons[integrationImage.toLowerCase()] || steamIcon;

  return (
    <div className="integration-item">
      <span className="integration-item Props">
        <img
          className="integration-item Logo"
          src={integrationImagePath}
          alt={`${extensionName} logo`}
        />
        <span className="integration-item Name">{extensionName}</span>
      </span>
      <span className="integration-item Props">
        <button onClick={handleVerifyClick} disabled={isVerifying || isInstalling}>
          {isVerifying ? "Verifying..." : "Verify"}
        </button>
        <button
          onClick={handleInstallClick}
          disabled={isInstalling || isVerifying || !updateAvailable || installReadiness.client_running}
          title={
            installReadiness.client_running
              ? installReadiness.message
              : "Install the selected integration"
          }
        >
          {isInstalling ? "Installing..." : "Install"}
        </button>
      </span>
      <span className="integration-item Status">
        {installReadiness.client_running ? installReadiness.message : statusText}
      </span>
    </div>
  );
}

export default IntegrationsContainerPattern;
