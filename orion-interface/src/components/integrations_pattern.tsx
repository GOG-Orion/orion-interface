import { useState } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import { confirm, message } from "@tauri-apps/api/dialog";

interface IntegrationsPatternProps {
    isEnabled: boolean;
    name: string;
    integrationImage: string;
    currentVersion: string; // This can be an empty string or undefined
    latestVersion: string;  // This can be an empty string or undefined
    updateSelectedCount: (delta: number) => void;
}

export function IntegrationsPattern({
    isEnabled,
    updateSelectedCount,
    name,
    integrationImage,
    currentVersion,
    latestVersion 
}: IntegrationsPatternProps) {
    const [extensionName] = useState(name);
    const [isChecked, setIsChecked] = useState(false);
    const [isVerifying, setIsVerifying] = useState(false); // Verification state
    const [buttonText, setButtonText] = useState("Verify"); // Button text state

    const handleCheckboxChange = () => {
        setIsChecked(!isChecked);
        updateSelectedCount(isChecked ? -1 : 1);
    };

    const handleVerifyClick = async () => {
        console.log("Verification button clicked"); // Debugging log
        const confirmation = await confirm("Do you want to start verification?", "Verification Started");
        console.log("Confirmation response:", confirmation); // Debugging log
        if (confirmation) {
            setIsVerifying(true); // Disables button
            setButtonText("Verifying..."); // Change button text

            try {
                const result = await invoke("download_and_install", { integration_name: extensionName });
                console.log(result);

                // Check if there's a new version
                if (currentVersion !== latestVersion) {
                    setButtonText("Download"); // If there's a new version, change button text
                    await message("New version available! Click 'Download' to install.", { title: "Info" });
                } else {
                    await message("Latest version installed.", { title: "Info", type: "error" });
                }
            } catch (error) {
                console.error("Verification failed:", error);
                await message("Verification failed: " + error.message, { title: "Error", type: "error" });
            } finally {
                setIsVerifying(false); // Enables button
                setButtonText("Verify"); // Reset button text after verification
            }
        }
    };

    // Generate image path based on integrationImage prop
    const integrationImagePath = `../src/assets/${integrationImage}Logo.svg`;

    // Helper function to determine the version display
    const formatVersion = (version: string) => {
        return version && version.trim() !== "" ? version : "-"; // Show '-' if version is empty or undefined
    };

    return (
        <div className="integration-item">
            <span className="integration-item Props">
                <input type="checkbox" checked={isChecked} onChange={handleCheckboxChange} />
                <img className="integration-item Logo" src={integrationImagePath} alt="Extension logo" />
                <span className="integration-item Name">{extensionName}</span>
            </span>
            <span className="integration-item Props">
                <span className="integration-item Ver">{formatVersion(latestVersion)}</span> {/* Format version display */}
                <button onClick={handleVerifyClick} disabled={isVerifying || !isEnabled}>
                    {buttonText}
                </button>
            </span>
        </div>
    );
}

export default IntegrationsPattern;
