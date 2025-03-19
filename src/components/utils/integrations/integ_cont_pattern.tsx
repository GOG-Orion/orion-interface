// integrations_pattern.tsx
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { confirm, message } from "@tauri-apps/api/dialog";

interface IntegrationsContainerPatternProps {
  name: string;
  integrationImage: string;
  downloadUrl: string;
}

export function IntegrationsContainerPattern({
  name,
  integrationImage,
  downloadUrl,
}: IntegrationsContainerPatternProps) {
  const [extensionName] = useState(name);
  // Assume the installed version is managed internally via a file,
  // but the latest available version is obtained here (for example, "v1.0.7")
  const [latestVersion] = useState("v1.0.7");
  const [isVerifying, setIsVerifying] = useState(false);
  const [buttonText, setButtonText] = useState("Verify");
  // Estado para saber se há atualização disponível
  const [updateAvailable, setUpdateAvailable] = useState(false);
  const [disableButton, setDisableButton] = useState(false);
 

  // Função para verificar a versão
  const handleVerifyClick = async () => {
    const confirmation = await confirm(
      "Do you want to start verification?",
      "Verification Started"
    );
    if (confirmation) {
      setIsVerifying(true);
      setButtonText("Verifying...");
      try {
        const update = await invoke<boolean>("verify_latest_version", {
          integrationName: extensionName,
          latestVersion: latestVersion,
        });
        if (update) {
          setUpdateAvailable(true);
          setButtonText("Download & Install");
          await message("New version available! Click the button to download and install.", {
            title: "Info",
          });
        } else {
          setUpdateAvailable(false);
          await message("Latest version installed.", {
            title: "Info",
            type: "error",
          });
          setDisableButton(true);
          setTimeout(() => {
            setDisableButton(false);
          }, 60000);
          setButtonText("Verify"); // Mantém como Verify, já que não há atualização
        }
      } catch (error) {
        console.error("Verification failed:", error);
        await message("Verification failed: " + String(error), {
          title: "Error",
          type: "error",
        });
        setButtonText("Verify");
      } finally {
        setIsVerifying(false);
      }
    }
  };

  // Função para download e instalação
  const handleDownloadAndInstall = async () => {
    try {
      const result = await invoke("download_file", {
        integrationName: extensionName,
      });
      console.log(result);
      await message("Download, installation, and cleanup completed successfully!", {
        title: "Success",
      });
      // Após a instalação, reseta o estado
      setUpdateAvailable(false);
      setButtonText("Verify");
    } catch (error) {
      console.error("Download and install failed:", error);
      await message("Download and install failed: " + String(error), {
        title: "Error",
        type: "error",
      });
    }
  };

  // Ao clicar, decide qual ação executar com base no estado
  const handleButtonClick = async () => {
    if (updateAvailable) {
      await handleDownloadAndInstall();
    } else {
      await handleVerifyClick();
    }
  };

  const integrationImagePath = `../src/assets/${integrationImage}Logo.svg`;

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
        <button onClick={handleButtonClick} disabled={isVerifying || disableButton}>
          {buttonText}
        </button>
      </span>
    </div>
  );
}

export default IntegrationsContainerPattern;