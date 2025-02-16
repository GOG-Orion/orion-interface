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
  // currentVersion can be managed here (default is empty or "0.0.0")
  const [currentVersion] = useState("");
  const [isVerifying, setIsVerifying] = useState(false);
  const [buttonText, setButtonText] = useState("Verify");

  const handleVerifyClick = async () => {
    const confirmation = await confirm(
      "Do you want to start verification?",
      "Verification Started"
    );
    if (confirmation) {
      setIsVerifying(true);
      setButtonText("Verifying...");
      try {
        // Call the Tauri command 'verify_latest_version'
        const updateAvailable = await invoke<boolean>("verify_latest_version", {
          integration_name: extensionName,
          current_version: currentVersion,
        });

        if (updateAvailable) {
          setButtonText("Update!");
          await message("New version available! Click 'Download' to install.", {
            title: "Info",
          });
        } else {
          await message("Latest version installed.", {
            title: "Info",
            type: "error",
          });
        }
      } catch (error) {
        console.error("Verification failed:", error);
        await message("Verification failed: " + (error as Error).message, {
          title: "Error",
          type: "error",
        });
      } finally {
        setIsVerifying(false);
        // Reset button text after verification (or leave "Update!" if you want to allow direct updates)
        setButtonText("Verify");
      }
    }
  };

  // Build the logo image path using the integrationImage property
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
        <button onClick={handleVerifyClick} disabled={isVerifying}>
          {buttonText}
        </button>
      </span>
    </div>
  );
}

export default IntegrationsContainerPattern;
